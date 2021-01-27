mod game;
mod agent;


use game::{Board};

use sfml::{
    graphics::{Color, Font, RenderTarget, RenderWindow}, 
    window::{Event, Style, mouse::Button}
};

const WIDTH: f32 = 400.0;
const HEIGHT: f32 = 400.0;

fn main() {

    let mut window = RenderWindow::new(
        (WIDTH as u32, HEIGHT as u32),
        "IAtomas",
        Style::CLOSE,
        &Default::default(),
    );

    window.set_mouse_cursor_visible(true);
    window.set_framerate_limit(60);

    let font = Font::from_file("resources/Aaargh.ttf").unwrap();

    let mut board = Board::new(&font);
    board.draw_on(&mut window);
    window.display();

    'mainloop: loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => { break 'mainloop; },
                // Event::MouseButtonPressed { button, x, y } => {
                //     // ...
                // },
                Event::MouseButtonReleased { button, x, y } => {
                    if button == Button::Left {
                        board.click(x, y);
                    }
                },
                _ => {}
            }
        }
        window.clear(Color::rgb(0, 0, 0));
        board.draw_on(&mut window);
        window.display();
    }
}