use crate::psim::visualizer::Visualizer;

mod psim;

fn main() {
    let mut visualizer = Visualizer::new(1000,1000);
    // visualizer.run();
    visualizer.run_constant_time(0.000000000005);
}

