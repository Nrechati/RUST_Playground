use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn set_color_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn xy_from_ij(i: i64, j: i64) -> (i64, i64) {
    let x = (i - (WIDTH as i64 / 2)) as i64;
    let y = (j - (HEIGHT as i64 / 2)) as i64;
    (x, y)
}

#[allow(dead_code)]
fn euclidian_norm(x: i32, y: i32) -> f64 {
    ((x.pow(2) + y.pow(2)) as f64).sqrt()
}

fn main() {
    let mut buffer: Vec<u32> = vec![set_color_rgb(0, 127, 255); WIDTH * HEIGHT];
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = set_color_rgb(0, 127, 255);

            // for i in 0..WIDTH {
            //     for j in 0..HEIGHT {
            //         let k = i + (j * WIDTH);
            //         // println!("(x,y) = ({} , {}) => k = {}", x, y, k);
            //         buffer[k] = set_color_rgb(0, 127, 255);
            //         // let (x, y) = xy_from_ij(i as i64, j as i64);
            //         // if x != 0 && y != 0 && x % 2 == 0 && y % 2 == 0 {
            //         //     let k = i + (j * WIDTH);
            //         //     println!("(x,y) = ({} , {}) => k = {}", x, y, k);
            //         //     buffer[k] = set_color_rgb(0, 127, 255);
            //         // }
            //     }
        }
    }
    window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
}
