use boon_lay::Nuclide;
use egui::{Color32, Rect, Ui};

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {

    pub fn main_page(&mut self, ui: &mut Ui) {

        let ui_rectangle: Rect = ui.min_rect();

        let left_most_side = ui_rectangle.left();
        let top_most_side = ui_rectangle.top();

        // this part is vibe coded
        // Fixed drawing area: 1600 x 1600 pixels
        const SIZE: f32 = 1600.0;
        const COLS: usize = 500;
        const ROWS: usize = 500;

        // Reserve exactly 1600x1600 px in the UI (won't resize with the panel)
        let (rect, _response) = ui.allocate_exact_size(egui::vec2(SIZE, SIZE), egui::Sense::hover());
        let painter = ui.painter_at(rect);

        // Grid cell size (fixed, independent of the UI rectangle size)
        let dx = SIZE / COLS as f32; // 3.2 px
        let dy = SIZE / ROWS as f32; // 3.2 px
        let radius = 0.45 * dx.min(dy); // ~1.44 px

        let origin = rect.min; // top-left of the allocated 1600x1600 area

        // let me obtain the four vectors of nuclides 
        

        for row in 0..ROWS {
            for col in 0..COLS {
                // Center each circle in its cell
                let x = origin.x + (col as f32 + 0.5) * dx;
                let y = origin.y + (row as f32 + 0.5) * dy;
                let center = egui::pos2(x, y);

                // Example color gradient by position (any palette can be used)
                let r = (col * 255 / (COLS - 1)) as u8;
                let g = (row * 255 / (ROWS - 1)) as u8;
                let b = 160u8;
                let color = egui::Color32::from_rgb(r, g, b);

                painter.circle_filled(center, radius, color);
            }
        }

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
