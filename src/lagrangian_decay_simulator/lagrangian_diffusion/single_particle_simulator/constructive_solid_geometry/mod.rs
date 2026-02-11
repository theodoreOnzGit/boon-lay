// the thing about constructive_solid_geometry (CSG) 
// is detemrine where a particle is relative to a shape or plane.
//
// for the sphere, the L2 norm (straight line distance) will suffice 
// as to 


pub mod norms;
pub use norms::*;
use uom::si::{f64::*, length::meter};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Region {
    Sphere(Sphere),
    Box,
    Plane,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub(crate) struct Sphere {
    x: Length,
    y: Length,
    z: Length,
    r: Length
}

impl Sphere {
    fn is_point_in_sphere(&self, point: [Length; 3]) -> bool {

        let center_meters: [f64;3] = [
            self.x.get::<meter>(),
            self.y.get::<meter>(),
            self.z.get::<meter>(),
        ];

        let point_meters: [f64;3] = [
            point[0].get::<meter>(),
            point[1].get::<meter>(),
            point[2].get::<meter>(),
        ];
        let radius_meters = self.r.get::<meter>();

        point_in_sphere(center_meters, radius_meters, point_meters)

    }
}

/// Returns `true` if point `p` is inside or on the sphere, `false` otherwise.
/// Sphere is defined by `center` and `radius`.
#[inline]
fn point_in_sphere(center: [f64; 3], radius: f64, p: [f64; 3]) -> bool {
    let dx = p[0] - center[0];
    let dy = p[1] - center[1];
    let dz = p[2] - center[2];

    // Compare squared distance to squared radius (avoids sqrt, faster)
    let dist2 = dx.mul_add(dx, dy.mul_add(dy, dz * dz));
    dist2 <= radius * radius
}
