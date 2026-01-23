use boon_lay::Nuclide;
use egui::{Align2, Color32, FontId, Pos2, Rect, Rounding, Stroke, Ui, Vec2, Widget};

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {
    /// displays a periodic table of elements
    pub fn periodic_table(&mut self, ui: &mut Ui) {

        let hydrogen = Nuclide::H1;
        Self::ui_element_box_at_position(ui, hydrogen, 100.0, 100.0);
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

    /// Example usage: call this in your egui UI code (e.g., inside `eframe::App::update`)
    fn ui_element_box_at_position(ui: &mut egui::Ui, nuclide: Nuclide,
        x_pixel: f32,
        y_pixel: f32) {
        // Choose a box size; adjust as needed
        let size = Vec2::new(80.0, 80.0);
        let pos = Pos2::new(x_pixel, y_pixel);
        let (z,_a) = nuclide.get_z_a();
        let element_string: &str = Self::symbol_from_z(z);
        Self::draw_element_box_at(ui, nuclide, element_string, pos, size);
    }

    /// Draw a single element box at a specific UI coordinate.
    /// - `pos`: top-left corner in the current `ui` coordinate space
    /// - `size`: width/height of the box
    /// - `symbol`: element symbol string (e.g., "H")
    /// - `nuclide`: your crate’s nuclide for computing color and Z
    pub fn draw_element_box_at(
        ui: &mut egui::Ui,
        nuclide: Nuclide,
        symbol: &str,
        pos: Pos2,
        size: Vec2,
    ) {

        let centre_x_pixels = pos.x;
        let centre_y_pixels = pos.y;
        let x_width_pixels = size.x;
        let y_width_pixels = size.y;
        let element_box: ElementBox = nuclide.clone().into();
        Self::put_widget_with_size_and_centre(ui, 
            element_box, 
            centre_x_pixels, 
            centre_y_pixels, 
            x_width_pixels, 
            y_width_pixels
        );

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
    pub fn put_widget_with_size_and_centre(ui: &mut Ui, widget: impl Widget,
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

        ui.put(rect, widget);

    }


}

pub struct ElementBox {
    pub nuclide: Nuclide,
}

impl From<Nuclide> for ElementBox {
    fn from(nuclide: Nuclide) -> Self {
        ElementBox { nuclide }
    }
}

impl Widget for ElementBox {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Allocate the exact size so the widget participates in layout and interaction
        let size = Vec2::new(80.0, 80.0);
        let (mut response, painter) = ui.allocate_painter(
            size, egui::Sense::hover()
        );


        let (z, _a) = self.nuclide.get_z_a();
        let symbol = DecaySimApp::symbol_from_z(z);
        let fill = DecaySimApp::element_color(self.nuclide);
        let text_color = DecaySimApp::contrasting_text(fill);

        let pos = Pos2::new(0.0, 0.0);
        let rect = Rect::from_min_size(pos, size);

        // Draw filled rounded rect and stroke
        let rounding = Rounding::same(8.0);
        painter.rect_filled(rect, rounding, fill);
        painter.rect_stroke(rect, rounding, Stroke::new(2.0, Color32::BLACK));

        // Text settings
        let font32 = FontId::proportional(32.0);
        let pad = 6.0;

        // Atomic number (top-left, padded)
        let num_pos = Pos2::new(rect.min.x + pad, rect.min.y + pad);
        painter.text(num_pos, Align2::LEFT_TOP, format!("{}", z), font32.clone(), text_color);

        // Symbol centered
        painter.text(rect.center(), Align2::CENTER_CENTER, symbol, font32, text_color);
        // Create a child UI scoped to this rect where we’ll place standard widgets

        // Provide hover tooltip or interactions if desired
        response = response.on_hover_text(format!("{} (Z={})", symbol, self.nuclide.get_z_a().0));
        response
    }
}
