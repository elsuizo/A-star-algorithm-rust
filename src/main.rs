extern crate sfml;

use sfml::graphics::{CircleShape, Color, Font, RectangleShape, RenderTarget,
                     RenderWindow, Shape,Text, Transformable, Vertex, VertexArray, PrimitiveType};

use sfml::system::{Clock, Time, Vector2f, Vector2u, Vector3f};
use sfml::window::{VideoMode, ContextSettings, Event, Key, Style};

const WINDOW_WIDTH:  f32 = 500.0;
const WINDOW_HEIGHT: f32 = 500.0;

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        "Double pendulum simulation",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let background_color = Color::BLACK;
    let mut is_running = true;
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => return,
                Event::KeyPressed {code: Key::Space, ..} if !is_running => {
                    is_running = true;
                    //clock.restart();
                    println!("space!!!");
                }
                _ => {}
            }
        }
        window.clear(background_color);
        window.display();
    }
}
