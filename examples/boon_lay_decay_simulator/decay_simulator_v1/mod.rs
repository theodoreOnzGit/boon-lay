use std::time::SystemTime;
use std::{sync::Arc, thread, time::Duration};

use std::sync::{Barrier, Mutex};

use boon_lay::prelude::decay_library::DecayLibrary;
use boon_lay::prelude::SingleNuclideSimulatorMC;
use boon_lay::Nuclide;

use crate::decay_simulator_v1::backend::simulator_state::SimulatorState;
use crate::decay_simulator_v1::panels_and_pages::Panel;

pub fn decay_simulator_v1() -> eframe::Result<()> {


    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Decay Simulator v1 Powered by Boon Lay",
        native_options,
        Box::new(|cc| {
            // image support,
            // from 
            // https://github.com/emilk/egui/tree/master/examples/images
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(DecaySimApp::new(cc)))

    }

        ),
    )
}
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DecaySimApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f64,

    open_panel: Panel,

    /// these are pointers for the each decay simulation
    #[serde(skip)]
    decay_sim_thread_1_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>,DecayLibrary)>>,
    /// these are pointers for the each decay simulation
    #[serde(skip)]
    decay_sim_thread_2_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>,DecayLibrary)>>,
    /// these are pointers for the each decay simulation
    #[serde(skip)]
    decay_sim_thread_3_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>,DecayLibrary)>>,
    /// these are pointers for the each decay simulation
    #[serde(skip)]
    decay_sim_thread_4_ptr: Arc<Mutex<(Vec<SingleNuclideSimulatorMC>,DecayLibrary)>>,

    // we also need plot data here 
    // this is for the data to be transferred between threads
    #[serde(skip)]
    simulator_state: Arc<Mutex<SimulatorState>>,

    //// this is for direct use in plots 
    //#[serde(skip)]
    //ciet_plot_data: PagePlotData,

    //#[serde(skip)]
    //frequency_response_settings: FreqResponseAndTransientSettings,

    // checks whether user wants fast fwd or slow motion
    user_wants_fast_fwd_on: bool,
    // checks whether user wants fast fwd or slow motion
    user_wants_slow_motion_on: bool,

    //// for the user to select the heater type desierd
    //#[serde(skip)]
    //user_desired_heater_type: HeaterType,

}

impl DecaySimApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        //// Load previous app state (if any).
        //// Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        let new_decay_sim_app: DecaySimApp = Default::default();

        //// I'll clone the pointer and start a thread 

        //let ciet_state_ptr: Arc<Mutex<CIETState>> = 
        //    new_ciet_app.ciet_state.clone();

        //// this is the current state of ciet for plotting
        //// like the instantaneous temperature and such
        //let ciet_state_ptr_for_plotting: Arc<Mutex<CIETState>> = 
        //    new_ciet_app.ciet_state.clone();
        //// for data recording, 
        //// I'll also clone the pointer and start a thread
        //// this contains arrays with historical data
        //let ciet_plot_ptr: Arc<Mutex<PagePlotData>> = 
        //    new_ciet_app.ciet_plot_data_mutex_ptr_for_parallel_data_transfer.clone();

        let decay_sim_thread_1_ptr = 
            new_decay_sim_app.decay_sim_thread_1_ptr.clone();
        let decay_sim_thread_2_ptr = 
            new_decay_sim_app.decay_sim_thread_2_ptr.clone();
        let decay_sim_thread_3_ptr = 
            new_decay_sim_app.decay_sim_thread_3_ptr.clone();
        let decay_sim_thread_4_ptr = 
            new_decay_sim_app.decay_sim_thread_4_ptr.clone();

        let simulator_state_thread_1_ptr: Arc<Mutex<SimulatorState>> = 
            new_decay_sim_app.simulator_state.clone();
        let simulator_state_thread_2_ptr: Arc<Mutex<SimulatorState>> = 
            new_decay_sim_app.simulator_state.clone();
        let simulator_state_thread_3_ptr: Arc<Mutex<SimulatorState>> = 
            new_decay_sim_app.simulator_state.clone();
        let simulator_state_thread_4_ptr: Arc<Mutex<SimulatorState>> = 
            new_decay_sim_app.simulator_state.clone();

        // the barrier keeps each thread running in sync, so that 
        // no one thread outruns another
        let num_threads = 4;
        let barrier: Arc<Barrier> = Arc::new(Barrier::new(num_threads));
        let barrier_1 = Arc::clone(&barrier);
        let barrier_2 = Arc::clone(&barrier);
        let barrier_3 = Arc::clone(&barrier);
        let barrier_4 = Arc::clone(&barrier);

        //// now spawn a thread moving in the pointer 
        //
        thread::spawn(move ||{
            let thread_number = 1;
            Self::run_decay_chain_simulation(
                decay_sim_thread_1_ptr,
                simulator_state_thread_1_ptr,
                thread_number,
                barrier_1,
            );
        });
        thread::spawn(move ||{
            let thread_number = 2;
            Self::run_decay_chain_simulation(
                decay_sim_thread_2_ptr,
                simulator_state_thread_2_ptr,
                thread_number,
                barrier_2,
            );
        });
        thread::spawn(move ||{
            let thread_number = 3;
            Self::run_decay_chain_simulation(
                decay_sim_thread_3_ptr,
                simulator_state_thread_3_ptr,
                thread_number,
                barrier_3,
            );
        });
        thread::spawn(move ||{
            let thread_number = 4;
            Self::run_decay_chain_simulation(
                decay_sim_thread_4_ptr,
                simulator_state_thread_4_ptr,
                thread_number,
                barrier_4,
            );
        });

        // spawn a thread to update the plotting bits
        thread::spawn(move ||{
            //update_ciet_plot_from_ciet_state(
            //    ciet_state_ptr_for_plotting, 
            //    ciet_plot_ptr);
        });

        new_decay_sim_app
    }

    
}

