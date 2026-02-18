/// the boon lay decay simulator 
///
/// this is powered by egui
///
/// 
///
fn main() {

    println!("Starting Boon Lay Diffusion and Decay Simulator...");
    triso_simulator_v1::triso_decay_diffusion_simulator_v1().unwrap();
}

pub mod triso_simulator_v1;
