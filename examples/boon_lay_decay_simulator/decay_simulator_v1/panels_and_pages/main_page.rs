use egui::{Rect, Ui};

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
}
