use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

pub struct Renderer {
    pub state: Option<RenderState>,
}

pub struct RenderState {
    pub size: (u32, u32),
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Self {
        Self { state: Some(RenderState { size: (800, 600) }) }
    }

    pub fn render(&mut self) {
        // TODO: wgpu frame rendering pipeline
    }
}

pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .build(&event_loop)
        .unwrap();

    let mut renderer = pollster::block_on(Renderer::new(&window));

    event_loop
        .run(move |event, control_flow| {
            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.exit(),
                    _ => {}
                },
                Event::MainEventsCleared => {
                    renderer.render();
                }
                _ => {}
            }
        })
        .unwrap();
}
