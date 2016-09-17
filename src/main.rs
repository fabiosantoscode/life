
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

    return ret;
}

fn is_cell_live(map: &mut Vec<bool>, x: usize, y: usize) -> bool {
    return map[(y * 64) + x];
}

fn does_cell_live(map: &mut Vec<bool>, x: usize, y: usize) -> bool {
    let mut live_neighbour_count = 0;
    if x > 0 && is_cell_live(map, x - 1, y) {
        live_neighbour_count += 1;
    }
    if y > 0 && is_cell_live(map, x, y - 1) {
        live_neighbour_count += 1;
    }
    if y > 0 && x > 0 && is_cell_live(map, x - 1, y - 1) {
        live_neighbour_count += 1;
    }
    if x < 63 && is_cell_live(map, x + 1, y) {
        live_neighbour_count += 1;
    }
    if y < 47 && is_cell_live(map, x, y + 1) {
        live_neighbour_count += 1;
    }
    if y < 47 && x < 63 && is_cell_live(map, x + 1, y + 1) {
        live_neighbour_count += 1;
    }
    if y < 47 && x > 0 && is_cell_live(map, x - 1, y + 1) {
        live_neighbour_count += 1;
    }
    if x < 63 && y > 0 && is_cell_live(map, x + 1, y - 1) {
        live_neighbour_count += 1;
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
        for y in 0..48 {
            let screen_x = x as f64;
            let screen_y = y as f64;
            let on = map[(y * 64) + x];
            if on {
                rectangle([1.0, 0.0, 0.0, 1.0],
                          [screen_x * 10.0, screen_y * 10.0, 8.0, 8.0],
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
