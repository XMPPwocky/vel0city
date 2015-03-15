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

#[derive(Copy, Debug, PartialEq)]
pub enum PlaneTestResult {
    Front,
    Back,
    Span(CastResult)
}

#[derive(RustcDecodable, RustcEncodable, Clone, Debug)]
pub struct Plane {
    pub norm: na::Vec3<f32>,
    pub dist: f32
}
impl Plane {
    /// Returns a point that lies on this plane.
    pub fn point_on(&self) -> na::Pnt3<f32> {
        (self.norm * self.dist).to_pnt()
    }

    fn dist_to_point(&self, point: &na::Pnt3<f32>) -> f32 {
        na::dot(&self.norm, point.as_vec()) - self.dist
    }
}

pub type NodeIndex = i32;

#[derive(Debug, Clone)]
pub struct InnerNode {
    pub plane: Plane,
    /// Subtree in the same direction as the normal.
    /// If this is negative, it's a leaf!
    pub pos: NodeIndex,
    /// Subtree against the normal.
    /// If this is negative, it's a leaf!
    pub neg: NodeIndex,
}

#[derive(Debug)]
pub struct Leaf {
    pub solid: bool,
}

impl Leaf {
    fn is_solid(&self) -> bool {
        self.solid
    }
}

pub trait PlaneCollisionVisitor {
    /// Visit a solid face.
    fn visit_plane(&mut self, plane: &Plane, castresult: &CastResult);
    
    /// Recurse down both sides of a split?
    fn should_visit_both(&self) -> bool {
        false
    }
}

struct JustFirstPlaneVisitor {
    /// The first plane hit (as in, least TOI).
    /// Initially None.
    best: Option<CastResult>
}
impl JustFirstPlaneVisitor {
    pub fn new() -> JustFirstPlaneVisitor {
        JustFirstPlaneVisitor {
            best: None
        }
    }
}
impl PlaneCollisionVisitor for JustFirstPlaneVisitor {
    fn visit_plane(&mut self, plane: &Plane, castresult: &CastResult) {
        if let Some(CastResult { toi: best_toi, .. }) = self.best {
            if castresult.toi <= best_toi {
                self.best = Some(*castresult);
            }
        } else {
            self.best = Some(*castresult);
        }
    }
}


#[derive(Debug)]
pub struct Tree {
    pub inodes: Vec<InnerNode>,
    pub leaves: Vec<Leaf>,
    pub root: NodeIndex
}
impl Tree {
    /// Looks up a leaf by (negative) NodeIndex.
    fn get_leaf(&self, nodeidx: NodeIndex) -> &Leaf {
        &self.leaves[(-nodeidx - 1) as usize]
    }

    /// Is this point solid?
    pub fn contains_point(&self, point: &na::Pnt3<f32>) -> bool {
        self.contains_point_recursive(point, self.root)
    }

    /// Ye Olde Binarye Searche
    fn contains_point_recursive(&self, point: &na::Pnt3<f32>, nodeidx: NodeIndex) -> bool {
        let InnerNode { ref plane, pos, neg } = self.inodes[nodeidx as usize];
        
        let dir = *point - plane.point_on(); 
        if na::dot(&dir, &plane.norm) > 0.0 {
            if pos < 0 {
                self.get_leaf(pos).is_solid() 
            } else {
                self.contains_point_recursive(point, pos)
            }
        } else {
            if neg < 0 {
                self.get_leaf(neg).is_solid() 
            } else {
                self.contains_point_recursive(point, neg)
            }
        }
    }

    /// Casts a ray against the tree, returning a CastResult if it hit anything.
    pub fn cast_ray(&self, ray: &Ray) -> Option<CastResult> {
        let mut visitor = JustFirstPlaneVisitor::new();
        self.cast_ray_recursive(ray, self.root, (0.0, 1.0), (ray.orig, (ray.orig.to_vec() + ray.dir).to_pnt()), &mut visitor);
        visitor.best
    }

    /// Casts a ray against the tree, using a custom `PlaneCollisionVisitor`.
    /// You probably do not need this, unless you need access to a full
    /// collision manifold.
    pub fn cast_ray_visitor<V>(&self, ray: &Ray, visitor: &mut V)
    where V: PlaneCollisionVisitor {
        self.cast_ray_recursive(ray, self.root, (0.0, 1.0), (ray.orig, (ray.orig.to_vec() + ray.dir).to_pnt()), visitor);
    }

