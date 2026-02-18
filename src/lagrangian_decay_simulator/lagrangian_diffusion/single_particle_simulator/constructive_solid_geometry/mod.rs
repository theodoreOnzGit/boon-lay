// the thing about constructive_solid_geometry (CSG) 
// is detemrine where a particle is relative to a shape or plane.
//
// for the sphere, the L2 norm (straight line distance) will suffice 
// as to 


pub mod norms;
use fission_yields_data::prelude::Nuclide;
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

    pub fn is_within_region(&self, point: [Length;3]) -> bool {

        match self {
            Region::Sphere(sphere) => sphere.is_point_in_sphere(point),
        }
    }

    pub fn try_return_center_and_radius_of_sphere(&self) -> 
        Option<([Length;3],Length)>{

            match self {
                Region::Sphere(sphere) => {
                    let centre = [sphere.x, sphere.y, sphere.z];
                    let radius = sphere.r;

                    Some((centre,radius))
                },

            }
    }
}


pub(crate) mod sphere;
pub(crate) use sphere::*;
use uom::{ConstZero, si::{f64::*, length::meter, thermodynamic_temperature::kelvin, time::second}};

use crate::lagrangian_decay_simulator::lagrangian_diffusion::{single_particle_simulator::constructive_solid_geometry::chatgpt_vibe_coded_sphere_crossing::{sphere_first_crossing_uom, SphereCrossing}, temperature_dependent_collisions::{try_get_diffusion_coeff_jiang, TrisoPebbleLayerMaterial}};

// for a single triso particle, 
// it is many cocentric spheres together

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TrisoCell {
    fuel_region: Region,
    buffer_region: Region,
    ipyc_region: Region,
    sic_region: Region,
    opyc_region: Region,

    // temperatures for each region
    fuel_region_temp: ThermodynamicTemperature,
    buffer_region_temp: ThermodynamicTemperature,
    ipyc_region_temp: ThermodynamicTemperature,
    sic_region_temp: ThermodynamicTemperature,
    opyc_region_temp: ThermodynamicTemperature,

    // neutron fluence for the triso as a whole (mean free path is short,
    // I'm just going to use one neutron fluence)
    gamma_neutron_fluence: ArealNumberDensity,
}


impl TrisoCell {
    /// creates a new triso cell based on the radii
    pub fn new(fuel_radius: Length,
        buffer_radius: Length,
        ipyc_radius: Length,
        sic_radius: Length,
        opyc_radius: Length) -> Self {
        // first, need to ensure the lengths are correct
        assert!(buffer_radius > fuel_radius);
        assert!(ipyc_radius > buffer_radius);
        assert!(sic_radius > ipyc_radius);
        assert!(opyc_radius > sic_radius);

        // let's have a center 
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

        let default_temperature = ThermodynamicTemperature::new::<kelvin>(1800.0);
        let default_fluence = ArealNumberDensity::ZERO;

        
        return TrisoCell {
            fuel_region,
            buffer_region,
            ipyc_region,
            sic_region,
            opyc_region,
            fuel_region_temp: default_temperature,
            buffer_region_temp: default_temperature,
            ipyc_region_temp: default_temperature,
            sic_region_temp: default_temperature,
            opyc_region_temp: default_temperature,
            gamma_neutron_fluence: default_fluence,
        };


    }

    /// checks which region the particle is in 
    /// chatgpt fixed
    pub fn get_triso_region(&self, coordinates: [Length;3]) -> TrisoRegion {
        // Choose eps: tune based on your smallest layer thickness.
        // This is a conservative default.
        let eps = Length::new::<meter>(1e-12);

        let x = coordinates[0];
        let y = coordinates[1];
        let z = coordinates[2];
        let r = (x * x + y * y + z * z).sqrt();

        // Extract radii (meters). Assumes concentric spheres.
        let (_, r_fuel) = self.fuel_region.try_return_center_and_radius_of_sphere().unwrap();
        let (_, r_buf)  = self.buffer_region.try_return_center_and_radius_of_sphere().unwrap();
        let (_, r_ipyc) = self.ipyc_region.try_return_center_and_radius_of_sphere().unwrap();
        let (_, r_sic)  = self.sic_region.try_return_center_and_radius_of_sphere().unwrap();
        let (_, r_opyc) = self.opyc_region.try_return_center_and_radius_of_sphere().unwrap();


        // Convention: boundaries belong to the OUTER region.
        // i.e. r == r_fuel -> Buffer, r == r_buf -> IPyC, etc.
        if r < r_fuel - eps {
            TrisoRegion::Fuel
        } else if r < r_buf - eps {
            TrisoRegion::Buffer
        } else if r < r_ipyc - eps {
            TrisoRegion::IPyC
        } else if r < r_sic - eps {
            TrisoRegion::SiC
        } else if r < r_opyc - eps {
            TrisoRegion::OPyC
        } else {
            TrisoRegion::Outside
        }
    }

