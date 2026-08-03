use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run_ui() {
    let event_loop = EventLoop::new();

    // Crear la ventana principal simulando una pantalla Mac de resolución 640x480
    let window = WindowBuilder::new()
        .with_title("DesktopHLE - UITest (Beta)")
        .with_inner_size(LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    // Configurar el buffer de píxeles
    let mut pixels = Pixels::new(640, 480, surface_texture).unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            Event::MainEventsCleared => {
                // Dibujar un fondo básico en el framebuffer de píxeles
                let frame = pixels.frame_mut();
                for (_i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    pixel[0] = 30; // Red
                    pixel[1] = 30; // Green
                    pixel[2] = 35; // Blue
                    pixel[3] = 255; // Alpha
                }

                if let Err(err) = pixels.render() {
                    eprintln!("Error al renderizar los píxeles: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}