impl Default for DecaySimApp {
    fn default() -> Self {

        let time_start = SystemTime::now();

        let num_of_nuclides = 62_500;
        let nuclide = Nuclide::U238;

        let rng_seed_1 = 550;
        let rng_seed_2 = 47;
        let rng_seed_3 = 58;
        let rng_seed_4 = 1414;
        let decay_sim_thread_1_ptr = 
            Self::construct_new_single_thread_multi_particle_simulation(
                num_of_nuclides, 
                nuclide,
                rng_seed_1,
            );
        let decay_sim_thread_2_ptr = 
            Self::construct_new_single_thread_multi_particle_simulation(
                num_of_nuclides, 
                nuclide,
                rng_seed_2,
            );
        let decay_sim_thread_3_ptr = 
            Self::construct_new_single_thread_multi_particle_simulation(
                num_of_nuclides, 
                nuclide,
                rng_seed_3,
            );
        let decay_sim_thread_4_ptr = 
            Self::construct_new_single_thread_multi_particle_simulation(
                num_of_nuclides, 
                nuclide,
                rng_seed_4,
            );

        let simulator_state = Arc::new(Mutex::new(SimulatorState::default()));

        let initiation_time_secs = time_start.elapsed().unwrap().as_secs();

        dbg!(&initiation_time_secs);

        Self {
            // Example stuff:
            label: "Boon Lay Decay Simulator v1".to_owned(),
            value: 3.6,
            open_panel: Panel::MainPage,
            //ciet_state,
            //ciet_plot_data_mutex_ptr_for_parallel_data_transfer: ciet_plot_data,
            //ciet_plot_data: PagePlotData::default(),
            //frequency_response_settings: FreqResponseAndTransientSettings::default(),
            user_wants_fast_fwd_on: false,
            user_wants_slow_motion_on: false,
            decay_sim_thread_1_ptr,
            decay_sim_thread_2_ptr,
            decay_sim_thread_3_ptr,
            decay_sim_thread_4_ptr,
            simulator_state,
            //user_desired_heater_type: HeaterType::InsulatedHeaterV1Fine15Mesh,

        }
    }
}



impl eframe::App for DecaySimApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {

