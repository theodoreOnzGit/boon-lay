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

pub(crate) mod sphere;
pub(crate) use sphere::*;

