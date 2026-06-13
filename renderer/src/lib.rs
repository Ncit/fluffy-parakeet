use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod mesh;
mod pipeline;
mod scene;
mod state;
mod uniforms;

pub struct Renderer<'window> {
    pub state: state::State<'window>,
}

impl<'window> Renderer<'window> {
    pub async fn new(window: &'window winit::window::Window) -> Self {
        let state = state::State::new(window).await;
        Self { state }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.state.resize(size);
    }

    pub fn render(&mut self) {
        self.state.render();
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("Fluffy Parakeet Renderer")
        .build(&event_loop)
        .unwrap();

    let mut renderer = pollster::block_on(Renderer::new(&window));

    event_loop
        .run(move |event, control_flow| {
            control_flow.set_control_flow(ControlFlow::Poll);

            match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                    WindowEvent::CloseRequested => control_flow.exit(),
                    WindowEvent::Resized(size) => renderer.resize(size),
                    WindowEvent::ScaleFactorChanged { .. } => renderer.resize(window.inner_size()),
                    WindowEvent::RedrawRequested => renderer.render(),
                    _ => {}
                },
                Event::AboutToWait => {
                    window.request_redraw();
                }
                _ => {}
            }
        })
        .unwrap();
}
