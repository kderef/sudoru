use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

use sdl2::{
    EventPump, Sdl, VideoSubsystem,
    event::EventWaitTimeoutIterator,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    rwops::RWops,
    ttf::{Font, Sdl2TtfContext},
};

pub static DPI_SCALE: AtomicU32 = AtomicU32::new(1);

pub fn dpi_scale() -> u32 {
    DPI_SCALE.load(Ordering::Relaxed)
}

fn fps_millis(seconds: u32) -> u32 {
    let ms = (1. / seconds as f32) * 1000.;
    ms.ceil() as u32
}

pub struct Window {
    sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    canvas: WindowCanvas,
    event_pump: EventPump,
    fpms: u32,
}

impl Window {
    pub fn open(title: &str, w: u32, h: u32) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let window = video_subsystem
            .window(title, w, h)
            .position_centered()
            .resizable()
            .allow_highdpi()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        // check for hidpi
        let out_size = canvas.output_size()?;
        let dpi_scale = out_size.0 / w; // NOTE probably wrong

        DPI_SCALE.store(dpi_scale, Ordering::Relaxed);

        let event_pump = sdl_context.event_pump()?;

        let win = Self {
            sdl_context,
            video_subsystem,
            canvas,
            event_pump,
            fpms: fps_millis(60),
        };

        Ok(win)
    }

    pub fn wait_for_events(&mut self) -> EventWaitTimeoutIterator {
        self.event_pump.wait_timeout_iter(self.fpms)
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }

    pub fn clear(&mut self, color: Color) {
        self.canvas.set_draw_color(color);
        self.canvas.clear();
    }

    pub fn size(&self) -> Result<(u32, u32), String> {
        self.canvas.output_size()
    }

    pub fn rect(&self) -> Result<Rect, String> {
        let (w, h) = self.size()?;
        Ok(Rect::new(0, 0, w, h))
    }

    pub fn draw(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }
}

impl AsRef<WindowCanvas> for Window {
    fn as_ref(&self) -> &WindowCanvas {
        &self.canvas
    }
}
