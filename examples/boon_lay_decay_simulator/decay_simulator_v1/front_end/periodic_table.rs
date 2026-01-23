use boon_lay::Nuclide;
use egui::{Align2, Color32, FontId, Rounding, Stroke, Ui, Vec2};

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {
    /// displays a periodic table of elements
    pub fn periodic_table(&mut self, ui: &mut Ui) {

        let hydrogen = Nuclide::H1;
        Self::ui_element_box(ui, hydrogen, 0.0, 0.0);
    }

    /// Simple contrast helper: choose black or white text based on fill.
    fn contrasting_text(fill: Color32) -> Color32 {
        let r = fill.r() as f32 / 255.0;
        let g = fill.g() as f32 / 255.0;
        let b = fill.b() as f32 / 255.0;
        let luminance = 0.2126 * r + 0.7152 * g + 0.0722 * b;
        if luminance > 0.6 { Color32::BLACK } else { Color32::WHITE }
    }

    /// Draw a single element box (rounded rectangle) with Z and symbol at font size 32.
    fn draw_element_box(ui: &mut egui::Ui, nuclide: Nuclide, symbol: &str, size: Vec2) {
        let fill = Self::element_color(nuclide);
        
        let (z, _a) = nuclide.get_z_a();
        let text_color = Self::contrasting_text(fill);

        // Allocate space for the box
        let (rect, _resp) = ui.allocate_exact_size(size, egui::Sense::hover());
        let painter = ui.painter();

        // Draw rounded filled rect with an outline
        let rounding = Rounding::same(8.0);
        painter.rect_filled(rect, rounding, fill);
        painter.rect_stroke(rect, rounding, Stroke::new(2.0, Color32::BLACK));

        // Text settings
        let font32 = FontId::proportional(32.0);

        // Draw atomic number (top-left, with a bit of padding)
        let pad = 6.0;
        let num_pos = egui::pos2(rect.min.x + pad, rect.min.y + pad);
        painter.text(num_pos, Align2::LEFT_TOP, format!("{}", z), font32.clone(), text_color);

        // Draw symbol centered
        let center = rect.center();
        painter.text(center, Align2::CENTER_CENTER, symbol, font32, text_color);
    }

    /// Example usage: call this in your egui UI code (e.g., inside `eframe::App::update`)
    fn ui_element_box(ui: &mut egui::Ui, nuclide: Nuclide,
        x_pixel: f32,
        y_pixel: f32) {
        // Choose a box size; adjust as needed
        let size = Vec2::new(80.0, 80.0);
        let (z,_a) = nuclide.get_z_a();
        let element_string: &str = Self::symbol_from_z(z);
        Self::draw_element_box(ui, nuclide, element_string, size);
    }

    
    /// Convert atomic number (Z) to element symbol.
    /// Covers Z = 1..=118; returns "?" for out of range.
    pub fn symbol_from_z(z: u32) -> &'static str {
        // index = Z; index 0 is unused
        static SYMBOLS: [&str; 119] = [
            "",  // 0
            "H","He","Li","Be","B","C","N","O","F","Ne",
            "Na","Mg","Al","Si","P","S","Cl","Ar",
            "K","Ca","Sc","Ti","V","Cr","Mn","Fe","Co","Ni","Cu","Zn",
            "Ga","Ge","As","Se","Br","Kr",
            "Rb","Sr","Y","Zr","Nb","Mo","Tc","Ru","Rh","Pd","Ag","Cd",
            "In","Sn","Sb","Te","I","Xe",
            "Cs","Ba","La","Ce","Pr","Nd","Pm","Sm","Eu","Gd","Tb","Dy","Ho","Er","Tm","Yb","Lu",
            "Hf","Ta","W","Re","Os","Ir","Pt","Au","Hg",
            "Tl","Pb","Bi","Po","At","Rn",
            "Fr","Ra","Ac","Th","Pa","U","Np","Pu","Am","Cm","Bk","Cf","Es","Fm","Md","No","Lr",
            "Rf","Db","Sg","Bh","Hs","Mt","Ds","Rg","Cn",
            "Nh","Fl","Mc","Lv","Ts","Og",
        ];
        if z <= 118 { SYMBOLS[z as usize] } else { "?" }
    }

}
