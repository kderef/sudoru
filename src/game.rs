use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;
use sdl2::rwops::RWops;
use sdl2::ttf::{Font, Sdl2TtfContext};

use crate::board::{self, Board, Position, Strategy};
use crate::draw::DrawCanvas;
use crate::theme::{self, Theme};
use crate::window::{self, Window};

pub struct Game<'a> {
    pub theme: Theme,
    pub window: Rect,
    pub board_rect: Rect,
    pub board: Board,

    pub selected_square: Option<usize>,
    pub highlighted_square: Option<usize>,

    pub font: Font<'a, 'static>,
}

const FONT_BYTES: &[u8] = include_bytes!("../ConsolaMono-Book.ttf");

impl<'a> Game<'a> {
    pub fn new(ttf_context: &'a Sdl2TtfContext) -> Result<Self, String> {
        let rect_zero = Rect::new(0, 0, 0, 0);

        let gen_strategy = Strategy::TryRandomSparse;
        let board = board::generate_board(gen_strategy);

        // load font
        let font_data = RWops::from_bytes(FONT_BYTES)?;
        let font_size = 100;
        let font = ttf_context.load_font_from_rwops(font_data, font_size)?;

        let s = Self {
            theme: theme::LIGHT,
            window: rect_zero,
            board_rect: rect_zero,
            board,

            selected_square: None,
            highlighted_square: None,
            font,
        };

        Ok(s)
    }

    pub fn update(&mut self, window: &Window) -> Result<(), String> {
        self.window = window.rect()?;
        let margin = 10;

        let (win_w, win_h) = self.window.size();

        let smallest_side = win_w.min(win_h);
        let largest_side = win_w.max(win_h);

        let size = smallest_side - margin * 2;

        let padding_x = (win_w - size) / 2;
        let padding_y = (win_h - size) / 2;

        self.board_rect = Rect::new(padding_x as i32, padding_y as i32, size, size);

        Ok(())
    }

    pub fn handle_mouse(&mut self, btn: MouseButton, mut mouse_pos: Point) {
        let board = self.board_rect;

        let dpi_scale = window::dpi_scale();
        if dpi_scale > 1 {
            mouse_pos *= dpi_scale as i32;
        }

        println!("{mouse_pos:?} <> {:?}", board);

        if !board.contains_point(mouse_pos) {
            return;
        }
        if btn != MouseButton::Left {
            return;
        }

        let cell_size = board.w / 9;
        let pos = mouse_pos - board.top_left();

        let index = pos / cell_size;
        let index = index.x as usize + index.y as usize * 9;

        if index > self.board.size() {
            return;
        }

        if Some(index) == self.selected_square {
            self.selected_square = None;
        } else {
            self.selected_square = Some(index);
        }
        println!("{:?}", self.selected_square);
    }

    pub fn handle_key(&mut self, key: Keycode) {
        if self.selected_square.is_none() {
            return;
        }

        let insert_num = match key {
            // insertion
            Keycode::NUM_1 => 1,
            Keycode::NUM_2 => 2,
            Keycode::NUM_3 => 3,
            Keycode::NUM_4 => 4,
            Keycode::NUM_5 => 5,
            Keycode::NUM_6 => 6,
            Keycode::NUM_7 => 7,
            Keycode::NUM_8 => 8,
            Keycode::NUM_9 => 9,

            // check for movement keys and return
            other => {
                if let Some(ref mut selected) = self.selected_square {
                    let (x, y) = selected.coords();
                    let (w, h) = (self.board.width() - 1, self.board.height() - 1);
                    let (mut nx, mut ny) = (x, y);

                    match other {
                        Keycode::RIGHT => {
                            nx = (nx + 1) % (w + 1);
                        }
                        Keycode::DOWN => {
                            ny = (ny + 1) % (h + 1);
                        }
                        Keycode::LEFT => {
                            nx = nx.checked_sub(1).unwrap_or(w);
                        }
                        Keycode::UP => {
                            ny = ny.checked_sub(1).unwrap_or(h);
                        }
                        _ => {}
                    }
                    *selected = (nx, ny).index();
                }
                return;
            }
        };

        // try to insert
        if let Some(ref mut selected) = self.selected_square {
            let cell = Some(insert_num);
            let problem = self.board.placement_error(*selected, cell);

            println!("{problem:?}");
        }
    }

    pub fn draw(&mut self, canvas: &mut WindowCanvas) -> Result<(), String> {
        let Theme {
            bg,
            border_color,
            border_thick,
            square_color,
            square_thick,
            cell_thick,
            cell_border,
            ..
        } = self.theme;

        // clear
        canvas.set_draw_color(bg);
        canvas.clear();

        // show selected
        self.draw_selected(canvas);

        self.draw_cell_lines(canvas);
        self.draw_segment_lines(canvas);
        self.draw_cell_numbers(canvas);

        // show segment lines
        // show board outline
        self.draw_borders(canvas);

        Ok(())
    }
}
