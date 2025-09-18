use sdl2::{
    rect::{Point, Rect},
    render::{Canvas, RenderTarget, WindowCanvas},
};

pub trait DrawCanvas {
    fn draw_rect_lines(&mut self, rect: Rect, thick: u32);
    fn draw_hline(&mut self, x1: i32, x2: i32, y: i32, thick: u32);
    fn draw_vline(&mut self, y1: i32, y2: i32, x: i32, thick: u32);
}

impl<T: RenderTarget> DrawCanvas for Canvas<T> {
    fn draw_rect_lines(&mut self, rect: Rect, thick: u32) {
        let topl = rect.top_left();
        let topr = rect.top_right();
        let botl = rect.bottom_left();
        // let botr = rect.bottom_right();

        let _ = self.fill_rects(&[
            Rect::new(topl.x, topl.y, rect.width(), thick),
            Rect::new(botl.x, botl.y - thick as i32, rect.width(), thick),
            Rect::new(topl.x, topl.y, thick, rect.height()),
            Rect::new(topr.x - thick as i32, topr.y, thick, rect.height()),
        ]);

        // let _ = self.draw_line(topl, topr);
        // let _ = self.draw_line(topl, botl);
        // let _ = self.draw_line(topr, botr);
        // let _ = self.draw_line(botr, botl);
    }
    fn draw_hline(&mut self, x1: i32, x2: i32, y: i32, thick: u32) {
        let y = y - thick as i32 / 2;

        let rect = Rect::new(x1, y, (x2 - x1) as u32, thick as u32);

        self.fill_rect(rect);
    }
    fn draw_vline(&mut self, y1: i32, y2: i32, x: i32, thick: u32) {
        let x = x - thick as i32 / 2;
        let rect = Rect::new(x, y1, thick as u32, (y2 - y1) as u32);
        self.fill_rect(rect);
    }
}
