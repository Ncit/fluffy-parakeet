use winit::{event::*, event_loop::EventLoop, window::WindowBuilder};

mod state;

pub struct Renderer {
    pub state: Option<state::State>,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Self {
        let state = state::State::new(window).await;
        Self { state: Some(state) }
    }

    pub fn render(&mut self) {
        if let Some(state) = &mut self.state {
            state.render();
        }
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
