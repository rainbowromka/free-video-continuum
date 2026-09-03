use crate::ui::ui::Ui;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::{Api, ContextBuilder, GlRequest};

pub struct Window {
    width: u32,
    height: u32,
    title: String,
    client_color: (u8, u8, u8),
    on_draw: Option<Box<dyn FnMut(&mut Ui)>>,
}

impl Window {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            title: String::new(),
            client_color: (0xf8, 0xf8, 0xf8),    
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

    pub fn set_client_color(mut self, r: u8, g: u8, b: u8) -> Self {
        self.client_color = (r, g, b);
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
            .with_inner_size(glutin::dpi::LogicalSize::new(self.width, self.height))
            .with_min_inner_size(glutin::dpi::LogicalSize::new(1024.0, 720.0));

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

        
        let mut ui = Ui::new(self.width, self.height);
        ui.set_client_color(self.client_color.0, self.client_color.1, self.client_color.2);

        let mut on_draw = self.on_draw.unwrap();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(physical_size) => {
                        gl_context.resize(physical_size);
                        ui.resize(physical_size.width, physical_size.height);
                    }
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    let size = gl_context.window().inner_size();
                    ui.update_size(size.width, size.height);  // только размеры, без dirty
                    
                    on_draw(&mut ui);

                    ui.render();
                    gl_context.swap_buffers().unwrap();
                }
                _ => (),
            }
        });
    }
}

fn hex_to_rgb(r: u8, g: u8, b: u8) -> (f32, f32, f32) {
    (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}
