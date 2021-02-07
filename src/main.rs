extern crate piston_window;

use piston_window::*;
use std::option::Option;

fn create_window() -> PistonWindow {
    return WindowSettings::new("Life", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
}

fn create_map() -> Vec<bool> {
    let mut ret = vec![false; 64 * 48];

    //  x
    //   x
    // xxx
    ret[1 + (64 * 2)] = true;
    ret[2 + (64 * 3)] = true;
    ret[0 + (64 * 4)] = true;
    ret[1 + (64 * 4)] = true;
    ret[2 + (64 * 4)] = true;

    //  x
    //   x
    // xxx
    ret[1 + (64 * 8)] = true;
    ret[2 + (64 * 9)] = true;
    ret[0 + (64 * 10)] = true;
    ret[1 + (64 * 10)] = true;
    ret[2 + (64 * 10)] = true;

    // xxx
    ret[180] = true;
    ret[181] = true;
    ret[182] = true;

    return ret;
}

fn is_cell_alive(map: &Vec<bool>, x: usize, y: usize) -> bool {
    return map[(y * 64) + x];
}

// added 64 and 48 respectively to avoid negative numbers when using modulo
const POSSIBLE_NEIGHBOURS_PAIRS: [[usize; 2]; 8] = [
    // top row
    [64 - 1, 48 - 1],
    [64 - 1, 48 + 0],
    [64 - 1, 48 + 1],
    // middle row (only two because 0,0 is the cell
    // itself, not a neighbour
    [64 + 0, 48 - 1],
    [64 + 0, 48 + 1],
    // bottom row
    [64 + 1, 48 - 1],
    [64 + 1, 48 + 0],
    [64 + 1, 48 + 1],
];

fn does_cell_live(map: &Vec<bool>, x: usize, y: usize) -> bool {
    let mut live_neighbour_count = 0;
    for [x_diff, y_diff] in POSSIBLE_NEIGHBOURS_PAIRS.iter() {
        let is_alive = is_cell_alive(map, (x + x_diff) % 64, (y + y_diff) % 48);

        live_neighbour_count += is_alive as u8
    }

    if is_cell_alive(map, x, y) {
        match live_neighbour_count {
            0 | 1 => false,      // Starvation
            n if n > 3 => false, // Overpopulation
            _ => true,
        }
    } else {
        live_neighbour_count == 3 // Reproduction
    }
}

fn update_life(map: &mut Vec<bool>, scratchpad: &mut Vec<bool>) {
    scratchpad.copy_from_slice(&map);

    for x in 0..64 {
        for y in 0..48 {
            map[(y * 64) + x] = does_cell_live(&scratchpad, x, y);
        }
    }
}

const COLOR_LINE: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const COLOR_ON: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

fn draw_life(map: &Vec<bool>, state: AppState, c: Context, g: &mut G2d) {
    for x in 0..64 {
        let screen_x = x as f64;
        line(
            COLOR_LINE,
            0.5,
            [screen_x * 10.0, 0.0, screen_x * 10.0, 480.0],
            c.transform,
            g,
        );
    }
    for y in 0..48 {
        let screen_y = y as f64;
        line(
            COLOR_LINE,
            0.5,
            [0.0, screen_y * 10.0, 640.0, screen_y * 10.0],
            c.transform,
            g,
        );
    }
    for x in 0..64 {
        for y in 0..48 {
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

#[derive(PartialEq, Clone, Copy)]
struct AppState {
    x: usize,
    y: usize,
    down: bool,
    editing: bool,
}

fn edit_life(map: &mut Vec<bool>, state: AppState) {
    if state.down {
        map[(state.y * 64) + state.x] = !map[(state.y * 64) + state.x];
    }
}

fn get_new_state_from_input(app: AppState, inp: Input) -> Option<AppState> {
    match inp {
        Input::Move(Motion::MouseCursor([x, y])) => Some(AppState {
            x: (x / 10.0).floor() as usize % 64,
            y: (y / 10.0).floor() as usize % 48,
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

// Don't go too fast
const FPS: u64 = 8;

fn main() {
    let mut window: PistonWindow = create_window();
    window.set_max_fps(FPS);
    // The game level
    let mut map = create_map();
    // Every frame needs a vector to retrieve the old positions -- clone the current one
    let mut map_scratchpad = map.clone();
    let mut state = AppState {
        x: 0,
        y: 0,
        down: false,
        editing: false,
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
