
use boon_lay::Nuclide;
use boon_lay::prelude::SingleNuclideSimulatorMC;
use eframe::egui;
use eframe::egui::Widget;
use eframe::egui::Pos2;
use egui::Rect;
use egui::Ui;
use uom::si::f64::*;
use uom::si::ratio::ratio;
use uom::si::length::millimeter;
use uom::si::length::micrometer;

use crate::decay_simulator_v1::DecaySimApp;


#[derive(Clone,Copy, Debug)]
pub struct TrisoParticleUi {


    // fuel kernel diameter 
    kernel_diameter: Length, 

    // buffer thickness 
    buffer_thickness: Length,

    // inner pyrolytic carbon layer thickness
    ipyc_thickness: Length,
    // silicon carbide thickness
    sic_thickness: Length,
    // outer pyrolytic carbon thickness
    opyc_thickness: Length,
}

impl Default for TrisoParticleUi {
    fn default() -> Self {
        // Nominal values commonly cited in literature
        let kernel_diameter: Length = Length::new::<micrometer>(350.0);   // diameter
        let buffer_thickness: Length = Length::new::<micrometer>(100.0);  // thickness
        let ipyc_thickness: Length = Length::new::<micrometer>(40.0);     // thickness
        let sic_thickness: Length = Length::new::<micrometer>(35.0);      // thickness
        let opyc_thickness: Length = Length::new::<micrometer>(40.0);     // thickness

        Self {
            kernel_diameter,
            buffer_thickness,
            ipyc_thickness,
            sic_thickness,
            opyc_thickness,
        }
    }
}

// Implement the Widget trait so you can `ui.add(triso.clone())`
impl Widget for TrisoParticleUi {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Reserve all available space in the current UI region.
        let desired = ui.available_size();
        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::hover());

        // Compute center and target radius based on the 40% diameter target.
        let min_dim = rect.width().min(rect.height());
        let outer_radius: f32 = 0.5 * min_dim; // radius = 0.5 * (ratio * min_dim)
        let center: Pos2 = rect.center();

        // Painter for drawing.
        let painter = ui.painter_at(rect);


        // lets do a proper triso particle 

        // first let's get the inner kernel diameter 
        let inner_kernel_radius: Length = 0.5 * self.kernel_diameter;
        let buffer_radius: Length = 0.5 * self.get_diameter_after_buffer();
        let ipyc_radius: Length = 0.5 * self.get_diameter_after_ipyc();
        let sic_radius: Length = 0.5 * self.get_diameter_after_sic();
        let opyc_radius: Length = 0.5 * self.get_diameter_after_opyc();

        // second, convert it to pixels
        let scale_length_per_pixel: Length = opyc_radius/(outer_radius as f64);
        let inner_kernel_radius_pixels: f32 
            = (inner_kernel_radius/scale_length_per_pixel).get::<ratio>() as f32;
        let buffer_radius_pixels: f32 
            = (buffer_radius/scale_length_per_pixel).get::<ratio>() as f32;
        let ipyc_radius_pixels: f32 
            = (ipyc_radius/scale_length_per_pixel).get::<ratio>() as f32;
        let sic_radius_pixels: f32 
            = (sic_radius/scale_length_per_pixel).get::<ratio>() as f32;
        let opyc_radius_pixels: f32 
            = (opyc_radius/scale_length_per_pixel).get::<ratio>() as f32;

        // this is for colouring
        let fuel_kernel_nuclide = Nuclide::U235;
        let buffer_nuclide = Nuclide::C12;
        let ipyc_nuclide = Nuclide::C12;
        let sic_nuclide = Nuclide::Si28;
        let opyc_nuclide = Nuclide::C12;

        let fuel_kernel_colour = DecaySimApp::element_color(fuel_kernel_nuclide);
        let buffer_colour = DecaySimApp::element_color(buffer_nuclide);
        let ipyc_colour = DecaySimApp::element_color(ipyc_nuclide);
        let sic_colour = DecaySimApp::element_color(sic_nuclide);
        let opyc_colour = DecaySimApp::element_color(opyc_nuclide);

        // painter response is important!
        //
        // you paint the largest layer and work your way in
        painter.circle_filled(center, opyc_radius_pixels, opyc_colour);
        painter.circle_filled(center, sic_radius_pixels, sic_colour);
        painter.circle_filled(center, ipyc_radius_pixels, ipyc_colour);
        painter.circle_filled(center, buffer_radius_pixels, buffer_colour);
        painter.circle_filled(center, inner_kernel_radius_pixels, fuel_kernel_colour);


        response
    }
}


