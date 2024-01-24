mod camera;
mod error;
mod image;
mod materials;
pub mod math;
mod ray;
mod world;

pub use self::image::*;
pub use camera::*;
pub use error::*;
pub use materials::*;
pub use ray::*;
pub use world::*;





use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        },
        _ => {}
    });
}









fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

#[cfg(test)]
mod tests {
    use crate::clamp;

    #[test]
    fn test_clamp() {
        use rand::Rng;

        let mut rng = rand::thread_rng();

        let min: f64 = rng.gen_range(-100.0..50.0);
        let max: f64 = rng.gen_range(min + 1.0..100.0);

        let x: f64 = rng.gen_range(min + 0.25..max - 0.25);
        let result = clamp(x, min, max);
        assert_eq!(result, x);

        let x: f64 = rng.gen_range(max..1000.0);
        let result = clamp(x, min, max);
        assert_eq!(result, max);

        let x: f64 = rng.gen_range(-1000.0..min);
        let result = clamp(x, min, max);
        assert_eq!(result, min);
    }
}