    /// Takes a ray, bounded by [start, end) (from its origin to its direction)
    /// Tests that against the node nodeidx, and returns true if the ray hit
    /// the tree (starting at this node) within the bounds. If so, it also
    /// invokes the PlaneCollisionVisitor `visitor`.
    fn cast_ray_recursive<V>(&self, ray: &Ray, nodeidx: NodeIndex, (start, end): (f32, f32), (startpos, endpos): (na::Pnt3<f32>, na::Pnt3<f32>), visitor: &mut V) -> bool
    where V: PlaneCollisionVisitor {
        if nodeidx < 0 {
            return self.get_leaf(nodeidx).is_solid();
        }

        if start >= end {
            return false;
        }

        let InnerNode { ref plane, pos, neg } = self.inodes[nodeidx as usize];
        
        let d1 = plane.dist_to_point(&startpos);
        let d2 = plane.dist_to_point(&endpos);

        let pad = na::abs(&(ray.halfextents.x * plane.norm.x)) +
            na::abs(&(ray.halfextents.y * plane.norm.y)) + 
            na::abs(&(ray.halfextents.z * plane.norm.z));

        const EPS: f32 = 1.0/16.0;

        // How does the ray interact with this plane?
        if d1 >= (pad - EPS) && d2 >= (pad - EPS) {
            // Then just check the front subtree.
            self.cast_ray_recursive(&ray, pos, (start, end), (startpos, endpos), visitor) 
        } else if d1 <= -(pad - EPS) && d2 <= -(pad - EPS) { 
                self.cast_ray_recursive(&ray, neg, (start, end), (startpos, endpos), visitor) 
        } else if na::approx_eq(&d1, &d2) { 
            self.cast_ray_recursive(&ray, pos, (start, end), (startpos, endpos), visitor) 
        ||        self.cast_ray_recursive(&ray, neg, (start, end), (startpos, endpos), visitor) 
        } else {

            let td = d2 - d1;
            let (ns, fs);
            let coincident;
            if d1 < d2 {  
                coincident = true;
                ns = (d1 + EPS + pad) / td;
                fs = (d1 + EPS - pad) / td;
            } else if d1 > d2 {
                coincident = false;
                ns = (d1 - EPS - pad) / td;
                fs = (d1 - EPS + pad) / td;
            } else {
                unreachable!();
            };
            
            let ns = start + (end - start) * ns;
            let fs = start + (end - start) * fs;

            let ns = na::clamp(ns, 0.0, 1.0);
            let fs = na::clamp(fs, 0.0, 1.0);

            let (near, far) = if coincident {
                (neg, pos) 
            } else {
                (pos, neg)
            };

            let (nearbounds, farbounds) =
                ((start, ns), (fs, end));

            let nmid = (startpos.to_vec() + ray.dir * ns).to_pnt();
            let fmid = (startpos.to_vec() + ray.dir * fs).to_pnt();

            let mut hit = false;
            if self.cast_ray_recursive(ray, far, farbounds, (fmid, endpos), visitor) {
                visitor.visit_plane(&plane, &CastResult { pos: fmid, norm: plane.norm, toi: fs });
                hit = true;
            }
            if self.cast_ray_recursive(ray, near, nearbounds, (startpos, nmid), visitor) {
                //visitor.visit_plane(&plane, &CastResult { pos: nmid, norm: plane.norm, toi: ns });
                hit = true;
            }

            hit
        }
    }
}

/// Loads a test BSP. The exact contents of this change with the
/// phase of the moon, but there is guaranteed to be a "floor" at z=0.
pub fn test_tree() -> Tree {
    use assets;
    let asset = assets::load_bin_asset("test.bsp").unwrap();
    ::qbsp_import::import_collision(&asset).unwrap()
}

pub mod cast {
    use na;

    /// Secretly not a ray, it can have thickness to it.
    pub struct Ray {
        pub orig: na::Pnt3<f32>,
        pub dir: na::Vec3<f32>,
        pub halfextents: na::Vec3<f32>,
    }

    #[derive(Copy, Clone,Debug, PartialEq)]
    pub struct CastResult {
        pub pos: na::Pnt3<f32>,
        /// Time of impact.
        pub toi: f32,
        /// Normal of the plane it hit. 
        pub norm: na::Vec3<f32>,
    }
}

#[cfg(test)]
pub mod test {
    use na::{
        self,
        ApproxEq
    };
    use super::{
        test_tree,
        Plane,
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
            dist: 16.0,
        };

        let result = plane.test_ray(&Ray {
            orig: na::Pnt3::new(16.1, 0.0, 0.0),
            dir: na::Vec3::new(0.0, 0.0, 1.0),
            halfextents: na::Vec3::new(1.0, 1.0, 1.0),
        });

        match result {
            PlaneTestResult::Span(c) => {
                assert_approx_eq!(c.toi, 0.5);
                assert_approx_eq!(c.norm, plane.norm);
            },
            x => panic!("{:?}", x)
        };

        let result = plane.test_ray(&Ray {
            orig: na::Pnt3::new(0.1, 0.0, 0.0),
            dir: na::Vec3::new(1.0, 0.0, 0.0),
            halfextents: na::Vec3::new(0.5, 0.0, 0.0),
        });

        match result {
            PlaneTestResult::Span(c) => {
                assert_approx_eq!(c.toi, 0.0);
                assert_approx_eq!(c.norm, plane.norm);
            },
            x => panic!("{:?}", x)
        };
    }
}
