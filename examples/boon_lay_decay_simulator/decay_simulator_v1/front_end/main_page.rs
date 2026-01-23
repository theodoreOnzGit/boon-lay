use boon_lay::{prelude::{decay_library::DecayLibrary, SingleNuclideSimulatorMC}, Nuclide};
use egui::{Color32, Pos2, Rect, Ui};

use crate::decay_simulator_v1::DecaySimApp;
use rayon::prelude::*;

impl DecaySimApp {

    pub fn main_page(&mut self, ui: &mut Ui) {

        let ui_rectangle: Rect = ui.min_rect();
        let viewport = ui.clip_rect();

        let _left_most_side = ui_rectangle.left();
        let _top_most_side = ui_rectangle.top();

        // this part is vibe coded
        // Fixed drawing area: 2500 x 2500 pixels
        const SIZE: f32 = 2500.0;
        const COLS: usize = 500;
        const ROWS: usize = 500;

        // Reserve exactly 2500x2500 px in the UI (won't resize with the panel)
        let (rect, _response) = ui.allocate_exact_size(egui::vec2(SIZE, SIZE), egui::Sense::hover());

        // Grid cell size (fixed, independent of the UI rectangle size)
        let dx = SIZE / COLS as f32; // 3.2 px
        let dy = SIZE / ROWS as f32; // 3.2 px
        let radius = 0.45 * dx.min(dy); // ~1.44 px

        let origin = rect.min; // top-left of the allocated 2500x2500 area

        // let me obtain the four vectors of nuclides 
        
        // Convert a Vec<SingleNuclideSimualtorMC> into Vec<Nuclide>
        // using the provided getter.

        // also vibe coded
        fn to_nuclides(simulators: &[SingleNuclideSimulatorMC]) -> Vec<Nuclide> {
            simulators.iter().map(|s| s.get_current_nuclide()).collect()
        }

        // If you have four vectors:
        fn convert_all(
            v0: &[SingleNuclideSimulatorMC],
            v1: &[SingleNuclideSimulatorMC],
            v2: &[SingleNuclideSimulatorMC],
            v3: &[SingleNuclideSimulatorMC],
        ) -> (Vec<Nuclide>, Vec<Nuclide>, Vec<Nuclide>, Vec<Nuclide>) {
            (
                to_nuclides(v0),
                to_nuclides(v1),
                to_nuclides(v2),
                to_nuclides(v3),
            )
        }

        // collect all nuclides and then display them
        let (nuclide_sim_vec_1,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_1_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_2,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_2_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_3,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_3_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_4,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_4_ptr.lock().unwrap().clone();

        let (nuclide_vec_1, nuclide_vec_2, nuclide_vec_3, nuclide_vec_4): 
            (Vec<Nuclide>, Vec<Nuclide>, Vec<Nuclide>, Vec<Nuclide>) 
             = convert_all(
                 &nuclide_sim_vec_1, 
                 &nuclide_sim_vec_2, 
                 &nuclide_sim_vec_3, 
                 &nuclide_sim_vec_4,
             );

        let full_nuclide_vector: Vec<Nuclide> = 
            nuclide_vec_1.into_iter()
            .chain(nuclide_vec_2)
            .chain(nuclide_vec_3)
            .chain(nuclide_vec_4)
            .collect();

        //let mut nuclide_index = 0;


        //for row in 0..ROWS {
        //    for col in 0..COLS {
        //        // Center each circle in its cell
        //        let x = origin.x + (col as f32 + 0.5) * dx;
        //        let y = origin.y + (row as f32 + 0.5) * dy;
        //        let center = egui::pos2(x, y);

        //        // Example color gradient by position (any palette can be used)
        //        //let r = (col * 255 / (COLS - 1)) as u8;
        //        //let g = (row * 255 / (ROWS - 1)) as u8;
        //        //let b = 160u8;
        //        // not vibe coded:
        //        // now to obtain colour, we get the nuclide index
        //        // 
        //        let nuclide = full_nuclide_vector[nuclide_index];
        //        let color = Self::element_color(nuclide);
        //        nuclide_index += 1;

        //        // just assert to be printing different nuclides, 
        //        // this works correct 
        //        // dbg!(&nuclide_index);

        //        painter.circle_filled(center, radius, color);
        //    }
        //}

        // refactored using vibe coding to increase smoothness
        #[derive(Clone, Copy)]
        struct CircleInst {
            center: egui::Pos2,
            radius: f32,
            color: egui::Color32,
        }

        fn draw_grid_parallel(
            ui: &mut egui::Ui,
            origin: egui::Pos2,
            dx: f32,
            dy: f32,
            radius: f32,
            full_nuclide_vector: &[Nuclide],
            rows: usize,
            cols: usize,
            viewport: Rect,
        ) {
            // Parallel precompute
            let circles: Vec<CircleInst> = (0..rows * cols)
                .into_par_iter()
                .map(|idx| {
                    let row = idx / cols;
                    let col = idx % cols;

                    let x = origin.x + (col as f32 + 0.5) * dx;
                    let y = origin.y + (row as f32 + 0.5) * dy;
                    let center = egui::pos2(x, y);

                    let nuclide = full_nuclide_vector[idx];
                    let color = DecaySimApp::element_color(nuclide);

                    CircleInst { center, radius, color }
                })
            .collect();

            // Single-threaded draw (UI thread)
            let painter = ui.painter();
            let content_origin: Pos2 = ui.min_rect().min;
            let content_origin_rect: Rect = ui.min_rect();

            let left_limit = content_origin_rect.left();
            let top_limit = content_origin_rect.top();

            let right_limit = left_limit + viewport.right();
            let bottom_limit = top_limit + viewport.bottom();

            
            // so basically, i need to get the position relative to the content origin 
            
            let mut skipped_circles_counter = 0;

            for c in &circles {


                let circle_abs_pos_x: f32 = content_origin.x + c.center.x;
                let circle_abs_pos_y: f32 = content_origin.y + c.center.y;

                if circle_abs_pos_x < left_limit || circle_abs_pos_x > right_limit   {
                    skipped_circles_counter += 1;
                    continue;
                };
                if circle_abs_pos_y < top_limit || circle_abs_pos_y > bottom_limit   {
                    skipped_circles_counter += 1;
                    continue;
                };

                

                painter.circle_filled(c.center, c.radius, c.color);
            }

            dbg!(&skipped_circles_counter);

        }

        draw_grid_parallel(ui, origin, dx, dy, radius, 
            &full_nuclide_vector, ROWS, COLS,
            viewport);

    }


    /// this is a vibe coded colour scheme for elements in the periodic table
    pub fn element_color(nuclide: Nuclide) -> Color32 {
        // Define palette for categories (pick your own colors if you prefer)
        const HYDROGEN: Color32 = Color32::from_rgb(255, 255, 255);      // White
        const ALKALI: Color32 = Color32::from_rgb(255, 128, 0);          // Orange
        const ALKALINE_EARTH: Color32 = Color32::from_rgb(255, 215, 0);  // Gold
        const TRANSITION: Color32 = Color32::from_rgb(70, 130, 180);     // Steel blue
        const LANTHANOID: Color32 = Color32::from_rgb(123, 104, 238);    // Medium slate blue
        const ACTINOID: Color32 = Color32::from_rgb(199, 21, 133);       // Medium violet red
        const POST_TRANSITION: Color32 = Color32::from_rgb(176, 196, 222); // Light steel blue
        const METALLOID: Color32 = Color32::from_rgb(0, 128, 0);         // Green
        const OTHER_NONMETAL: Color32 = Color32::from_rgb(34, 139, 34);  // Forest green
        const HALOGEN: Color32 = Color32::from_rgb(0, 255, 255);         // Cyan
        const NOBLE_GAS: Color32 = Color32::from_rgb(135, 206, 235);     // Sky blue
        const UNKNOWN: Color32 = Color32::from_rgb(128, 128, 128);       // Gray
                                                                         //

        let (z,_a) = nuclide.get_z_a();

        match z {
            // Special case
            1 => HYDROGEN,

            // Noble gases
            2 | 10 | 18 | 36 | 54 | 86 | 118 => NOBLE_GAS,

            // Alkali metals
            3 | 11 | 19 | 37 | 55 | 87 => ALKALI,

            // Alkaline earth metals
            4 | 12 | 20 | 38 | 56 | 88 => ALKALINE_EARTH,

            // Transition metals
            21..=30 | 39..=48 | 72..=80 | 104..=112 => TRANSITION,

            // Lanthanoids
            57..=71 => LANTHANOID,

            // Actinoids
            89..=103 => ACTINOID,

            // Post-transition metals (poor metals)
            13 | 31 | 49 | 50 | 81 | 82 | 83 | 84 | 113 | 114 | 115 | 116 => POST_TRANSITION,

            // Metalloids (semimetals)
            5 | 14 | 32 | 33 | 51 | 52 => METALLOID,

            // Other nonmetals
            6 | 7 | 8 | 15 | 16 | 34 => OTHER_NONMETAL,

            // Halogens
            9 | 17 | 35 | 53 | 85 | 117 => HALOGEN,

            // If out of range or unclassified
            _ => UNKNOWN,
        }
    }
}
