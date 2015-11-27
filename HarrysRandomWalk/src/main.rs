extern crate piston_window;
extern crate image as im;
extern crate vecmath;
extern crate rand;

use im::GenericImage;
use piston_window::*;
use vecmath::*;

use rand::{thread_rng, Rng};

fn clamp(x: u32, min: u32, max: u32) -> u32 {
    if x < min {
        x = min;
    } else if x > max {
        x = max;
    }

    x as u32;
}

fn main() {
    let mut rng = thread_rng();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const MAX: u32 = 300;
    const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
    const win_pos:(u32, u32) = (MAX, MAX);

    let (width, height) = (MAX, MAX);

    let start_pos: (u32, u32) = (0,0);
    let mut last_pos: (u32, u32) = (0,0);
    let mut new_pos: (u32, u32) = (0,0);

    // Create a window
    let window: PistonWindow =
        WindowSettings::new("Harry's Random Walk", (width, height))
            .exit_on_esc(true)
            .opengl(opengl)
            .build()
            .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let mut texture = Texture::from_image(
        &mut *window.factory.borrow_mut(),
        &canvas,
        &TextureSettings::new()
     ).unwrap();

    last_pos = start_pos;

    for e in window {
        e.draw_2d(|c, g| {
            clear(WHITE, g);
            image(&texture, c.transform, g);
        });

        if new_pos == win_pos {
            println!("Done!");
            std::process::exit(1);
        }

        let dx: i32 = rng.gen_range(-1, 2);
        let dy: i32 = rng.gen_range(-1, 2);
        println!("{}, {}", dx,dy);

        new_pos = (clamp(last_pos.0 + dx, 0, MAX), clamp(last_pos.1 + dy, 0, MAX));

        canvas.put_pixel(new_pos.0, new_pos.1, im::Rgba([255, 0, 0, 255]));
        texture.update(&mut* e.factory.borrow_mut(), &canvas).unwrap();

        last_pos = new_pos;
    }
}
