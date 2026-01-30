
use boon_lay::prelude::SingleNuclideSimulatorMC;
use eframe::{egui, egui::{Color32, Pos2, Stroke, Widget}};
use egui::{Rect, Ui};
use uom::si::{f64::*, length::micrometer, ratio::ratio};

use crate::decay_simulator_v1::DecaySimApp;


#[derive(Clone,Copy, Debug)]
pub struct TrisoParticleUi {
    // Metadata: physical diameter (not used for scaling unless you decide to map mm→px)
    diameter_mm: f32,

    // Fraction of the smallest UI dimension that the particle’s diameter should occupy.
    // 0.40 means the drawn diameter will be ~40% of the panel's min(width, height).
    ui_diameter_ratio: f32,

    // How many concentric rings to draw (uniform spacing from center to outer radius).
    num_rings: usize,

    // Line style for rings.
    stroke: Stroke,

    // Optional tint for the rings (stroke color), background is taken from the panel.
    color: Color32,

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
            diameter_mm: 1.0,          // 1 mm (metadata)
            ui_diameter_ratio: 0.8,   // occupy ~80% of the UI
            num_rings: 18,             // adjust to taste
            stroke: Stroke { width: 6.0, color: Color32::WHITE },
            color: Color32::WHITE,
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
        let outer_radius = 0.5 * self.ui_diameter_ratio * min_dim; // radius = 0.5 * (ratio * min_dim)
        let center: Pos2 = rect.center();

        // Painter for drawing.
        let painter = ui.painter_at(rect);

        // Stroke setup (allow color override separate from stroke.color if desired).
        let mut stroke = self.stroke;
        stroke.color = self.color;

        // Draw concentric rings from inner to outer radius.
        // If num_rings == 0, we draw just a single circle at outer_radius.
        if self.num_rings == 0 {
            if outer_radius > 0.0 {
                painter.circle_stroke(center, outer_radius, stroke);
            }
        } else {
            let spacing = outer_radius / (self.num_rings as f32);
            // Start slightly away from 0 to avoid a degenerate tiny center circle thicker than its radius.
            for i in 1..=self.num_rings {
                let r = spacing * (i as f32);
                if r > 0.0 {
                    painter.circle_stroke(center, r, stroke);
                }
            }
        }

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
        let triso_diameter: Length = self.diameter_after_opyc();

        // then I scale the width of the radionuclide by like 2% of the triso 
        // particle
        let radionuclide_x_width_pixels = triso_width_pixels/100.0;
        let radionuclide_y_width_pixels = triso_centre_x_pixels;

        // next i need a code to convert diameter coordinates to pixels 

        // first is to get scaling right 
        let scale_length_per_pixel: Length = triso_diameter/triso_width_pixels as f64;

        fn convert_coordinate_to_pixel( 
            coordinate: (Length, Length, Length),
            scale_length_per_pixel: Length) -> (f32, f32, f32) {

            let (x,y,z) = coordinate;
            let x_pixel: f32 = (x/scale_length_per_pixel).get::<ratio>() as f32;
            let y_pixel: f32 = (y/scale_length_per_pixel).get::<ratio>() as f32;
            let z_pixel: f32 = (z/scale_length_per_pixel).get::<ratio>() as f32;

            return (x_pixel,y_pixel,z_pixel);

        }


        let painter = ui.painter();
        for radionuclide_sim in sampled_particle_sims_for_plotting {

            // first lets get x,y,z relative to the centre 

            let radionuclide_position = radionuclide_sim.position;

            let (x_pixel,y_pixel, z_pixel) = 
                convert_coordinate_to_pixel(radionuclide_position, scale_length_per_pixel);

            let radionuclide_center_x_pixels = triso_centre_x_pixels + x_pixel;
            let radionuclide_center_y_pixels = triso_centre_y_pixels + y_pixel;


            let top_left_x: f32 = radionuclide_center_x_pixels - 0.5 * radionuclide_x_width_pixels;
            let top_left_y: f32 = radionuclide_center_y_pixels - 0.5 * radionuclide_y_width_pixels;
            let bottom_right_x: f32 = radionuclide_center_x_pixels + 0.5 * radionuclide_x_width_pixels;
            let bottom_right_y: f32 = radionuclide_center_y_pixels + 0.5 * radionuclide_y_width_pixels;

            let rect: Rect = Rect {
                // top left
                min: Pos2 { x: top_left_x, y: top_left_y },
                // bottom right
                max: Pos2 { x: bottom_right_x, y: bottom_right_y },
            };

            // now let's obtain the nuclide 
            let nuclide = radionuclide_sim.get_current_nuclide();
            let colour = DecaySimApp::element_color(nuclide);

            let center = Pos2::new(radionuclide_center_x_pixels, radionuclide_center_y_pixels);
            let radius = radionuclide_x_width_pixels;
            painter.circle_filled(center, radius, colour);
        }


    }

    // vibe coded
    pub fn diameter_after_buffer(&self) -> Length {
        self.kernel_diameter + self.buffer_thickness * 2.0
    }
    pub fn diameter_after_ipyc(&self) -> Length {
        self.diameter_after_buffer() + self.ipyc_thickness * 2.0
    }
    pub fn diameter_after_sic(&self) -> Length {
        self.diameter_after_ipyc() + self.sic_thickness * 2.0
    }
    pub fn diameter_after_opyc(&self) -> Length {
        self.diameter_after_sic() + self.opyc_thickness * 2.0
    }


}
