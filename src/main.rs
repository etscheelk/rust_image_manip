use winit::raw_window_handle::{HasDisplayHandle, HasRawDisplayHandle};

#[derive(Default)]
struct Application
{
    window: Option<winit::window::Window>,
    pixels: Option<pixels::Pixels>,

    image_a: Option<image::RgbaImage>,
    image_b: Option<image::RgbaImage>,
    mouse_pressed: bool
}

impl Application
{
    /// Given a new image in field self.image_a,
    /// this function shall resize the window and load and display
    /// the image
    fn new_image_placed(&mut self)
    {
        // assumes window and image_a are something. They will
        // be if this is being called.
        if let (Some(window), Some(img), Some(pixels)) = (&mut self.window, &mut self.image_a, &mut self.pixels)
        {
            let mut surface_size = window.inner_size();
            surface_size.width = img.width();
            surface_size.height = img.height();

            let _res = window.request_inner_size(surface_size);
            let _res = pixels.resize_surface(img.width(), img.height());
            let _res = pixels.resize_buffer(img.width(), img.height());

            pixels
            .frame_mut()
            .chunks_exact_mut(4)
            .zip(img.pixels())
            .for_each(
            |(f, i)|
            {
                if let (
                    [r, g, b, a],
                    [rr, gg, bb, aa]
                ) = (f, i.0)
                {
                    (*r, *g, *b, *a) = (rr, gg, bb, aa);
                }
            });
            let _ = pixels.render();
        }
    }

    // fn get_pixels(&mut self) -> &mut pixels::Pixels
    // {
    //     self.pixels
    // }
}

fn verify_image_correct(path_buf: std::path::PathBuf) -> Result<image::DynamicImage, Box<dyn std::error::Error>>
{
    let img = image::ImageReader::open(path_buf)?.decode()?;
    Ok(img)
}

impl winit::application::ApplicationHandler for Application
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) 
    {
        let window = event_loop.create_window(winit::window::Window::default_attributes()).unwrap();
        let size = window.inner_size();
        self.window = Some(window);
        let w = self.window.as_mut();
        if let Some(w) = w
        {
            // let a = w.display_handle().as_ref().unwrap();
            let surface_texture = pixels::SurfaceTexture::new(size.width, size.height, w);
            let pixels = 
                pixels::PixelsBuilder::new(size.width, size.height, surface_texture)
                .build()
                .expect("Failed to setup rendering");
            self.pixels = Some(pixels);
        }
        // let surface_texture = pixels::SurfaceTexture::new(size.width, size.height, &window);
        // let pixels = pixels::PixelsBuilder::new(window., height, surface_texture)
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) 
    {
        let _ = window_id;
        use winit::event::WindowEvent::*;
        match event
        {
            // ActivationTokenDone { serial, token } => todo!(),
            // Resized(physical_size) => todo!(),
            // Moved(physical_position) => 
            // {
            //     println!("new position: {:?}", physical_position);
            // },
            CloseRequested => 
            {
                println!("Close button pressed");
                event_loop.exit();
            },
            // Destroyed => todo!(),
            DroppedFile(_path_buf) => 
            {
                self.image_a = self.image_b.take(); 
                if let Some(img) = &self.image_a
                {
                    println!("{:?}x{:?}", img.height(), img.width());
                }
                // predicting a bug: If I have an image currently loaded and I drag a bogus item it will unload the current image

                self.new_image_placed();
            },
            HoveredFile(path_buf) => 
            {
                println!("User hovered on a file {:?}", path_buf);
                match verify_image_correct(path_buf)
                {
                    Ok(ok) => { self.image_b = Some(ok.into_rgba8()); },
                    Err(e) => { println!("{e}"); self.image_b = None; }
                };
            },
            // HoveredFileCancelled => todo!(),
            // Focused(_) => todo!(),
            // KeyboardInput { device_id, event, is_synthetic } => todo!(),
            // ModifiersChanged(modifiers) => todo!(),
            // Ime(ime) => todo!(),
            // CursorMoved { device_id, position } => todo!(),
            // CursorEntered { device_id } => todo!(),
            // CursorLeft { device_id } => todo!(),
            // MouseWheel { device_id, delta, phase } => todo!(),
            MouseInput { state, button, .. } => 
            {
                use winit::event::{MouseButton, ElementState};
                println!("Mouse pressed:\n\tstate: {:?}\n\tbutton: {:?}", state, button);
                self.mouse_pressed = button == MouseButton::Left && state == ElementState::Pressed;
            },
            // PinchGesture { device_id, delta, phase } => todo!(),
            // PanGesture { device_id, delta, phase } => todo!(),
            // DoubleTapGesture { device_id } => todo!(),
            // RotationGesture { device_id, delta, phase } => todo!(),
            // TouchpadPressure { device_id, pressure, stage } => todo!(),
            // AxisMotion { device_id, axis, value } => todo!(),
            // Touch(touch) => todo!(),
            // ScaleFactorChanged { scale_factor, inner_size_writer } => todo!(),
            // ThemeChanged(theme) => todo!(),
            // Occluded(_) => todo!(),
            // RedrawRequested => todo!(),
            
            _ => ()
        }
    }
}

fn main()  -> Result<(), Box<dyn std::error::Error>>
{
    let event_loop = winit::event_loop::EventLoop::new()?;

    let mut app = Application::default();
    event_loop.run_app(&mut app)?;

    Ok(())
}
