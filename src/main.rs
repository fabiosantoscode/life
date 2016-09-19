
extern crate piston_window;

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
        if live_neighbour_count < 2 || live_neighbour_count > 3 {
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

fn draw_life(map: &mut Vec<bool>, c: Context, g: &mut G2d) {
    for x in 0..64 {
        let screen_x = x as f64;
        line([0.95, 0.95, 0.95, 1.0],
             0.5,
             [screen_x * 10.0, 0.0, screen_x * 10.0, 480.0],
             c.transform,
             g);
    }
    for y in 0..64 {
        let screen_y = y as f64;
        line([0.95, 0.95, 0.95, 1.0],
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
}

fn main() {
    let mut window: PistonWindow = create_window();
    window.set_max_fps(8);
    let mut map = create_map();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, mut g| {
            clear([1.0; 4], g);
            draw_life(&mut map, c, &mut g);
            update_life(&mut map);
        });
    }
}
