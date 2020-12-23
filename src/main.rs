// TODO(elsuizo:2020-12-20): cosas que faltan:
// - [X] Hacer una funcion para graficar la grilla
// - [  ] cuando pasamos con el mouse por una cell tenemos que poder elegirla
// - [  ] hacer una gui chica para elegir los colores
use sfml::graphics::{Color, RectangleShape, RenderTarget, Rect,
                     RenderWindow, Shape,Text, Transformable, Vertex, VertexArray, PrimitiveType};

use sfml::system::{Clock, Time, Vector2f, Vector2u, Vector3f, Vector2};
use sfml::window::{mouse, VideoMode, ContextSettings, Event, Key, Style};

use rand::{Rng};

const WINDOW_WIDTH:  f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[derive(Debug)]
struct Cell<'a> {
    shape: RectangleShape<'a>,
    position: Vector2f,
    color: Color,
    active: bool,
    // id: usize
}

// NOTE(elsuizo:2020-12-22): voy a probar teniendo un solo shape para todos los
// shapes ya que
impl<'a> Cell<'a> {
    const WH: u8 = 26;

    fn new(position: Vector2f, color: Color) -> Self {
        let mut shape = RectangleShape::default();
        shape.set_position(position);
        shape.set_outline_thickness(3.0);
        shape.set_fill_color(color);
        shape.set_outline_color(Color::BLACK);
        shape.set_size((Self::WH as f32, Self::WH as f32));

        Self{shape, color, position, active: true}
    }

}

#[derive(Debug)]
struct Grid<'a> {
    cells: Vec<Cell<'a>>,
}

impl<'a> Grid<'a> {
    const WH: u8 = 16;
    const TOPLEFT: (u16, u16) = (200, 100);
    fn new(cells: Vec<Cell<'a>>) -> Self {
        Self{cells}
    }

    fn create_grid() -> Self {
        let mut cells = Vec::new();
        for y in 0..Self::WH {
            for x in 0..Self::WH {
                let cell_position = Vector2f::new(Self::TOPLEFT.0 as f32 + (x as f32 * Cell::WH as f32),
                                                  Self::TOPLEFT.1 as f32 + (y as f32 * Cell::WH as f32),);
                let color = pick_random_color();
                let mut cell = Cell::new(cell_position, color);
                cells.push(cell);
            }
        }
        Self::new(cells)
    }
}

fn mouse_over(rect: &Rect<i32>, mouse_x: i32, mouse_y: i32) -> bool {
    rect.contains(Vector2::new(mouse_x, mouse_y))
}

// fn gridindex(grid: &mut [bool; DRAW_GRID_WH as usize * DRAW_GRID_WH as usize],
//              x: usize, y: usize) -> Option<&mut bool>
// {
//     grid.get_mut(y * DRAW_GRID_WH as usize + x)
// }

fn gen_random_color() -> Color {
    let mut rng = rand::thread_rng();
    Color::rgb(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255))
}

fn pick_random_color() -> Color {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..5) {
        0 => Color::RED,
        1 => Color::GREEN,
        2 => Color::WHITE,
        3 => Color::BLUE,
        _ => gen_random_color()
    }
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

    let grid = Grid::create_grid();

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

        // let mouse_position = window.mouse_position();
        for cell in &grid.cells {
            window.draw(&cell.shape);
        }
        window.display();
        window.clear(background_color);
    }
}
