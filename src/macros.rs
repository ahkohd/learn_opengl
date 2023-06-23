#[macro_export]
/// A macro to scaffold glutin window
macro_rules! glwin {
    ($title:expr, $width:expr, $height:expr, $el:expr) => {{
        let event_loop = glium::glutin::event_loop::EventLoop::new();
        let wb = glium::glutin::window::WindowBuilder::new().with_title($title);
        let cb = glium::glutin::ContextBuilder::new();
        let display = glium::Display::new(wb, cb, &event_loop).unwrap();
        event_loop.run(move |ev, _, control_flow| {
            $el(display.clone());
            let next_frame_time =
                std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
            *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
            if let glium::glutin::event::Event::WindowEvent { event, .. } = ev {
                if event == glium::glutin::event::WindowEvent::CloseRequested {
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                }
            }
        });
    }};
}
