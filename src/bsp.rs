#![allow(dead_code, unused_variables)]

use na;
use self::cast::{
    Ray,
    CastResult
};


const EPS: f32 = 1.0/32.0;

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
    pub leafbrush: i32,
    pub n_leafbrushes: i32,
}

#[derive(Debug)]
pub struct Brush {
    pub sides: Vec<BrushSide>
}
impl Brush {
    pub fn cast_ray(&self, ray: &Ray, (start, end): (f32, f32)) -> Option<CastResult> {
        let mut sf = -1.0;
        let mut ef = 1.0;
        let mut norm = na::zero();
        for side in &self.sides {
            if !(side.contents & 1 == 1) {
                debug!("Skipping non-solid brush side... contents {}", side.contents);
                continue;
            }

            let pad = na::abs(&(ray.halfextents.x * side.plane.norm.x)) +
                na::abs(&(ray.halfextents.y * side.plane.norm.y)) + 
                na::abs(&(ray.halfextents.z * side.plane.norm.z));

            let startpos = (ray.orig.to_vec()).to_pnt();
            let endpos = (ray.orig.to_vec() + ray.dir).to_pnt();

            let d1 = side.plane.dist_to_point(&startpos) - pad;
            let d2 = side.plane.dist_to_point(&endpos) - pad;
            if d1 > 0.0 && (d2 >= d1 || d2 >= EPS) { 
                return None;
            } else if d1 <= 0.0 && d2 <= 0.0 {
                continue;
            }
            if d1 > d2 {
                let frac = (d1 - EPS) / (d1 - d2);
                let frac = na::clamp(frac, 0.0, frac);
                if frac > sf {
                    sf = frac;
                    norm = side.plane.norm;
                }
            } else {
                let frac = (d1 + EPS) / (d1 - d2);
                let frac = na::clamp(frac, frac, 1.0);
                if frac < ef {
                    ef = frac;
                }
            }
        }
        if sf >= start && sf <= end {
            let toi = na::clamp(sf, 0.0, 1.0);
            return Some(CastResult {
                toi: toi,
                norm: norm
            });
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct BrushSide {
    pub plane: Plane,
    pub flags: i32,
    pub contents: i32,
}

fn combine_results(a: Option<CastResult>, b: Option<CastResult>) -> Option<CastResult> {
    if let Some(a) = a {
        match b {
            Some(b) => {
                if a.toi <= b.toi {
                    Some(a)
                } else {
                    Some(b)
                }
            },
            None => Some(a)
        }
    } else {
        b
    }
}

#[derive(Debug)]
pub struct Tree {
    pub inodes: Vec<InnerNode>,
    pub leaves: Vec<Leaf>,
    pub brushes: Vec<Brush>,
    pub leafbrushes: Vec<u32>,
}
impl Tree {
    /// Looks up a leaf by (negative) NodeIndex.
    fn get_leaf(&self, nodeidx: NodeIndex) -> &Leaf {
        &self.leaves[(-nodeidx - 1) as usize]
    }

    pub fn cast_ray(&self, ray: &Ray) -> Option<CastResult> {
        self.cast_ray_recursive(ray, 0, (0.0, 1.0), (ray.orig, (ray.orig.to_vec() + ray.dir).to_pnt()))
    }

    fn cast_ray_recursive(&self,
                          ray: &Ray,
                          nodeidx: NodeIndex,
                          (start, end): (f32, f32),
                          (startpos, endpos): (na::Pnt3<f32>, na::Pnt3<f32>))
                                               -> Option<CastResult> 
    {
        if start > end {
            return None;
        }
        if nodeidx < 0 {
            // check brushes for this leaf
            let leaf = self.get_leaf(nodeidx);

            let mut best = None;
            for &leafbrush in &self.leafbrushes[leaf.leafbrush as usize..(leaf.leafbrush + leaf.n_leafbrushes) as usize] {
                let brush = &self.brushes[leafbrush as usize];
                let result = brush.cast_ray(ray, (start, end)); 
                best = combine_results(result, best);
            }
            return best ;
        }

        let InnerNode { ref plane, pos, neg } = self.inodes[nodeidx as usize];
        
        let d1 = plane.dist_to_point(&startpos);
        let d2 = plane.dist_to_point(&endpos);

        let pad = na::abs(&(ray.halfextents.x * plane.norm.x)) +
            na::abs(&(ray.halfextents.y * plane.norm.y)) + 
            na::abs(&(ray.halfextents.z * plane.norm.z));


        // How does the ray interact with this plane?
        if d1 > (pad ) && d2 > (pad ) {
            // Then just check the front subtree.
            self.cast_ray_recursive(&ray, pos, (start, end), (startpos, endpos))
        } else if d1 < -(pad ) && d2 < -(pad ) {
            self.cast_ray_recursive(&ray, neg, (start, end), (startpos, endpos))
        } else {
            let td = d1 - d2;
            let coincident;
            let (ns, fs);
            if d1 < d2 {
                coincident = true;
                ns = (d1 - pad + EPS) / td;
                fs = (d1 + pad + EPS) / td;
            } else if d2 < d1 {
                coincident = false;
                ns = (d1 + pad - EPS) / td;
                fs = (d1 - pad - EPS) / td;
            } else {
                coincident = false;
                ns = 1.0;
                fs = 0.0;
            }
            
            let ns = na::clamp(ns, 0.0, 1.0);
            let fs = na::clamp(fs, 0.0, 1.0);

            let ns = start + (end - start) * ns;
            let fs = start + (end - start) * fs;

            let (near, far) = if coincident {
                (neg, pos) 
            } else {
                (pos, neg)
            };

            let (nearbounds, farbounds) =
                ((start, ns), (fs, end));

            let nmid = (startpos.to_vec() + ray.dir * ns).to_pnt();
            let fmid = (startpos.to_vec() + ray.dir * fs).to_pnt();

            combine_results(self.cast_ray_recursive(ray, near, nearbounds, (startpos, nmid)), self.cast_ray_recursive(ray, far, farbounds, (fmid, endpos)))
        }
    }
}

/// Loads a test BSP. The exact contents of this change with the
/// phase of the moon, but there is guaranteed to be a "floor" at z=0.
pub fn test_tree() -> Tree {
    use assets;
    let asset = assets::load_bin_asset("maps/test.bsp").unwrap();
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
