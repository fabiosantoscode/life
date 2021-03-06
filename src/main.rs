extern crate piston_window;

use piston_window::*;
use std::ops::Range;
use std::option::Option;

// Don't go too fast
const FPS: u64 = 8;

const WIDTH: usize = 64;
const HEIGHT: usize = 48;

const COLUMNS: Range<usize> = 0..WIDTH;
const ROWS: Range<usize> = 0..HEIGHT;

#[derive(PartialEq, Clone, Copy, Default)]
struct AppState {
    x: usize,
    y: usize,
    down: bool,
    editing: bool,
}

type GameMap = [bool; 64 * 48];
type Coords = (usize, usize);

trait GameMapTrait {
    fn new() -> GameMap;
    fn get(self: &Self, c: Coords) -> bool;
    fn set(self: &mut Self, c: Coords, new_alive: bool);
}

impl GameMapTrait for GameMap {
    fn new() -> Self {
        [false; 64 * 48]
    }
    fn get(self: &Self, (x, y): Coords) -> bool {
        self[(y * WIDTH) + x]
    }
    fn set(self: &mut Self, (x, y): Coords, new_alive: bool) {
        self[(y * WIDTH) + x] = new_alive;
    }
}

fn fill_map() -> GameMap {
    let mut ret = GameMap::new();

    //  x
    //   x
    // xxx
    ret.set((1, 2), true);
    ret.set((2, 3), true);
    ret.set((0, 4), true);
    ret.set((1, 4), true);
    ret.set((2, 4), true);

    //  x
    //   x
    // xxx
    ret.set((1, 8), true);
    ret.set((2, 9), true);
    ret.set((0, 10), true);
    ret.set((1, 10), true);
    ret.set((2, 10), true);

    // xxx
    ret.set((30, 3), true);
    ret.set((31, 3), true);
    ret.set((32, 3), true);

    ret
}

fn is_cell_alive(map: &GameMap, x: usize, y: usize) -> bool {
    map[(y * 64) + x]
}

const NEG_X: usize = WIDTH - 1;
const NEG_Y: usize = HEIGHT - 1;
// added 64 and 48 respectively to avoid negative numbers when using modulo
const POSSIBLE_NEIGHBOURS_PAIRS: [Coords; 8] = [
    // top row
    (NEG_X, NEG_Y),
    (NEG_X, 0),
    (NEG_X, 1),
    // middle row (only two because 0,0 is the cell
    // itself, not a neighbour
    (0, NEG_Y),
    (0, 1),
    // bottom row
    (1, NEG_Y),
    (1, 0),
    (1, 1),
];

fn does_cell_live(map: &GameMap, (x, y): Coords) -> bool {
    let mut live_neighbour_count = 0;
    for (x_diff, y_diff) in POSSIBLE_NEIGHBOURS_PAIRS.iter() {
        let is_alive = is_cell_alive(map, (x + x_diff) % 64, (y + y_diff) % 48);

        live_neighbour_count += is_alive as u8
    }

    if map.get((x, y)) {
        match live_neighbour_count {
            0 | 1 => false,      // Starvation
            n if n > 3 => false, // Overpopulation
            _ => true,
        }
    } else {
        live_neighbour_count == 3 // Reproduction
    }
}

fn update_life(map: &mut GameMap, scratchpad: &mut GameMap) {
    scratchpad.copy_from_slice(map);

    for x in COLUMNS {
        for y in ROWS {
            map[(y * 64) + x] = does_cell_live(&scratchpad, (x, y));
        }
    }
}

const COLOR_LINE: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const COLOR_ON: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

fn create_window() -> PistonWindow {
    WindowSettings::new("Life", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap()
}

fn draw_life(map: &GameMap, state: AppState, c: Context, g: &mut G2d) {
    for x in COLUMNS {
        let screen_x = x as f64;
        line(
            COLOR_LINE,
            0.5,
            [screen_x * 10.0, 0.0, screen_x * 10.0, 480.0],
            c.transform,
            g,
        );
    }
    for y in ROWS {
        let screen_y = y as f64;
        line(
            COLOR_LINE,
            0.5,
            [0.0, screen_y * 10.0, 640.0, screen_y * 10.0],
            c.transform,
            g,
        );
    }
    for x in COLUMNS {
        for y in ROWS {
            let screen_x = x as f64;
            let screen_y = y as f64;
            if is_cell_alive(map, x, y) {
                rectangle(
                    COLOR_ON,
                    [screen_x * 10.0, screen_y * 10.0, 9.0, 9.0],
                    c.transform,
                    g,
                );
            }
        }
    }
    if state.editing {
        let rect_color = if is_cell_alive(map, state.x, state.y) {
            [1.0, 0.8, 0.8, 1.0]
        } else {
            [1.0, 0.1, 0.1, 1.0]
        };
        rectangle(
            rect_color,
            [(state.x as f64) * 10.0, (state.y as f64) * 10.0, 9.0, 9.0],
            c.transform,
            g,
        );
    }
}

fn edit_life(map: &mut GameMap, state: AppState) {
    if state.down {
        map[(state.y * WIDTH) + state.x] = !map[(state.y * WIDTH) + state.x];
    }
}

fn get_new_state_from_input(app: AppState, inp: Input) -> Option<AppState> {
    match inp {
        Input::Move(Motion::MouseCursor([x, y])) => Some(AppState {
            x: (x / 10.0).floor() as usize % WIDTH,
            y: (y / 10.0).floor() as usize % HEIGHT,
            ..app
        }),
        Input::Button(ButtonArgs { button, state, .. }) => {
            match (button, state == ButtonState::Press) {
                (Button::Keyboard(Key::Space), true) => Some(AppState {
                    editing: !app.editing,
                    ..app
                }),
                (Button::Mouse(MouseButton::Left), down) => Some(AppState { down, ..app }),
                _ => None,
            }
        }
        _ => None,
    }
}

fn main() {
    let mut window: PistonWindow = create_window();
    window.set_max_fps(FPS);
    // The game level
    let mut map = fill_map();
    // Every frame needs a vector to retrieve the old positions -- clone the current one
    let mut map_scratchpad = map.clone();
    let mut state = AppState {
        ..Default::default()
    };

    while let Some(e) = window.next() {
        if let Event::Input(e, _) = e {
            if let Some(new_state) = get_new_state_from_input(state, e) {
                if state.editing {
                    edit_life(&mut map, state);
                }
                state = new_state;
            }
        } else {
            window.draw_2d(&e, |c, mut g, _| {
                clear([1.0; 4], g);
                draw_life(&map, state, c, &mut g);
                if !state.editing {
                    update_life(&mut map, &mut map_scratchpad);
                }
            });
        }
    }
}
