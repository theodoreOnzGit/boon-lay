// the thing about constructive_solid_geometry (CSG) 
// is detemrine where a particle is relative to a shape or plane.
//
// for the sphere, the L2 norm (straight line distance) will suffice 
// as to 


pub mod norms;
pub use norms::*;
use uom::si::f64::*;

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
