use graphicat::winit;
use graphicat::winit::dpi::PhysicalSize;
use graphicat::winit::event_loop::EventLoop;
use graphicat::winit::window::WindowBuilder;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop).unwrap();


}