                egui::widgets::global_theme_preference_buttons(ui);
            });


            ui.heading("Boon Lay Decay Simulator v1");
            ui.separator();
            // allow user to select which panel is open
            ui.horizontal( 
                |ui| {
                    ui.selectable_value(&mut self.open_panel, Panel::MainPage, "Main Page"); 
                    ui.selectable_value(&mut self.open_panel, Panel::GraphPage, "Graph Page"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::Heater, "Heater"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::CTAH, "CTAH"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::CTAHPump, "CTAH Pump"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::TCHX, "TCHX"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::DHX, "DHX STHE"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::FrequencyResponseAndTransients, "Frequency Response and Transients"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::OnlineCalibration, "Online Calibration"); 
                    //ui.selectable_value(&mut self.open_panel, Panel::NodalisedDiagram, "CIET Nodalised Diagram"); 
            }
            );
            ui.separator();
        });

        egui::SidePanel::right("Supplementary Info").show(ctx, |ui|{
            match self.open_panel {
                Panel::MainPage => {
                    egui::ScrollArea::both().show(ui, |ui| {
                        self.side_panel(ui);
                        self.citation_disclaimer_and_acknowledgements(ui);
                    });
                },
                Panel::GraphPage => {
                    egui::ScrollArea::both().show(ui, |ui| {
                        //self.ciet_main_page_side_panel(ui);
                        self.citation_disclaimer_and_acknowledgements(ui);
                    });
                },
            //    Panel::CTAHPump => {
            //        self.ciet_sim_ctah_pump_page_csv(ui);
            //    },
            //    Panel::CTAH => {
            //        self.ciet_sim_ctah_page_csv(ui);
            //    },
            //    Panel::Heater => {
            //        
            //        // display csv file on side panel when heater page 
            //        // is open
            //        self.ciet_sim_heater_page_csv(ui);
            //    },
            //    Panel::DHX => {
            //        self.ciet_sim_dhx_page_csv(ui);
            //    },
            //    Panel::TCHX => {
            //        self.ciet_sim_tchx_page_csv(ui);
            //    },
            //    Panel::FrequencyResponseAndTransients => {

            //        self.ciet_sim_heater_page_csv(ui);
            //    },
            //    Panel::NodalisedDiagram => {},
            //    Panel::OnlineCalibration => {},

            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's





            // show correct panel or page based on user selection

            match self.open_panel {
                Panel::MainPage => {
                    self.main_page(ui);
                },
                Panel::GraphPage => {
                    // nothing yet
                },
            }

            //match self.open_panel {
            //    Panel::FrequencyResponseAndTransients => {
            //        // enables scrolling within the image
            //        //egui::ScrollArea::both().show(ui, |ui| {
            //        //    ui.image(egui::include_image!("ciet_gui_schematics.png"));
            //        //});
            //        self.ciet_sim_transients_and_freq_response_page(ui);

            //        
            //    },
            //    Panel::MainPage => {
            //        self.ciet_sim_main_page_central_panel(ui);

            //    },
            //    Panel::CTAHPump => {
            //        self.ciet_sim_ctah_pump_page_and_graphs(ui);
            //    },
            //    Panel::CTAH => {
            //        self.ciet_sim_ctah_page_graph(ui);
            //    },
            //    Panel::Heater => {
            //        self.ciet_sim_heater_page_graph(ui);
            //    },
            //    Panel::DHX => {
            //        self.ciet_sim_dhx_branch_page_graph(ui);
            //    },
            //    Panel::TCHX => {
            //        self.ciet_sim_tchx_page_graph(ui);
            //    },
            //    Panel::NodalisedDiagram => {
            //        // enables scrolling within the image
            //        egui::ScrollArea::both().show(ui, |ui| {
            //            ui.image(egui::include_image!("ciet_sam_diagram_replica.jpg"));
            //        });
            //    },
            //    Panel::OnlineCalibration => {
            //        self.ciet_sim_online_calibration_page(ui);

            //    },

            //}

            ui.add(egui::github_link_file!(
                    "https://github.com/theodoreOnzGit/boon-lay/blob/develop/",
                    "Boon Lay Github Repo"
            ));




        });

        egui::TopBottomPanel::bottom("github").show(ctx, |ui|{

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });

        });

        

        //// frequency response should only switch on IF 
        //// both advanced heater control and frequency response are 
        //// switched on
        //if self.frequency_response_settings.advanced_heater_control_switched_on
        //    && self.frequency_response_settings.frequency_response_switched_on {

        //        // frequency response controls
        //        // first get current state
        //        let mut ciet_state_local: CIETState 
        //            = self.ciet_state.lock().unwrap().clone();
        //        let current_sim_time = 
        //            Time::new::<second>(
        //                ciet_state_local.simulation_time_seconds
        //            );

        //        let total_heater_power_kw = 
        //            self.frequency_response_settings
        //            .get_frequency_response_signal(current_sim_time)
        //            .get::<kilowatt>();
        //        ciet_state_local.heater_power_kilowatts = 
        //            total_heater_power_kw;
        //        // update frequency response back into state 
        //        self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
        //} else if self.frequency_response_settings.advanced_heater_control_switched_on
        //    && !self.frequency_response_settings.frequency_response_switched_on {
        //        // if advanced heater control is switched on and 
        //        // frequency response off, only take steady 
        //        // state power

        //        // frequency response controls
        //        // first get current state
        //        let mut ciet_state_local: CIETState 
        //            = self.ciet_state.lock().unwrap().clone();

        //        let total_heater_power_kw = 
        //            self.frequency_response_settings
        //            .get_steady_state_power_signal()
        //            .get::<kilowatt>();

        //        ciet_state_local.heater_power_kilowatts = 
        //            total_heater_power_kw;
        //        // update frequency response back into state 
        //        self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
        //}

        //// now for calibration functions, eg. heater type 

        //{
        //    let user_desired_heater_type: HeaterType = self.user_desired_heater_type;

        //    let mut ciet_state_local: CIETState 
        //        = self.ciet_state.lock().unwrap().clone();
        //    ciet_state_local.current_heater_type = user_desired_heater_type;
        //    self.ciet_state.lock().unwrap().overwrite_state(ciet_state_local);
        //}

        // request update every 0.1 s 

        ctx.request_repaint_after(Duration::from_millis(50));

        // adding the return here because there are too many closing 
        // parantheses
        // just demarcates the end
        return ();
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}



/// code for panels and pages in the simulator
/// this is basically like the frontend
pub mod panels_and_pages;


/// this is the backend code 
pub mod backend;
