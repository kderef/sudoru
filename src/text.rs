use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};

pub struct FontManager<'a> {
    ttf_ctx: Sdl2TtfContext,
    font: Option<Font<'a, 'static>>,
}

impl<'a> FontManager<'a> {
    pub fn init() -> Result<Self, String> {
        let ttf_ctx = sdl2::ttf::init()?;

        let s = Self {
            ttf_ctx,
            font: None,
        };

        Ok(s)
    }

    pub fn load(&'a mut self) -> Result<(), String> {
        let font_data = RWops::from_bytes(FONT_BYTES)?;
        let font_size = 16;
        let font = self.ttf_ctx.load_font_from_rwops(font_data, font_size)?;

        self.font = Some(font);

        Ok(())
    }

    pub fn font(&'a self) -> Result<&'a Font<'a, '_>, String> {
        self.font.as_ref().ok_or("Font was not loaded".to_owned())
    }
}