    /// checks the diffusion coefficient based on coordinates of the 
    /// triso particle
    pub fn try_get_diffusion_coefficient(
        &self, coordinates: [Length;3], 
        nuclide: Nuclide)
        -> Option<DiffusionCoefficient>{

        if self.fuel_region.is_within_region(coordinates) {
            // obtain diffusion coeff for kernel
            let triso_layer = TrisoPebbleLayerMaterial::KernelUO2;
            let diffusion_coeff = try_get_diffusion_coeff_jiang(
                triso_layer, nuclide, 
                self.fuel_region_temp, 
                Some(self.gamma_neutron_fluence)
            );
            return diffusion_coeff;

        } else if self.buffer_region.is_within_region(coordinates) {

            let triso_layer = TrisoPebbleLayerMaterial::Buffer;
            let diffusion_coeff = try_get_diffusion_coeff_jiang(
                triso_layer, nuclide, 
                self.buffer_region_temp, 
                Some(self.gamma_neutron_fluence)
            );
            return diffusion_coeff;
        } else if self.ipyc_region.is_within_region(coordinates) {

            let triso_layer = TrisoPebbleLayerMaterial::PyC;
            let diffusion_coeff = try_get_diffusion_coeff_jiang(
                triso_layer, nuclide, 
                self.ipyc_region_temp, 
                Some(self.gamma_neutron_fluence)
            );
            return diffusion_coeff;
        } else if self.sic_region.is_within_region(coordinates) {

            let triso_layer = TrisoPebbleLayerMaterial::SiC;
            let diffusion_coeff = try_get_diffusion_coeff_jiang(
                triso_layer, nuclide, 
                self.sic_region_temp, 
                Some(self.gamma_neutron_fluence)
            );
            return diffusion_coeff;
        } else if self.opyc_region.is_within_region(coordinates) {

            let triso_layer = TrisoPebbleLayerMaterial::PyC;
            let diffusion_coeff = try_get_diffusion_coeff_jiang(
                triso_layer, nuclide, 
                self.opyc_region_temp, 
                Some(self.gamma_neutron_fluence)
            );
            return diffusion_coeff;
        } 

        // if it is not within any of these regions

        return None;


    }

    #[inline]
    pub fn get_time_to_sphere_boundary(&self,
        position: [Length;3],
        velocity: [Velocity;3],) -> Option<Time>{

        TrisoRegion::get_time_to_sphere_boundary(position, velocity, *self)
    }
}

// question is, how to do particle tracing if the length crosses boundary 
// of the sphere?



#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TrisoRegion {
    Fuel,
    Buffer,
    IPyC,
    SiC,
    OPyC,
    Outside
}

impl TrisoRegion {
    #[inline]
    pub fn get_time_to_sphere_boundary(
        position: [Length; 3],
        velocity: [Velocity; 3],
        triso_cell: TrisoCell,
    ) -> Option<Time> {
        // Filter out "hits" that are at t=0 or negative (common when starting on boundary)
        const T_EPS_S: f64 = 1e-15;
        let t_eps = Time::new::<second>(T_EPS_S);

        #[inline]
        fn crossing_time_any(c: Option<SphereCrossing>, t_eps: Time) -> Option<Time> {
            let t = match c {
                Some(SphereCrossing::Exit { t }) => Some(t),
                Some(SphereCrossing::Entry { t }) => Some(t),
                None => None,
            };
            t.filter(|&tt| tt > t_eps)
        }

        #[inline]
        fn pick_min_positive(t1: Option<Time>, t2: Option<Time>) -> Option<Time> {
            match (t1, t2) {
                (Some(a), Some(b)) => Some(if a < b { a } else { b }),
                (Some(a), None) => Some(a),
                (None, Some(b)) => Some(b),
                (None, None) => None,
            }
        }

        let current_region = triso_cell.get_triso_region(position);

        match current_region {
            TrisoRegion::Fuel => {
                let (center, radius) = triso_cell
                    .fuel_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                crossing_time_any(
                    sphere_first_crossing_uom(center, radius, position, velocity),
                    t_eps,
                )
            }

            TrisoRegion::Buffer => {
                let (c1, r1) = triso_cell
                    .fuel_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                let (c2, r2) = triso_cell
                    .buffer_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                assert_eq!(c1, c2);

                let t_inner =
                    crossing_time_any(sphere_first_crossing_uom(c1, r1, position, velocity), t_eps);
                let t_outer =
                    crossing_time_any(sphere_first_crossing_uom(c2, r2, position, velocity), t_eps);
                pick_min_positive(t_inner, t_outer)
            }

            TrisoRegion::IPyC => {
                let (c1, r1) = triso_cell
                    .buffer_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                let (c2, r2) = triso_cell
                    .ipyc_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                assert_eq!(c1, c2);

                let t_inner =
                    crossing_time_any(sphere_first_crossing_uom(c1, r1, position, velocity), t_eps);
                let t_outer =
                    crossing_time_any(sphere_first_crossing_uom(c2, r2, position, velocity), t_eps);
                pick_min_positive(t_inner, t_outer)
            }

            TrisoRegion::SiC => {
                let (c1, r1) = triso_cell
                    .ipyc_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                let (c2, r2) = triso_cell
                    .sic_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                assert_eq!(c1, c2);

                let t_inner =
                    crossing_time_any(sphere_first_crossing_uom(c1, r1, position, velocity), t_eps);
                let t_outer =
                    crossing_time_any(sphere_first_crossing_uom(c2, r2, position, velocity), t_eps);
                pick_min_positive(t_inner, t_outer)
            }

            TrisoRegion::OPyC => {
                let (c1, r1) = triso_cell
                    .sic_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                let (c2, r2) = triso_cell
                    .opyc_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                assert_eq!(c1, c2);

                let t_inner =
                    crossing_time_any(sphere_first_crossing_uom(c1, r1, position, velocity), t_eps);
                let t_outer =
                    crossing_time_any(sphere_first_crossing_uom(c2, r2, position, velocity), t_eps);
                pick_min_positive(t_inner, t_outer)
            }

            TrisoRegion::Outside => {
                let (center, radius) = triso_cell
                    .opyc_region
                    .try_return_center_and_radius_of_sphere()
                    .unwrap();
                crossing_time_any(
                    sphere_first_crossing_uom(center, radius, position, velocity),
                    t_eps,
                )
            }
        }
    }
}

/// this is a vibe coded sphere crossing code
/// to determine time to sphere crossing
pub mod chatgpt_vibe_coded_sphere_crossing;