impl TrisoParticleUi {

    /// copied this from my tuas solver
    ///
    /// this also sets the appropriate width
    /// for the triso particle in pixels
    pub fn put_self_with_size_and_centre(
        &mut self,
        ui: &mut Ui, 
        centre_x_pixels: f32,
        centre_y_pixels: f32,
        x_width_pixels: f32,
        y_width_pixels: f32){

        let top_left_x: f32 = centre_x_pixels - 0.5 * x_width_pixels;
        let top_left_y: f32 = centre_y_pixels - 0.5 * y_width_pixels;
        let bottom_right_x: f32 = centre_x_pixels + 0.5 * x_width_pixels;
        let bottom_right_y: f32 = centre_y_pixels + 0.5 * y_width_pixels;

        let rect: Rect = Rect {
            // top left
            min: Pos2 { x: top_left_x, y: top_left_y },
            // bottom right
            max: Pos2 { x: bottom_right_x, y: bottom_right_y },
        };

        ui.put(rect, *self);

    }

    // coded myself 
    pub fn put_particle_vector_with_size_and_centre(
        &mut self,
        ui: &mut Ui, 
        triso_centre_x_pixels: f32,
        triso_centre_y_pixels: f32,
        triso_width_pixels: f32,
        sampled_particle_sims_for_plotting: Vec<SingleNuclideSimulatorMC>,
    ){

        // first i get the diameter
        let triso_diameter: Length = self.get_diameter_after_opyc();

        // then I scale the width of the radionuclide by like 2% of the triso 
        // particle
        let radionuclide_x_width_pixels = triso_width_pixels * 0.003;

        // next i need a code to convert diameter coordinates to pixels 

        // first is to get scaling right 
        let scale_length_per_pixel: Length = triso_diameter/triso_width_pixels as f64;



        let painter = ui.painter();
        // first, i want to get a slice of 
        // particles, between z = 0.1 mm and z = -0.1mm 
        let z_max = Length::new::<millimeter>(0.1);
        let z_min = Length::new::<millimeter>(-0.1);

        for radionuclide_sim in sampled_particle_sims_for_plotting {

            // first lets get x,y,z relative to the centre 

            let radionuclide_position = radionuclide_sim.position;

            let (_x, _y, z) = radionuclide_position;

            // we only want to plot a fraction of radionuclides
            if z > z_max {
                continue;
            }
            if z < z_min {
                continue;
            }
            

            let (x_pixel,y_pixel, _z_pixel) = 
                Self::convert_coordinate_to_pixel(
                    radionuclide_position, 
                    scale_length_per_pixel
                );

            

            let radionuclide_center_x_pixels = triso_centre_x_pixels + x_pixel;
            let radionuclide_center_y_pixels = triso_centre_y_pixels + y_pixel;


            // now let's obtain the nuclide 
            let nuclide = radionuclide_sim.get_current_nuclide();
            let colour = DecaySimApp::element_color(nuclide);

            let center = Pos2::new(
                radionuclide_center_x_pixels, 
                radionuclide_center_y_pixels
            );
            let radius = radionuclide_x_width_pixels;
            painter.circle_filled(center, radius, colour);
        }


    }

    // vibe coded
    pub fn get_diameter_after_buffer(&self) -> Length {
        self.kernel_diameter + self.buffer_thickness * 2.0
    }
    pub fn get_diameter_after_ipyc(&self) -> Length {
        self.get_diameter_after_buffer() + self.ipyc_thickness * 2.0
    }
    pub fn get_diameter_after_sic(&self) -> Length {
        self.get_diameter_after_ipyc() + self.sic_thickness * 2.0
    }
    pub fn get_diameter_after_opyc(&self) -> Length {
        self.get_diameter_after_sic() + self.opyc_thickness * 2.0
    }

    pub fn convert_coordinate_to_pixel( 
        coordinate: (Length, Length, Length),
        scale_length_per_pixel: Length) -> (f32, f32, f32) {

        let (x,y,z) = coordinate;
        let x_pixel: f32 = (x/scale_length_per_pixel).get::<ratio>() as f32;
        let y_pixel: f32 = (y/scale_length_per_pixel).get::<ratio>() as f32;
        let z_pixel: f32 = (z/scale_length_per_pixel).get::<ratio>() as f32;

        return (x_pixel,y_pixel,z_pixel);

    }

}
