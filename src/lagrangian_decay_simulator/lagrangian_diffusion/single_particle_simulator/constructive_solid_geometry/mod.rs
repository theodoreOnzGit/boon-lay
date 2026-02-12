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
use uom::{si::{f64::*, thermodynamic_temperature::kelvin}, ConstZero};

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
    pub fn get_triso_region(&self, coordinates: [Length;3]) -> TrisoRegion {

        if self.fuel_region.is_within_region(coordinates) {
            return TrisoRegion::Fuel;

        } else if self.buffer_region.is_within_region(coordinates) {

            return TrisoRegion::Buffer;
        } else if self.ipyc_region.is_within_region(coordinates) {

            return TrisoRegion::IPyC;
        } else if self.sic_region.is_within_region(coordinates) {

            return TrisoRegion::SiC;
        } else if self.opyc_region.is_within_region(coordinates) {

            return TrisoRegion::OPyC;
        } 

        // if it is not within any of these regions

        return TrisoRegion::Outside;
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

    // suppose a particle was inside the fuel 
    //
    // how would it determine the length to the boundary?
    // I would need a unit vector, or simply a displacement or velocity vector 
    //
    // it is more convenient to use a velocity vector 
    // to see how much time it takes to reach a boundary 

    pub fn get_time_to_sphere_boundary(
        position: [Length; 3],
        velocity: [Velocity; 3],
        triso_cell: TrisoCell,
    ) -> Option<Time> {
        // firstly i want to see what region i am in 

        let current_region = triso_cell.get_triso_region(position);

        // based on the current region, I'm going to obtain the spheres 

        match current_region {
            TrisoRegion::Fuel => {
                // if in the fuel region
                // get centre and radius of fuel 

                let fuel_sphere = triso_cell.fuel_region;
                let (center, radius) = 
                    fuel_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                let sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(center, radius, position, velocity);

                match sphere_crossing {
                    Some(SphereCrossing::Exit { t: time_to_sphere_exit }) => {
                        return Some(time_to_sphere_exit);
                    },
                    Some(SphereCrossing::Entry { t: _ }) => {
                        // if it's inside the fuel region,
                        // doesn't make sense for it to enter
                        return None;
                    },
                    None => return None,
                };


            },
            TrisoRegion::Buffer => {

                let inner_sphere = triso_cell.fuel_region;
                let (inner_center, inner_radius) = 
                    inner_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                let outer_sphere = triso_cell.buffer_region;
                let (outer_center, outer_radius) = 
                    outer_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                // assert both centres are the same
                assert_eq!(inner_center, outer_center);

                // check crossing for both inner and outer
                let inner_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(inner_center, inner_radius, position, velocity);
                let outer_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(outer_center, outer_radius, position, velocity);

                // if you are within the region between inner and outer spheres, you 
                // check if you are entering the inner sphere
                let time_to_inner_crossing_opt: Option<Time> = match inner_sphere_crossing {
                    Some(SphereCrossing::Exit { t: _time_to_sphere_exit }) => {

                        // you won't be exiting the inner sphere
                        // if you are outside of it
                        // this will be an error of sorts
                        return None;
                    },
                    Some(SphereCrossing::Entry { t: time_to_sphere_entry }) => {
                        // it makes sense for it to cross
                        Some(time_to_sphere_entry)
                    },
                    None => None,
                };

                // if you are within the region between inner and outer spheres, you 
                // check if you are exiting the outer sphere
                let time_to_outer_crossing_opt: Option<Time> = match outer_sphere_crossing {
                    Some(SphereCrossing::Exit { t: time_to_sphere_exit }) => {
                        Some(time_to_sphere_exit)
                    },
                    Some(SphereCrossing::Entry { t: _ }) => {
                        // if it's inside the outer region,
                        // doesn't make sense for it to enter the outer region
                        return None;
                    },
                    None => None,
                };

                // check which is the shorter time, 
                // this will be the correct time to the boundary
                //
                // first, let's deal with the fringe case both are none
                if time_to_inner_crossing_opt == None && time_to_outer_crossing_opt == None {
                    return None;
                }

                // now for sure, you will have an outer crossing 
                //
                // otherwise, there is some error

                let time_to_outer_crossing: Time = match time_to_outer_crossing_opt {
                    Some(time_to_outer_crossing) => {
                        time_to_outer_crossing
                    },
                    None => {
                        dbg!(&(triso_cell,position,velocity));
                        panic!()
                    },
                };

                // now let's do the time to inner crossing 

                match time_to_inner_crossing_opt {
                    Some(time_to_inner_crossing) => {
                        if time_to_outer_crossing < time_to_inner_crossing {
                            return Some(time_to_outer_crossing);
                        } else {
                            return Some(time_to_inner_crossing);
                        }
                    },
                    None => {
                        // if particle doesn't cross the inner sphere, 
                        // just return the time to outer crossing 
                        return Some(time_to_outer_crossing);

                    },


                }
                
                

            },
            TrisoRegion::IPyC => {

                let inner_sphere = triso_cell.buffer_region;
                let (inner_center, inner_radius) = 
                    inner_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                let outer_sphere = triso_cell.ipyc_region;
                let (outer_center, outer_radius) = 
                    outer_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                // assert both centres are the same
                assert_eq!(inner_center, outer_center);

                // check crossing for both inner and outer
                let inner_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(inner_center, inner_radius, position, velocity);
                let outer_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(outer_center, outer_radius, position, velocity);

                // if you are within the region between inner and outer spheres, you 
                // check if you are entering the inner sphere
                let time_to_inner_crossing_opt: Option<Time> = match inner_sphere_crossing {
                    Some(SphereCrossing::Exit { t: _time_to_sphere_exit }) => {

                        // you won't be exiting the inner sphere
                        // if you are outside of it
                        // this will be an error of sorts
                        return None;
                    },
                    Some(SphereCrossing::Entry { t: time_to_sphere_entry }) => {
                        // it makes sense for it to cross
                        Some(time_to_sphere_entry)
                    },
                    None => None,
                };

                // if you are within the region between inner and outer spheres, you 
                // check if you are exiting the outer sphere
                let time_to_outer_crossing_opt: Option<Time> = match outer_sphere_crossing {
                    Some(SphereCrossing::Exit { t: time_to_sphere_exit }) => {
                        Some(time_to_sphere_exit)
                    },
                    Some(SphereCrossing::Entry { t: _ }) => {
                        // if it's inside the outer region,
                        // doesn't make sense for it to enter the outer region
                        return None;
                    },
                    None => None,
                };

                // check which is the shorter time, 
                // this will be the correct time to the boundary
                //
                // first, let's deal with the fringe case both are none
                if time_to_inner_crossing_opt == None && time_to_outer_crossing_opt == None {
                    return None;
                }

                // now for sure, you will have an outer crossing 
                //
                // otherwise, there is some error

                let time_to_outer_crossing: Time = match time_to_outer_crossing_opt {
                    Some(time_to_outer_crossing) => {
                        time_to_outer_crossing
                    },
                    None => {
                        dbg!(&(triso_cell,position,velocity));
                        panic!()
                    },
                };

                // now let's do the time to inner crossing 

                match time_to_inner_crossing_opt {
                    Some(time_to_inner_crossing) => {
                        if time_to_outer_crossing < time_to_inner_crossing {
                            return Some(time_to_outer_crossing);
                        } else {
                            return Some(time_to_inner_crossing);
                        }
                    },
                    None => {
                        // if particle doesn't cross the inner sphere, 
                        // just return the time to outer crossing 
                        return Some(time_to_outer_crossing);

                    },


                }
                
                


            },
            TrisoRegion::SiC => {

                let inner_sphere = triso_cell.ipyc_region;
                let (inner_center, inner_radius) = 
                    inner_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                let outer_sphere = triso_cell.sic_region;
                let (outer_center, outer_radius) = 
                    outer_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                // assert both centres are the same
                assert_eq!(inner_center, outer_center);

                // check crossing for both inner and outer
                let inner_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(inner_center, inner_radius, position, velocity);
                let outer_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(outer_center, outer_radius, position, velocity);

                // if you are within the region between inner and outer spheres, you 
                // check if you are entering the inner sphere
                let time_to_inner_crossing_opt: Option<Time> = match inner_sphere_crossing {
                    Some(SphereCrossing::Exit { t: _time_to_sphere_exit }) => {

                        // you won't be exiting the inner sphere
                        // if you are outside of it
                        // this will be an error of sorts
                        return None;
                    },
                    Some(SphereCrossing::Entry { t: time_to_sphere_entry }) => {
                        // it makes sense for it to cross
                        Some(time_to_sphere_entry)
                    },
                    None => None,
                };

                // if you are within the region between inner and outer spheres, you 
                // check if you are exiting the outer sphere
                let time_to_outer_crossing_opt: Option<Time> = match outer_sphere_crossing {
                    Some(SphereCrossing::Exit { t: time_to_sphere_exit }) => {
                        Some(time_to_sphere_exit)
                    },
                    Some(SphereCrossing::Entry { t: _ }) => {
                        // if it's inside the outer region,
                        // doesn't make sense for it to enter the outer region
                        return None;
                    },
                    None => None,
                };

                // check which is the shorter time, 
                // this will be the correct time to the boundary
                //
                // first, let's deal with the fringe case both are none
                if time_to_inner_crossing_opt == None && time_to_outer_crossing_opt == None {
                    return None;
                }

                // now for sure, you will have an outer crossing 
                //
                // otherwise, there is some error

                let time_to_outer_crossing: Time = match time_to_outer_crossing_opt {
                    Some(time_to_outer_crossing) => {
                        time_to_outer_crossing
                    },
                    None => {
                        dbg!(&(triso_cell,position,velocity));
                        panic!()
                    },
                };

                // now let's do the time to inner crossing 

                match time_to_inner_crossing_opt {
                    Some(time_to_inner_crossing) => {
                        if time_to_outer_crossing < time_to_inner_crossing {
                            return Some(time_to_outer_crossing);
                        } else {
                            return Some(time_to_inner_crossing);
                        }
                    },
                    None => {
                        // if particle doesn't cross the inner sphere, 
                        // just return the time to outer crossing 
                        return Some(time_to_outer_crossing);

                    },


                }
                
                


            },
            TrisoRegion::OPyC => {

                let inner_sphere = triso_cell.sic_region;
                let (inner_center, inner_radius) = 
                    inner_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                let outer_sphere = triso_cell.opyc_region;
                let (outer_center, outer_radius) = 
                    outer_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();

                // assert both centres are the same
                assert_eq!(inner_center, outer_center);

                // check crossing for both inner and outer
                let inner_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(inner_center, inner_radius, position, velocity);
                let outer_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(outer_center, outer_radius, position, velocity);

                // if you are within the region between inner and outer spheres, you 
                // check if you are entering the inner sphere
                let time_to_inner_crossing_opt: Option<Time> = match inner_sphere_crossing {
                    Some(SphereCrossing::Exit { t: _time_to_sphere_exit }) => {

                        // you won't be exiting the inner sphere
                        // if you are outside of it
                        // this will be an error of sorts
                        return None;
                    },
                    Some(SphereCrossing::Entry { t: time_to_sphere_entry }) => {
                        // it makes sense for it to cross
                        Some(time_to_sphere_entry)
                    },
                    None => None,
                };

                // if you are within the region between inner and outer spheres, you 
                // check if you are exiting the outer sphere
                let time_to_outer_crossing_opt: Option<Time> = match outer_sphere_crossing {
                    Some(SphereCrossing::Exit { t: time_to_sphere_exit }) => {
                        Some(time_to_sphere_exit)
                    },
                    Some(SphereCrossing::Entry { t: _ }) => {
                        // if it's inside the outer region,
                        // doesn't make sense for it to enter the outer region
                        return None;
                    },
                    None => None,
                };

                // check which is the shorter time, 
                // this will be the correct time to the boundary
                //
                // first, let's deal with the fringe case both are none
                if time_to_inner_crossing_opt == None && time_to_outer_crossing_opt == None {
                    return None;
                }

                // now for sure, you will have an outer crossing 
                //
                // otherwise, there is some error

                let time_to_outer_crossing: Time = match time_to_outer_crossing_opt {
                    Some(time_to_outer_crossing) => {
                        time_to_outer_crossing
                    },
                    None => {
                        dbg!(&(triso_cell,position,velocity));
                        panic!()
                    },
                };

                // now let's do the time to inner crossing 

                match time_to_inner_crossing_opt {
                    Some(time_to_inner_crossing) => {
                        if time_to_outer_crossing < time_to_inner_crossing {
                            return Some(time_to_outer_crossing);
                        } else {
                            return Some(time_to_inner_crossing);
                        }
                    },
                    None => {
                        // if particle doesn't cross the inner sphere, 
                        // just return the time to outer crossing 
                        return Some(time_to_outer_crossing);

                    },


                }
                
                


            },
            TrisoRegion::Outside => {

                let inner_sphere = triso_cell.opyc_region;
                let (inner_center, inner_radius) = 
                    inner_sphere.try_return_center_and_radius_of_sphere()
                    .unwrap();


                // check crossing for both inner and outer
                let inner_sphere_crossing: Option<SphereCrossing> = 
                    sphere_first_crossing_uom(inner_center, inner_radius, position, velocity);

                // if you are within the region between inner and outer spheres, you 
                // check if you are entering the inner sphere
                let time_to_inner_crossing_opt: Option<Time> = match inner_sphere_crossing {
                    Some(SphereCrossing::Exit { t: _time_to_sphere_exit }) => {

                        // you won't be exiting the inner sphere
                        // if you are outside of it
                        // this will be an error of sorts
                        return None;
                    },
                    Some(SphereCrossing::Entry { t: time_to_sphere_entry }) => {
                        // it makes sense for it to cross
                        Some(time_to_sphere_entry)
                    },
                    None => None,
                };

                
                

                return time_to_inner_crossing_opt;

            },
        }





    }

    


    
}
