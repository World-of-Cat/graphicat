extern crate glfw;

use glfw::{Action, ClientApiHint, Key, Modifiers, WindowHint, WindowMode};
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, KeyEvent};
use winit::keyboard::{ModifiersKeyState, NamedKey, PhysicalKey};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use graphicat::gpu::{GpuSelectionParameters, PhysicalDevice};
use graphicat::instance::{Instance, RwhExtensionProvider};
use graphicat::surface::Surface;

fn glfw_main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    let instance = unsafe { Instance::new(&glfw) }.unwrap();
    let physical_device = PhysicalDevice::select(instance.clone(), GpuSelectionParameters::default()).unwrap();

    glfw.window_hint(WindowHint::ClientApi(ClientApiHint::NoApi));
    glfw.window_hint(WindowHint::Resizable(false));
    let (mut window, events) = glfw.create_window(800, 600, "Hello!", WindowMode::Windowed).unwrap();

    let surface = Surface::new_glfw(instance.as_ref(), &window).unwrap();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, Modifiers::Control) => window.set_should_close(true),
                _ => (),
            }
        }
    }

    println!("GPU: {}", physical_device.name());
    println!("GPU Type: {:?}", physical_device.device_type());

    println!("Required Extensions: {:?}", graphicat::gpu::required_device_extensions());
}

fn winit_main() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_inner_size(PhysicalSize::new(800, 600))
        .with_resizable(false)
        .build(&event_loop).unwrap();

    let instance = unsafe { Instance::new(&RwhExtensionProvider::new(&window)) }.unwrap();
    let physical_device = PhysicalDevice::select(instance.clone(), GpuSelectionParameters::default()).unwrap();

    let surface = Surface::new(instance.as_ref(), &window).unwrap();

    let mut modifiers = winit::event::Modifiers::default();

    let _ = event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent {
                window_id, event
            } if window_id == window.id() => {
                match event {
                    winit::event::WindowEvent::ActivationTokenDone { .. } => {}
                    winit::event::WindowEvent::Resized(_) => {}
                    winit::event::WindowEvent::Moved(_) => {}
                    winit::event::WindowEvent::CloseRequested => elwt.exit(),
                    winit::event::WindowEvent::Destroyed => {}
                    winit::event::WindowEvent::DroppedFile(_) => {}
                    winit::event::WindowEvent::HoveredFile(_) => {}
                    winit::event::WindowEvent::HoveredFileCancelled => {}
                    winit::event::WindowEvent::Focused(_) => {}
                    winit::event::WindowEvent::Ime(_) => {}
                    winit::event::WindowEvent::CursorMoved { .. } => {}
                    winit::event::WindowEvent::CursorEntered { .. } => {}
                    winit::event::WindowEvent::CursorLeft { .. } => {}
                    winit::event::WindowEvent::MouseWheel { .. } => {}
                    winit::event::WindowEvent::MouseInput { .. } => {}
                    winit::event::WindowEvent::TouchpadMagnify { .. } => {}
                    winit::event::WindowEvent::SmartMagnify { .. } => {}
                    winit::event::WindowEvent::TouchpadRotate { .. } => {}
                    winit::event::WindowEvent::TouchpadPressure { .. } => {}
                    winit::event::WindowEvent::AxisMotion { .. } => {}
                    winit::event::WindowEvent::Touch(_) => {}
                    winit::event::WindowEvent::ScaleFactorChanged { .. } => {}
                    winit::event::WindowEvent::ThemeChanged(_) => {}
                    winit::event::WindowEvent::Occluded(_) => {}
                    winit::event::WindowEvent::RedrawRequested => {}
                    winit::event::WindowEvent::KeyboardInput { .. } => {}
                    winit::event::WindowEvent::ModifiersChanged(_) => {}
                }
            },
            Event::AboutToWait => window.request_redraw(),
            _ => (),
        }
    });

}

fn main() {
    glfw_main();
}

