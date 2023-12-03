use crate::psim::simulator::forcefield::{ForceField, ForceType, Shape};
use crate::psim::simulator::vector2::Vector2;
use crate::psim::visualizer::Visualizer;

mod psim;

fn main() {
    let mut visualizer = Visualizer::new(1000,1000);
    // add walls
    visualizer.add_force_field(
        ForceField::new(
            Vector2::new(10.0,500.0),
            Shape::Rectangle {width:20.0,height:1000.0},
            ForceType::Force {force:Vector2::new(4000.0,0.0)}
        )
    );
    visualizer.add_force_field(
        ForceField::new(
            Vector2::new(990.0,500.0),
            Shape::Rectangle {width:20.0,height:1000.0},
            ForceType::Force {force:Vector2::new(-4000.0,0.0)}
        )
    );
    visualizer.add_force_field(
        ForceField::new(
            Vector2::new(500.0,10.0),
            Shape::Rectangle {width:1000.0,height:20.0},
            ForceType::Force {force:Vector2::new(0.0,4000.0)}
        )
    );
    visualizer.add_force_field(
        ForceField::new(
            Vector2::new(500.0,990.0),
            Shape::Rectangle {width:1000.0,height:20.0},
            ForceType::Force {force:Vector2::new(0.0,-4000.0)}
        )
    );

    // visualizer.run();
    visualizer.run_constant_time(0.000000000005);
}

