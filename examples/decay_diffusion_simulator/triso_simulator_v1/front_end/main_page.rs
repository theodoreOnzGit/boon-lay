use boon_lay::Nuclide;
use boon_lay::prelude::SingleNuclideSimulatorMC;
use boon_lay::prelude::decay_library::DecayLibrary;
use egui::{Color32, Rect, Ui};
use oorandom::Rand64;

use crate::triso_simulator_v1::{front_end::triso_particle::TrisoParticleUi, TRISOSimApp};

impl TRISOSimApp {

    pub fn main_page(&mut self, ui: &mut Ui) {

        let ui_rectangle: Rect = ui.min_rect();
        let viewport = ui.clip_rect();

        let left_most_side = ui_rectangle.left();
        let top_most_side = ui_rectangle.top();

        // this part is vibe coded
        const XSIZE: f32 = 1600.0;
        const YSIZE: f32 = 1000.0;
        const COLS: usize = 500;
        const ROWS: usize = 500;

        // Reserve exactly 2500x2500 px in the UI (won't resize with the panel)
        let (rect, _response) = ui.allocate_exact_size(
            egui::vec2(XSIZE, YSIZE), 
            egui::Sense::hover()
        );

        // Grid cell size (fixed, independent of the UI rectangle size)
        let dx = XSIZE / COLS as f32; // 3.2 px
        let dy = XSIZE / ROWS as f32; // 3.2 px
        let _radius = 0.45 * dx.min(dy); // ~1.44 px
        let _origin = rect.min; // top-left of the allocated 2500x2500 area

        // let me obtain the four vectors of nuclides 
        
        // Convert a Vec<SingleNuclideSimualtorMC> into Vec<Nuclide>
        // using the provided getter.



        // collect all nuclides and then display them
        let (nuclide_sim_vec_1, _decay_library): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_1_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_2,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_2_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_3,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_3_ptr.lock().unwrap().clone();
        let (nuclide_sim_vec_4,_): (Vec<SingleNuclideSimulatorMC>, DecayLibrary) = 
            self.decay_sim_thread_4_ptr.lock().unwrap().clone();

        // had chatGPT assistance
        let mut nuclide_sim_full_vec: Vec<SingleNuclideSimulatorMC> = 
            nuclide_sim_vec_1.into_iter()
            .chain(nuclide_sim_vec_2)
            .chain(nuclide_sim_vec_3)
            .chain(nuclide_sim_vec_4)
            .collect();

        // with this full vector, I want to sample about 20000 for plotting 
        // had chatGPT assistance

        // from chatgpt 5
        // Fisher–Yates shuffle using oorandom

        fn uniform_u64(rng: &mut oorandom::Rand64, bound: u64) -> u64 {
            // Rejection sampling to avoid modulo bias
            // Returns value in [0, bound)
            assert!(bound > 0);
            let zone = u64::MAX - (u64::MAX % bound);
            loop {
                let x = rng.rand_u64();
                if x < zone {
                    return x % bound;
                }
            }
        }

        fn uniform_usize(rng: &mut oorandom::Rand64, bound: usize) -> usize {
            uniform_u64(rng, bound as u64) as usize
        }


        fn shuffle_in_place<T>(v: &mut [T], seed: u64) {
            let len = v.len();
            if len <= 1 { return; }
            let mut rng = Rand64::new(seed.into());
            for i in (1..len).rev() {
                // pick j in [0, i]
                let j = uniform_usize(&mut rng, i + 1);
                v.swap(i, j);
            }
        }

        // Moves k random elements out, leaving the rest in `v`
        fn take_random_without_replacement<T>(v: &mut Vec<T>, k: usize, seed: u64) -> Vec<T> {
            if v.is_empty() || k == 0 {
                return Vec::new();
            }
            let k = k.min(v.len());
            shuffle_in_place(v, seed);
            v.split_off(v.len() - k)
        }

        let seed = 20_999_u64;

        let sampled_particles_for_plotting: Vec<SingleNuclideSimulatorMC> = 
            take_random_without_replacement(
                &mut nuclide_sim_full_vec, 
                2000, 
                seed
            );

        // this part deals with the triso particle
        // and then painting the radionuclide

        {
            // later on, i want to have a width changing the triso particle 
            // but maybe layer

            let mut triso_picture = TrisoParticleUi::default();


            let content_origin_rect: Rect = ui.min_rect();
            let left_limit = left_most_side;
            let top_limit = top_most_side;

            let right_limit = left_limit + content_origin_rect.right();
            let bottom_limit = top_limit + content_origin_rect.bottom();


            let triso_width = 1.0 * (viewport.bottom() - viewport.top());

            let triso_centre_x = left_limit + right_limit/3.0;
            let triso_centre_y = top_limit + bottom_limit/2.0;

            triso_picture.put_self_with_size_and_centre(ui, 
                triso_centre_x, 
                triso_centre_y, 
                triso_width, 
                triso_width,
            );
            triso_picture.put_particle_vector_with_size_and_centre(ui, 
                triso_centre_x, 
                triso_centre_y, 
                triso_width, 
                sampled_particles_for_plotting,
            );
        }



    }


    /// this is a vibe coded colour scheme for elements in the periodic table
    /// Vibe-coded colour scheme for elements, with heavier elements shaded darker.
    /// Darkness scales smoothly with atomic number Z: lightest at H (Z=1), darkest at Og (Z=118).
    pub fn element_color(nuclide: Nuclide) -> Color32 {
        use egui::Color32;

        // Base palette by category
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

        let (z, _a) = nuclide.get_z_a();

        // Pick base color by category
        let base = match z {
            1 => HYDROGEN,                                 // Special case

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

            _ => UNKNOWN,
        };

        // Apply darkness based on atomic number:
        // factor = 1.0 at Z=1 (no darkening) down to `min_factor` at Z=118.
        // Tune `min_factor` to control max darkening.
        let min_factor = 0.01_f32; // darkest multiplier for the heaviest elements
        let z_clamped = z.clamp(1, 118);
        let t = (z_clamped - 1) as f32 / (118 - 1) as f32; // 0.0..1.0
        let factor = 1.0 - t * (1.0 - min_factor);

        // For hydrogen, keep pure white (optional: you can remove this if you want it shaded too)
        if z == 1 {
            base
        } else {
            base.gamma_multiply(factor)
        }
    }
}



