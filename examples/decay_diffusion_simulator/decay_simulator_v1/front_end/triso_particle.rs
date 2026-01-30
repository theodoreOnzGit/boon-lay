
use eframe::{egui, egui::{Color32, Pos2, Stroke, Widget}};
use egui::{Rect, Ui};


#[derive(Clone,Copy, Debug)]
pub struct TrisoParticle {
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
}

impl Default for TrisoParticle {
    fn default() -> Self {
        Self {
            diameter_mm: 1.0,          // 1 mm (metadata)
            ui_diameter_ratio: 0.8,   // occupy ~80% of the UI
            num_rings: 18,             // adjust to taste
            stroke: Stroke { width: 6.0, color: Color32::WHITE },
            color: Color32::WHITE,
        }
    }
}

// Implement the Widget trait so you can `ui.add(triso.clone())`
impl Widget for TrisoParticle {
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


impl TrisoParticle {

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
    pub fn put_self_with_size_and_centre(
        &self,
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
}
