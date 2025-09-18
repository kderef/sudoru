#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod board;
mod draw;
mod game;
mod game_draw;
mod text;
mod theme;
mod window;

use sdl2::event::{Event, WindowEvent};

use crate::game::Game;
use crate::window::Window;

pub fn main() -> Result<(), String> {
    let mut window = Window::open("sudoru", 700, 700)?;
    let ttf_context = sdl2::ttf::init()?;

    let mut game = Game::new(&ttf_context)?;

    let mut update: bool;

    // initial frame
    game.update(&window)?;
    game.draw(window.draw())?;
    window.present();

    'running: loop {
        update = false;

        for event in window.wait_for_events() {
            match event {
                Event::Quit { .. } => break 'running,
                Event::Window { win_event, .. } => match win_event {
                    WindowEvent::Resized(_, _)
                    | WindowEvent::SizeChanged(_, _)
                    | WindowEvent::Exposed => {
                        update = true;
                    }
                    _ => {}
                },
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => {
                    game.handle_mouse(mouse_btn, (x, y).into());
                    update = true;
                }
                Event::KeyUp { keycode, .. } => {
                    if let Some(key) = keycode {
                        game.handle_key(key);
                        update = true;
                    }
                }
                _ => {}
            }
        }

        if update {
            println!("redraw");
            game.update(&window)?;
            game.draw(window.draw())?;

            window.present();
        }
    }

    Ok(())
}
