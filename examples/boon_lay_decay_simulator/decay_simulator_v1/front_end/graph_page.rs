use egui::Ui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use uom::si::{f64::Time, time::second};

use crate::decay_simulator_v1::DecaySimApp;

impl DecaySimApp {

    pub fn graph_page(&self, ui: &mut Ui){


        let simulator_state_clone = 
            self.simulator_state.lock().unwrap().clone();

        let nuclides_to_plot = simulator_state_clone.get_nuclides_to_plot();
        let nuclide_fractions_over_time: Vec<(Time, Vec<f64>)> = 
            simulator_state_clone.get_nuclides_fractions_over_time();

        let mut nuclide_plot = Plot::new("Nuclide Fractions over time").legend(Legend::default());

        // sets the aspect for plot 
        nuclide_plot = nuclide_plot.width(1800.0);
        nuclide_plot = nuclide_plot.view_aspect(16.0/9.0);

        nuclide_plot = nuclide_plot.x_axis_label(
            "time (seconds), current time (seconds): ".to_owned() 
        );
        nuclide_plot = nuclide_plot.y_axis_label(
            "nuclide fractions".to_owned());


        // let's make the time and bt11 vector

        ui.heading("Nuclide Fractions over Time");
        nuclide_plot.show(ui, |plot_ui| {
            // outer loop, ie do for every nuclide
            for (nuclide_index,nuclide) in nuclides_to_plot.iter().enumerate() {
                let nuclide_name: String = format!("{:?}",nuclide);

                let mut plot_vector_time_and_nuclide_fraction: Vec<[f64;2]>
                    = vec![];

                // we construct the plot vector for every nuclide
                for (_time_index, (simulation_time, nuclide_fractions_vector)) in 
                    nuclide_fractions_over_time.iter().enumerate() {

                        let time_seconds = simulation_time.get::<second>();
                        let nuclide_fraction = nuclide_fractions_vector[nuclide_index];

                        plot_vector_time_and_nuclide_fraction. 
                            push([time_seconds,nuclide_fraction]);


                    }


                plot_ui.line(Line::new(PlotPoints::from(
                            plot_vector_time_and_nuclide_fraction.clone()
                )).name(nuclide_name + " fraction"));

            }

            //
        });



        ui.separator();


    }
}
