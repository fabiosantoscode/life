
extern crate piston_window;

use std::option::Option;
use piston_window::*;

fn create_window() -> PistonWindow {
    return WindowSettings::new("Life", [640, 480])
        .exit_on_esc(true)
        .build()
        .unwrap();
}

fn create_map() -> Vec<bool> {
    let mut ret = vec!(false; 64 * 48);

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

    ret[180] = true;
    ret[181] = true;
    ret[182] = true;

    return ret;
}

fn is_cell_live(map: &mut Vec<bool>, x: usize, y: usize) -> bool {
    return map[(y * 64) + x];
}

// added 64 and 48 respectively to avoid negative numbers when using modulo
const POSSIBLE_NEIGHBOURS_PAIRS: [[usize; 2]; 8] = [// top row
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
                                                    [64 + 1, 48 + 1]];

fn does_cell_live(map: &mut Vec<bool>, x: usize, y: usize) -> bool {
    let mut live_neighbour_count = 0;
    for n in POSSIBLE_NEIGHBOURS_PAIRS.iter() {
        let (x_diff, y_diff) = (n[0], n[1]);
        if is_cell_live(map, (x + x_diff) % 64, (y + y_diff) % 48) {
            live_neighbour_count += 1;
        }
    }
    if is_cell_live(map, x, y) {
        if live_neighbour_count < 2 {
            return false;  // starvation
        }
        if live_neighbour_count > 3 {
            return false;  // overpopulation
        }
        return true;
    } else {
        if live_neighbour_count == 3 {
            return true; // reproduction
        }
        return false;
    }
}

fn update_life(map: &mut Vec<bool>) {
    let mut cpy = vec![true;64 * 48];
    cpy.copy_from_slice(map);
    for x in 0..64 {
        for y in 0..48 {
            map[(y * 64) + x] = does_cell_live(&mut cpy, x, y);
        }
    }
}

const GRAY: f32 = 0.80;

fn draw_life(map: &mut Vec<bool>, state: AppState, c: Context, g: &mut G2d) {
    for x in 0..64 {
        let screen_x = x as f64;
        line([GRAY, GRAY, GRAY, 1.0],
             0.5,
             [screen_x * 10.0, 0.0, screen_x * 10.0, 480.0],
             c.transform,
             g);
    }
    for y in 0..48 {
        let screen_y = y as f64;
        line([GRAY, GRAY, GRAY, 1.0],
             0.5,
             [0.0, screen_y * 10.0, 640.0, screen_y * 10.0],
             c.transform,
             g);
    }
    for x in 0..64 {
        for y in 0..48 {
            let screen_x = x as f64;
            let screen_y = y as f64;
            let on = map[(y * 64) + x];
            if on {
                rectangle([1.0, 0.0, 0.0, 1.0],
                          [screen_x * 10.0, screen_y * 10.0, 9.0, 9.0],
                          c.transform,
                          g);
            }
        }
    }
    if let AppMode::Editing = state.mode {
        let rect_color = if map[(state.y * 64) + state.x] {
            [1.0, 0.8, 0.8, 1.0]
        } else {
            [1.0, 0.1, 0.1, 1.0]
        };
        rectangle(rect_color,
                  [(state.x as f64) * 10.0, (state.y as f64) * 10.0, 9.0, 9.0],
                  c.transform,
                  g);
    }
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
enum AppMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Copy)]
struct AppState {
    x: usize,
    y: usize,
    down: bool,
    mode: AppMode,
}

fn edit_life(state: AppState, map: &mut Vec<bool>) {
    if state.down {
        map[(state.y * 64) + state.x] = !map[(state.y * 64) + state.x];
    }
}

fn toggle_app_state(state: AppState) -> AppState {
    let new_mode = if state.mode == AppMode::Normal {
        AppMode::Editing
    } else {
        AppMode::Normal
    };

    return AppState {
        x: state.x,
        y: state.y,
        down: state.down,
        mode: new_mode,
    };
}

fn update_state_from_input(inp: Input, state: AppState) -> Option<AppState> {
    return match inp {
        Input::Press(Button::Keyboard(Key::Space)) => Some(toggle_app_state(state)),
        Input::Move(Motion::MouseCursor(mouse_x, mouse_y)) => {
            Some(AppState {
                x: (mouse_x / 10.0).floor() as usize % 64,
                y: (mouse_y / 10.0).floor() as usize % 48,
                down: state.down,
                mode: state.mode,
            })
        }
        Input::Press(Button::Mouse(MouseButton::Left)) => {
            Some(AppState {
                x: state.x,
                y: state.y,
                down: true,
                mode: state.mode,
            })
        }
        Input::Release(Button::Mouse(MouseButton::Left)) => {
            Some(AppState {
                x: state.x,
                y: state.y,
                down: false,
                mode: state.mode,
            })
        }
        _ => None,
    };
}

fn main() {
    let mut window: PistonWindow = create_window();
    window.set_max_fps(8);
    let mut map = create_map();
    let mut state = AppState {
        x: 0,
        y: 0,
        down: false,
        mode: AppMode::Normal,
    };
    while let Some(e) = window.next() {
        if let Event::Input(e) = e {
            if let Some(new_state) = update_state_from_input(e, state) {
                if let AppMode::Editing = state.mode {
                    edit_life(state, &mut map);
                }
                state = new_state;
            }
        } else {
            window.draw_2d(&e, |c, mut g| {
                clear([1.0; 4], g);
                draw_life(&mut map, state, c, &mut g);
                if let AppMode::Normal = state.mode {
                    update_life(&mut map);
                }
            });
        }
    }
}
