// the thing about constructive_solid_geometry (CSG) 
// is detemrine where a particle is relative to a shape or plane.
//
// for the sphere, the L2 norm (straight line distance) will suffice 
// as to 


pub mod norms;
pub use norms::*;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Region {
    Sphere(Sphere),
}

impl Region {
    pub fn new_sphere(center: [Length; 3], radius: Length) -> Self {

        let sphere: Sphere = Sphere {
            x: center[0],
            y: center[1],
            z: center[2],
            r: radius,
        };


        return Region::Sphere(sphere);

    }
}


pub(crate) mod sphere;
pub(crate) use sphere::*;
use uom::{si::f64::*, ConstZero};

// for a single triso particle, 
// it is many cocentric spheres together

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TrisoCell {
    fuel_region: Region,
    buffer_region: Region,
    ipyc_region: Region,
    sic_region: Region,
    opyc_region: Region,
}

impl TrisoCell {
    /// creates a new triso cell based on the radii
    pub fn new(fuel_radius: Length,
        buffer_radius: Length,
        ipyc_radius: Length,
        sic_radius: Length,
        opyc_radius: Length) -> Self {

        let center = [Length::ZERO, Length::ZERO, Length::ZERO];
        let fuel_region = Region::new_sphere(center, fuel_radius);
        let buffer_region 
            = Region::new_sphere(center, buffer_radius);
        let ipyc_region 
            = Region::new_sphere(center, ipyc_radius);
        let sic_region 
            = Region::new_sphere(center, sic_radius);
        let opyc_region 
            = Region::new_sphere(center, opyc_radius);

        
        return TrisoCell {
            fuel_region,
            buffer_region,
            ipyc_region,
            sic_region,
            opyc_region,
        };


    }

    /// checks the diffusion coefficient based on coordinates of the 
    /// triso particle
    pub fn try_get_diffusion_coefficient(
        &self, coordinates: [Length;3]) -> Option<DiffusionCoefficient>{

        todo!()
    }
}

// question is, how to do particle tracing if the length crosses boundary 
// of the sphere?

