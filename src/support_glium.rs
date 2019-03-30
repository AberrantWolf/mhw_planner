use glium::{
    backend::{Context, Facade},
    Texture2d,
};
use imgui::{FontGlyphRange, ImFontConfig, ImGui, Ui};
use imgui_winit_support;
use std::rc::Rc;
use std::time::Instant;

pub type Textures = imgui::Textures<Texture2d>;

pub fn run<F>(title: String, clear_color: [f32; 4], mut run_ui: F)
where
    F: FnMut(&Ui, &Rc<Context>, &mut Textures) -> bool,
{
    use glium::glutin;
    use glium::{Display, Surface};
    use imgui_glium_renderer::Renderer;

    let mut events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let builder = glutin::WindowBuilder::new()
        .with_title(title)
        .with_dimensions(glutin::dpi::LogicalSize::new(1024f64, 768f64));
    let display = Display::new(builder, context, &events_loop).unwrap();
    let window = display.gl_window();

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);

    // In the examples we only use integer DPI factors, because the UI can get very blurry
    // otherwise. This might or might not be what you want in a real application.
    let hidpi_factor = window.get_hidpi_factor().round();

    let font_size = (13.0 * hidpi_factor) as f32;

    imgui.fonts().add_default_font_with_config(
        ImFontConfig::new()
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size),
    );

    // imgui.fonts().add_font_with_config(
    //     include_bytes!("../../../resources/mplus-1p-regular.ttf"),
    //     ImFontConfig::new()
    //         .merge_mode(true)
    //         .oversample_h(1)
    //         .pixel_snap_h(true)
    //         .size_pixels(font_size)
    //         .rasterizer_multiply(1.75),
    //     &FontGlyphRange::japanese(),
    // );

    // NORMAL = 1
    let font_size = (18.0 * hidpi_factor) as f32;
    imgui.fonts().add_font_with_config(
        include_bytes!("../resources/NotoSansJP-Regular.otf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            .merge_mode(false),
        &FontGlyphRange::japanese(),
    );

    // WINDOW TITLE = 2
    let font_size = (18.0 * hidpi_factor) as f32;
    imgui.fonts().add_font_with_config(
        include_bytes!("../resources/NotoSansJP-Bold.otf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            .merge_mode(false),
        &FontGlyphRange::japanese(),
    );

    // MENU = 3
    let font_size = (24.0 * hidpi_factor) as f32;
    imgui.fonts().add_font_with_config(
        include_bytes!("../resources/NotoSansJP-Bold.otf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            //.rasterizer_multiply(1.0)
            .merge_mode(false),
        &FontGlyphRange::japanese(),
    );

    // HEADER = 4
    let font_size = (48.0 * hidpi_factor) as f32;
    imgui.fonts().add_font_with_config(
        include_bytes!("../resources/NotoSansJP-Bold.otf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            //.rasterizer_multiply(1.0)
            .merge_mode(false),
        &FontGlyphRange::japanese(),
    );

    // MINI = 5
    let font_size = (14.0 * hidpi_factor) as f32;
    imgui.fonts().add_font_with_config(
        include_bytes!("../resources/NotoSansJP-Bold.otf"),
        ImFontConfig::new()
            .merge_mode(true)
            .oversample_h(1)
            .pixel_snap_h(true)
            .size_pixels(font_size)
            //.rasterizer_multiply(1.0)
            .merge_mode(false),
        &FontGlyphRange::japanese(),
    );

    imgui.set_font_global_scale((1.0 / hidpi_factor) as f32);

    let mut renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialize renderer");

    imgui_winit_support::configure_keys(&mut imgui);

    let mut last_frame = Instant::now();
    let mut quit = false;

    loop {
        events_loop.poll_events(|event| {
            use glium::glutin::{Event, WindowEvent::CloseRequested};

            imgui_winit_support::handle_event(
                &mut imgui,
                &event,
                window.get_hidpi_factor(),
                hidpi_factor,
            );

            if let Event::WindowEvent { event, .. } = event {
                match event {
                    CloseRequested => quit = true,
                    _ => (),
                }
            }
        });

        let now = Instant::now();
        let delta = now - last_frame;
        let delta_s = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1_000_000_000.0;
        last_frame = now;

        imgui_winit_support::update_mouse_cursor(&imgui, &window);

        let frame_size = imgui_winit_support::get_frame_size(&window, hidpi_factor).unwrap();

        let ui = imgui.frame(frame_size, delta_s);
        if !run_ui(&ui, display.get_context(), renderer.textures()) {
            break;
        }

        let mut target = display.draw();
        target.clear_color(
            clear_color[0],
            clear_color[1],
            clear_color[2],
            clear_color[3],
        );
        renderer.render(&mut target, ui).expect("Rendering failed");
        target.finish().unwrap();

        if quit {
            break;
        }
    }
}
