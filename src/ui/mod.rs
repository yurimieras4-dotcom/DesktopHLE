use crate::frameworks;
use egui::Context;
use egui_winit::State as EguiState;
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub fn run_ui() {
    frameworks::foundation::ns_log("Cargando subsistema de UI...");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("DesktopHLE - UITest (Beta)")
        .with_inner_size(LogicalSize::new(640.0, 480.0))
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
    let mut pixels = Pixels::new(640, 480, surface_texture).unwrap();

    let egui_ctx = Context::default();
    let mut egui_state = EguiState::new(&window);

    frameworks::app_kit::ns_application_main();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => {
                let _ = egui_state.on_event(&egui_ctx, &event);
                if let WindowEvent::CloseRequested = event {
                    *control_flow = ControlFlow::Exit;
                }
            }
            Event::MainEventsCleared => {
                // Dibujar fondo oscuro en el framebuffer
                let frame = pixels.frame_mut();
                for pixel in frame.chunks_exact_mut(4) {
                    pixel[0] = 30; // Red
                    pixel[1] = 30; // Green
                    pixel[2] = 35; // Blue
                    pixel[3] = 255; // Alpha
                }

                // Dibujar la interfaz flotante con egui
                let raw_input = egui_state.take_egui_input(&window);
                egui_ctx.begin_frame(raw_input);

                egui::Window::new("Controles HLE").show(&egui_ctx, |ui| {
                    ui.label("Teclas de Teclado Hardware:");
                    ui.horizontal(|ui| {
                        if ui.button("⌘ Command").clicked() {
                            frameworks::foundation::ns_log("Presionado: Tecla ⌘ Command");
                        }
                        if ui.button("⌥ Option").clicked() {
                            frameworks::foundation::ns_log("Presionado: Tecla ⌥ Option");
                        }
                    });
                });

                let _full_output = egui_ctx.end_frame();

                if let Err(err) = pixels.render() {
                    eprintln!("Error al renderizar los píxeles: {err}");
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
}
