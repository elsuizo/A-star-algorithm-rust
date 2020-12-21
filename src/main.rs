// TODO(elsuizo:2020-12-20): cosas que faltan:
// - [  ] Hacer una funcion para graficar la grilla
// - [  ] cuando pasamos con el mouse por una cell tenemos que poder elegirla
// - [  ] hacer una gui chica para elegir los colores
use sfml::graphics::{CircleShape, Color, Font, RectangleShape, RenderTarget, Rect,
                     RenderWindow, Shape,Text, Transformable, Vertex, VertexArray, PrimitiveType};

use sfml::system::{Clock, Time, Vector2f, Vector2u, Vector3f, Vector2};
use sfml::window::{mouse, VideoMode, ContextSettings, Event, Key, Style};

const WINDOW_WIDTH:  f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

const DRAW_GRID_WH: u8 = 16;
const DRAW_CELL_WH: u8 = 26;

const DRAW_AREA_TOPLEFT: (u16, u16) = (200, 100);

fn mouse_over(rect: &Rect<i32>, mouse_x: i32, mouse_y: i32) -> bool {
    rect.contains(Vector2::new(mouse_x, mouse_y))
}

fn gridindex(grid: &mut [bool; DRAW_GRID_WH as usize * DRAW_GRID_WH as usize],
             x: usize, y: usize) -> Option<&mut bool>
{
    grid.get_mut(y * DRAW_GRID_WH as usize + x)
}

fn main() {
    let mut window = RenderWindow::new(
        (WINDOW_WIDTH as u32, WINDOW_HEIGHT as u32),
        "A* algorithm",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let background_color = Color::BLACK;
    let mut is_running = true;
    // con una shape podemos reutilizarla para cada cell
    let mut shape = RectangleShape::default();
    shape.set_outline_thickness(-1.0);
    shape.set_outline_color(Color::RED);

    let set_button = Rect::new(348, 500, 100, 32);
    // esta es la grilla que usa de para saber si esta en la cell creo
    let mut pixel_grid = [false; DRAW_GRID_WH as usize * DRAW_GRID_WH as usize];
    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed {code: Key::Escape, ..} => return,
                Event::KeyPressed {code: Key::Space, ..} if !is_running => {
                    is_running = true;
                    //clock.restart();
                    println!("space!!!");
                },
                _ => {}
            }
        }

        let mouse_position = window.mouse_position();
        // TODO(elsuizo:2020-12-20): esto tiene que ir a una funcion
        shape.set_fill_color(Color::TRANSPARENT);
        for y in 0..DRAW_GRID_WH {
            for x in 0..DRAW_GRID_WH {
                if let Some(cell) = gridindex(&mut pixel_grid, x as usize, y as usize) {
                    shape.set_fill_color(Color::RED);
                    if mouse::Button::Left.is_pressed() {
                        *cell = false;
                        shape.set_fill_color(Color::BLACK);
                    }
                }
                shape.set_outline_color(Color::rgb(180, 180, 180));
                shape.set_size((DRAW_CELL_WH as f32, DRAW_CELL_WH as f32));
                shape.set_position((
                    DRAW_AREA_TOPLEFT.0 as f32 + (x as f32 * DRAW_CELL_WH as f32),
                    DRAW_AREA_TOPLEFT.1 as f32 + (y as f32 * DRAW_CELL_WH as f32),
                ));
                window.draw(&shape);
            }
        }
        window.display();
        window.clear(background_color);
    }
}
