use gl;
use glutin;
use glutin::{ContextBuilder, EventsLoop, GlContext, GlWindow, WindowBuilder};

pub struct Window {
    window: GlWindow,
    events: EventsLoop,
}

impl Window {
    pub fn new(title: &str, (width, height): (f64, f64)) -> Window {
        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_dimensions(glutin::dpi::LogicalSize::new(width, height));

        let context_builder = ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
            .with_gl_profile(glutin::GlProfile::Core)
            .with_vsync(true);

        let events = EventsLoop::new();

        let window = GlWindow::new(window_builder, context_builder, &events).unwrap();

        unsafe { window.make_current().unwrap() };

        gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

        Window { window, events }
    }

    pub fn run<F>(&mut self, callback: F)
    where
        F: Fn(),
    {
        let mut close_requested = false;

        while !close_requested {
            self.events.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => close_requested = true,
                    _ => (),
                },
                _ => (),
            });

            callback();

            self.window.swap_buffers().unwrap();
        }
    }
}
