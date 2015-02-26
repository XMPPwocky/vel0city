#![allow(dead_code, unused_variables)]

use na;
use na::Dot;
use self::cast::{
    Ray,
    CastResult
};

fn signcpy(n: f32, from: f32) -> f32 {
    if from >= 0.0 {
        n
    } else {
        -n
    }
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Plane {
    pub norm: na::Vec3<f32>,
    pub dist: f32
}
impl Plane {
    pub fn point_on(&self) -> na::Pnt3<f32> {
        (self.norm * self.dist).to_pnt()
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<CastResult> {
        let offset = na::Vec3::new(signcpy(ray.halfextents.x, self.norm.x),
                                   signcpy(ray.halfextents.y, self.norm.y),
                                   signcpy(ray.halfextents.z, self.norm.z));
        let start = ray.orig.to_vec() + offset;
        let end = ray.orig.to_vec() + ray.dir + offset; 

        let startdist = na::dot(&start, &self.norm) - self.dist;
        let enddist = na::dot(&end, &self.norm) - self.dist;

        if (startdist >= 0.0 && enddist >= 0.0) || (startdist < 0.0 && enddist < 0.0) {
            return None;
        };


        let toi = if startdist >= 0.0 {
            startdist / (startdist - enddist)
        } else {
            enddist / (enddist - startdist)
        };

        if toi >= 0.0 { 
            Some(
                CastResult {
                    toi: toi,
                    norm: self.norm
                }
            ) 
        } else {
            None
        }
    }
}

pub type NodeIndex = usize;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub enum Node {
    Inner {
        plane: Plane,
        /// Subtree in the same direction as the normal
        pos: NodeIndex,
        /// Subtree against the normal
        neg: NodeIndex,
    },
    Leaf {
        solid: bool,
    }
}

#[derive(RustcDecodable, RustcEncodable,Debug)]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub root: NodeIndex
}
impl Tree {
    pub fn contains_point(&self, point: &na::Pnt3<f32>) -> bool {
        self.contains_point_recursive(point, self.root)
    }
    fn contains_point_recursive(&self, point: &na::Pnt3<f32>, nodeidx: NodeIndex) -> bool {
        match self.nodes[nodeidx] {
            Node::Inner { ref plane, pos, neg } => {
                let dir = *point - plane.point_on(); 
                if na::dot(&dir, &plane.norm) > 0.0 {
                    self.contains_point_recursive(point, pos)
                } else {
                    self.contains_point_recursive(point, neg)
                }
            }
            Node::Leaf { solid, .. } => solid,
        }
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<CastResult> {
        // This check is necessary because the recursive check is "edge-triggered".
        // In other words, it only considers each plane and the line, and does not check the starting point.
        if self.contains_point(&ray.orig) {
            None
        } else {
            self.cast_ray_recursive(ray, self.root, None)
        }
    }

    fn cast_ray_recursive(&self, ray: &Ray, nodeidx: NodeIndex, hack: Option<&CastResult>) -> Option<CastResult> {
        match self.nodes[nodeidx] {
            Node::Inner { ref plane, pos, neg, .. } => {
                let dir = ray.orig - plane.point_on();
                let (first, last) = if plane.norm.dot(&dir) > 0.0 {
                    (pos, neg)
                } else {
                    (neg, pos)
                };

                let plcast = plane.cast_ray(ray);
                let plcast = plcast.as_ref();

                let hack = plcast.or(hack);

                let toi = plcast.map(|cast| cast.toi).unwrap_or(1.0);
                if toi < 1.0 {
                    // we might need to go "through" the plane
                    // check both sides
                    let (rfirst, rlast) = ray.split(toi);
                    self.cast_ray_recursive(&rfirst, first, hack) 
                        .or_else(|| self.cast_ray_recursive(&rlast, last, hack))
                } else {
                    let (rfirst, _) = ray.split(toi);
                    // only need to check one subtree
                    self.cast_ray_recursive(&rfirst, first, hack)
                }
            }
            Node::Leaf { solid } => {
                if solid {
                    hack.map(|c| (*c).clone())
                } else {
                    None
                }

            }
        }
    }


}

pub mod cast {
    use na;
    
    pub struct Ray {
        pub orig: na::Pnt3<f32>,
        pub dir: na::Vec3<f32>,
        pub halfextents: na::Vec3<f32>,
    }
    impl Ray {
        pub fn split(&self, toi: f32) -> (Ray, Ray) {
            (
                Ray {
                    orig: self.orig,
                    dir: self.dir * toi,
                    halfextents: self.halfextents
                },
                Ray {
                    orig: (self.orig.to_vec() + (self.dir * toi)).to_pnt(),
                    dir: self.dir * (1.0 - toi),
                    halfextents: self.halfextents
                }
                )
        }
    }       

    #[derive(Clone,Debug)]
    pub struct CastResult {
        /// Time of impact.
        pub toi: f32,
        /// Normal of what it hit, where it hit.
        pub norm: na::Vec3<f32>,
    }
}

#[cfg(test)]
mod test {
    use na;
    use super::{
        Node,
        Plane,
        Tree
    };
    use super::cast::{
        Ray,
        CastResult
    };

    macro_rules! assert_castresult {
        ($e: expr, $toi: expr, $norm: expr) => {
            if let Some(ref c) = $e {
                if na::approx_eq(&c.toi, &$toi) {
                    ()
                } else {
                    panic!("Wrong TOI: Expected {:?}, got {:?}", $toi, c.toi);
                }

                if na::approx_eq(&c.norm, &$norm) {
                    ()
                } else {
                    panic!("Wrong normal: Expected {:?}, got {:?}", $norm, c.norm);
                }
            } else {
                panic!("Expected a hit, got a miss!")
            }
        }
    }


    fn test_tree() -> Tree {
        Tree {
            nodes: vec![
                Node::Inner {
                    plane: Plane {
                        norm: na::Vec3::new(1.0, 0.0, 0.0),
                        dist: 0.0,
                    },
                    pos: 2,
                    neg: 1,
                },
                Node::Leaf {
                    solid: false,
                },
                Node::Inner {
                    plane: Plane {
                        norm: na::Vec3::new(1.0, 0.0, 0.0),
                        dist: 1.0,
                    },
                    pos: 3,
                    neg: 4,
                },
                Node::Leaf {
                    solid: false,
                },
                Node::Inner {
                    plane: Plane {
                        norm: na::Vec3::new(0.0, 1.0, 0.0),
                        dist: 1.0,
                    },
                    pos: 5,
                    neg: 6,
                },
                Node::Leaf {
                    solid: false,
                },
                Node::Leaf {
                    solid: true,
                }
            ],
            root: 0
        }
    }

    #[test]
    fn plane_raycast() {
        let plane = Plane {
            norm: na::Vec3::new(1.0, 0.0, 0.0),
            dist: 0.0
        };

        //      ->   |
        let r1 = Ray {
            orig: na::Pnt3::new(-1.0, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::zero(),
        };
        assert!(plane.cast_ray(&r1).is_some());

        //      <-   |
        let r2 = Ray {
            orig: na::Pnt3::new(-1.0, 0.0, 0.0),
            dir: na::Vec3::new(-1.0, 0.0, 0.0),
            halfextents: na::zero(),
        };
        assert!(!plane.cast_ray(&r2).is_some());
    }

    #[test]
    fn bsp_raycast() {
        let tree = test_tree();

        let r1 = Ray {
            orig: na::Pnt3::new(-0.5, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::zero(),
        };
        assert_castresult!(tree.cast_ray(&r1), 0.5, na::Vec3::new(1.0, 0.0, 0.0));

        let r2 = Ray {
            orig: na::Pnt3::new(-0.5, 0.0, 0.0),
            dir: na::Vec3::new(-1.0, 0.0, 0.0),
            halfextents: na::zero(),
        };
        assert!(!tree.cast_ray(&r2).is_some());
    }

    #[test]
    fn bsp_cubecast() { 
        let tree = test_tree();

        let r1 = Ray {
            orig: na::Pnt3::new(-1.5, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::Vec3::new(1.0, 0.0, 0.0),
        };
        assert_castresult!(tree.cast_ray(&r1), 0.5, na::Vec3::new(1.0, 0.0, 0.0));

        /*let r2 = Ray {
            orig: na::Pnt3::new(-0.5, 0.0, 0.0),
            dir: na::Vec3::new(-1.0, 0.0, 0.0),
            halfextents: na::zero(),
        };
        assert!(!tree.cast_ray(&r2).is_some());*/
    }

    #[test]
    fn bsp_contains_point() { 
        let tree = test_tree();

        let p1 = na::Pnt3::new(0.5, 0.0, 0.0);
        let p2 = na::Pnt3::new(1.5, 0.0, 0.0);
        let p3 = na::Pnt3::new(0.5, 1.5, 0.0);
        assert!(tree.contains_point(&p1));
        assert!(!tree.contains_point(&p2));
        assert!(!tree.contains_point(&p3));
    }

}
