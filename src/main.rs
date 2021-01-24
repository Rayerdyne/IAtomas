mod game;
mod agent;


use game::{GameState, draw_state};

use sfml::{
    graphics::{RenderWindow, Font}, 
    window::{Event, Style}
};

// use lazy_static::lazy_static;

// lazy_static!{
//     static ref STATE: Mutex<GameState> = 
//         Mutex::new(GameState::start_game());
// }

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;

fn main() {

    let mut window = RenderWindow::new(
        (WIDTH as u32, HEIGHT as u32),
        "SFML Pong",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_mouse_cursor_visible(true);
    window.set_framerate_limit(60);

    let font = Font::from_file("resources/Aaargh.ttf").unwrap();

    let mut state = GameState::start_game();

    'mainloop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => { break 'mainloop; },
                Event::MouseButtonPressed { button, x, y } => {
                    // ...
                },
                Event::MouseButtonReleased { button, x, y } => {
                    // ...
                },
                _ => {}
            }
        }
        draw_state(&mut window, &state, &font);
        window.display();
    }
}