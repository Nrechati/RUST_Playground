use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;
const W2: usize = 320;
const H2: usize = 180;
const ZOOM: usize = 1;


fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn xy_from_ij(i: i64, j: i64) -> (i64, i64) {
    let x = (i - (WIDTH as i64 / 2)) as i64;
    let y = (j - (HEIGHT as i64 / 2)) as i64;
    (x, y)
}

fn mandelbrot(x:i64,y:i64) -> u32{
    let	mut i = 0;
	let	mut z_re = 0;
	let	mut z_im = 0;
	let	mut re_sq = 0;
	let	mut im_sq = 0;

	while i <= 100 && re_sq + im_sq <= 4 {
		re_sq = z_re * z_re;
		im_sq = z_im * z_im;
		z_im = (z_re + z_im) * (z_re + z_im) - re_sq - im_sq;
		z_im += y;
		z_re = re_sq - im_sq + x;
		i+=1;
	}

	if i <= 100 {
        return (1 - (i / 100)) * from_u8_rgb(0, 127, 255);
    }
    return from_u8_rgb(0, 0, 0)
}

fn compute_fractal() -> [u32; WIDTH*HEIGHT]{
    let mut buffer =  [from_u8_rgb(0,0,0); WIDTH * HEIGHT];
    // for i in 0..WIDTH{
    //     for j in 0..HEIGHT{
    //         buffer[i + WIDTH*j] = from_u8_rgb(0, 127, 255);
    //     }
    // }

    for i in 0..WIDTH {
        for j in 0..HEIGHT {
            let (x, y) = xy_from_ij(i as i64, j as i64);
            let k = i + (j * WIDTH);
            let re:i64 = 2 * (x - W2 as i64) / (ZOOM as i64 * W2 as i64);
            let im:i64 = 2 * (y - H2 as i64) / (ZOOM as i64 * H2 as i64);
            buffer[k] = mandelbrot(re,im);
            // println!("(x,y) = ({} , {}) => k = {}", x, y, k);
        }
    }

    buffer
}

fn main() {
    let mut window = Window::new(
        "Fract-ol",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    window.set_target_fps(60);

    let mut ticks:u16 = 0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        ticks += 1;

        if ticks < 15 {
            window.update();
        }else{
            ticks = 0;
            let buffer: [u32; WIDTH * HEIGHT] = compute_fractal();
            window
                .update_with_buffer(&buffer, WIDTH, HEIGHT)
                .unwrap();
        }
    }
}
