use crate::ui::renderer::Renderer;
use crate::ui::ui::Ui;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

pub struct Window {
    width: u32,
    height: u32,
    title: String,
    on_draw: Option<Box<dyn FnMut(&mut Ui)>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            title: String::new(),
            on_draw: None,
        }
    }

    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn with_title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }

    pub fn on_draw<F: FnMut(&mut Ui) + 'static>(mut self, callback: F) -> Self {
        self.on_draw = Some(Box::new(callback));
        self
    }

    pub fn run(self) {
        let event_loop = EventLoop::new();
        let window_builder = WindowBuilder::new()
            .with_title(&self.title)
            .with_inner_size(glutin::dpi::LogicalSize::new(self.width, self.height));

        let gl_context = ContextBuilder::new()
            .with_gl(GlRequest::Specific(Api::OpenGl, (3, 3)))
            .build_windowed(window_builder, &event_loop)
            .expect("Cannot create windowed context");

        let gl_context = unsafe {
            gl_context
                .make_current()
                .expect("Failed to make context current")
        };

        gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        let mut renderer = Renderer::new(self.width, self.height).expect("Cannot create renderer");
        let mut ui = Ui::new(self.width, self.height);
        let mut on_draw = self.on_draw.unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        gl_context.resize(physical_size);
                        renderer.resize(physical_size.width, physical_size.height);
                        ui.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    renderer.clear();                    
                    on_draw(&mut ui);
                    ui.render(&renderer);
                    gl_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}