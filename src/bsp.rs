#![allow(dead_code, unused_variables)]

use na;
use na::Dot;
use std;
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

#[derive(Copy, Debug, PartialEq)]
enum PlaneTestResult {
    Front,
    Back,
    Span(CastResult)
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct Plane {
    pub norm: na::Vec3<f32>,
    pub dist: f32
}
impl Plane {
    /// Returns a point that lies on this plane.
    pub fn point_on(&self) -> na::Pnt3<f32> {
        (self.norm * self.dist).to_pnt()
    }

    /// Tests a ray against this plane
    fn test_ray(&self, ray: &Ray) -> PlaneTestResult {
        let pad = na::abs(&(ray.halfextents.x * self.norm.x)) +
            na::abs(&(ray.halfextents.y * self.norm.y)) + 
            na::abs(&(ray.halfextents.z * self.norm.z));

        // Turn the ray into a line segment...
        let start = ray.orig.to_vec();
        let end = ray.orig.to_vec() + ray.dir;

        // Find the distance from each endpoint to the plane...
        let startdist = na::dot(&start, &self.norm) - self.dist;
        let enddist = na::dot(&end, &self.norm) - self.dist;

        // Are they both in front / back?
        if startdist >= pad && enddist >= pad {
            return PlaneTestResult::Front
        } else if startdist < -pad && enddist < -pad {
            return PlaneTestResult::Back;
        };

        // Apparently, the line segment spans the plane.
        let absstart = na::abs(&startdist);
        let totaldist = na::abs(&(startdist - enddist));
        let toi = if absstart <= pad || totaldist == 0.0 {
            0.0
        } else {
            (absstart - pad) / totaldist
        };

        PlaneTestResult::Span(
            CastResult {
                toi: toi,
                norm: self.norm
            }
            ) 
    }
}

pub type NodeIndex = i32;

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct InnerNode {
    plane: Plane,
    /// Subtree in the same direction as the normal.
    /// If this is negative, it's a leaf!
    pos: NodeIndex,
    /// Subtree against the normal.
    /// If this is negative, it's a leaf!
    neg: NodeIndex,
}

#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Leaf {
        solid: bool,
}

impl Leaf {
    fn is_solid(&self) -> bool {
        self.solid
    }
}

pub trait PlaneCollisionVisitor {
    fn visit_plane(&mut self, plane: &Plane, castresult: &CastResult);
}

#[derive(RustcDecodable, RustcEncodable,Debug)]
pub struct Tree {
    pub inodes: Vec<InnerNode>,
    pub leaves: Vec<Leaf>,
    pub root: NodeIndex
}
impl Tree {
    /// Is this point solid?
    pub fn contains_point(&self, point: &na::Pnt3<f32>) -> bool {
        self.contains_point_recursive(point, self.root)
    }

    fn contains_point_recursive(&self, point: &na::Pnt3<f32>, nodeidx: NodeIndex) -> bool {
        let InnerNode { ref plane, pos, neg } = self.nodes[nodeidx as usize];
        
        let dir = *point - plane.point_on(); 
        if na::dot(&dir, &plane.norm) > 0.0 {
            if pos < 0 {
                self.leaves[(-pos - 1) as usize].solid
            } else {
                self.contains_point_recursive(point, pos)
            }
        } else {
            if neg < 0 {
                self.leaves[(-neg - 1) as usize].solid
            } else {
                self.contains_point_recursive(point, neg)
            }
        }
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<CastResult> {
        unimplemented!();
        //self.cast_ray_recursive(ray, self.root, CastResult { toi: std::f32::INFINITY, norm: na::zero() })
    }

    fn cast_ray_recursive<V>(&self, ray: &Ray, nodeidx: NodeIndex, visitor: &mut V) -> bool
    where V: PlaneCollisionVisitor {
        let InnerNode { ref plane, pos, neg } = self.nodes[nodeidx as usize];

        let pltest = plane.test_ray(ray);

        // Is this plane hit sooner than the previous best?
        let firstimpact = match pltest {
            PlaneTestResult::Span(c) if c.toi < firstimpact.toi => c,
            _ => firstimpact 
        };

        // How does the ray interact with this plane?
        match pltest {
            // Does it lie entirely in front?
            PlaneTestResult::Front => {
                // Then just check the front subtree.
                if pos < 0 {
                    self.leaves[(-pos - 1) as usize].is_solid()
                } else {
                    self.cast_ray_recursive(&ray, pos, visitor) 
                }
            },
            // ... or perhaps it's entirely behind the plane? 
            PlaneTestResult::Back => {
                // Then just check the back subtree.
                if neg < 0 {
                    self.leaves[(-neg - 1) as usize].is_solid()
                } else {
                    self.cast_ray_recursive(&ray, neg, visitor) 
                }
            }
            // Or does it intersect the plane?
            PlaneTestResult::Span(CastResult{toi, ..}) => {
                // Then we must check both subtrees.
                // Split the ray into two rays, the part of each ray in each subtree.
                let (rpos, rneg) = ray.split(toi);

                // Ray::split is along the ray's direction, but we need it along the
                // plane's normal. If they don't coincide, swap the two sub-rays.
                let (rfirst, rlast) = if ray.dir.dot(&plane.norm) >= 0.0 {
                    (rpos, rneg)
                } else {
                    (rneg, rpos)
                };

                if self.cast_ray_recursive(&rfirst, pos, visitor)
                    || self.cast_ray_recursive(&rlast, neg, visitor) {
                        // terrible recursion hack
                        visitor.visit_plane(&self.plane, &plcast);
                        true
                    } else {
                        false
                    }
            },
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

    #[derive(Copy, Clone,Debug, PartialEq)]
    pub struct CastResult {
        /// Time of impact.
        pub toi: f32,
        /// Normal of what it hit, where it hit.
        pub norm: na::Vec3<f32>,
    }
}

#[cfg(test)]
pub mod test {
    use na;
    use super::{
        Node,
        Plane,
        Tree,
        PlaneTestResult

    };
    use super::cast::{
        Ray,
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
    fn plane_raytest() {
        let plane = Plane {
            norm: na::Vec3::new(1.0, 0.0, 0.0),
            dist: 0.0,
        };

        let result = plane.test_ray(&Ray {
            orig: na::Pnt3::new(-0.5, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::zero(),
        });

        match result {
            PlaneTestResult::Span(c) => {
                assert_approx_eq!(c.toi, 0.5);
                assert_approx_eq!(c.norm, plane.norm);
            },
            x => panic!("{:?}", x)
        };
    }

    #[test]
    fn plane_cubetest() {
        let plane = Plane {
            norm: na::Vec3::new(1.0, 0.0, 0.0),
            dist: 0.0,
        };

        let result = plane.test_ray(&Ray {
            orig: na::Pnt3::new(-1.0, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::Vec3::new(0.5, 0.0, 0.0),
        });

        match result {
            PlaneTestResult::Span(c) => {
                assert_approx_eq!(c.toi, 0.5);
                assert_approx_eq!(c.norm, plane.norm);
            },
            x => panic!("{:?}", x)
        };
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
            orig: na::Pnt3::new(-1.0, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::Vec3::new(0.5, 0.0, 0.0),
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
