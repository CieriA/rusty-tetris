//! # Engine
//! Contains the struct which runs the program

/// Visualize the program
mod interface;

use game::Game;
use std::{
    error::Error as StdError,
    time::{
        Instant,
        Duration
    },
};
use sdl2::{
    event::Event::{
        self,
        KeyDown,
        KeyUp,
    },
    keyboard::Keycode,
    pixels::Color,
};

pub struct Engine {
    game: Game,
}

impl Engine {
    pub fn new(game: Game) -> Self {
        Self {
            game,
        }
    }
    /// Runs the program and draws it via `interface` module.
    pub fn run(&mut self) -> Result<(), Box<dyn StdError>> {
        let sdl = sdl2::init()?;
        let ttf = sdl2::ttf::init()?;
        let video = sdl.video()?;

        let window = video
            .window("Rusty Tetris", interface::WIDTH, interface::HEIGHT)
            .position_centered()
            .build()?;

        let mut canvas = window
            .into_canvas()
            .build()?;

        canvas.set_blend_mode(sdl2::render::BlendMode::Blend);

        let mut last_tick = Instant::now();

        let mut event_pump = sdl.event_pump()?;

        'running: loop {
            // event loop
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    KeyDown { keycode: Some(key), .. } => {
                        if key == Keycode::Escape {
                            break 'running;
                        }
                        self.game.update(key);
                    }
                    KeyUp { keycode: Some(Keycode::S | Keycode::Down), .. } => {
                        self.game.tick = Game::BASE_TICK;
                    }
                    _ => {}
                }
            }

            if last_tick.elapsed() >= self.game.tick {
                if !self.game.try_drop() {
                    self.game.place();
                }
                last_tick = Instant::now();
            }

            canvas.set_draw_color(Color::BLACK);
            canvas.clear();

            interface::draw(&mut canvas, &ttf, &mut self.game)?;

            canvas.present();

            std::thread::sleep(Duration::from_millis(16));
        }

        Ok(())
    }
}
