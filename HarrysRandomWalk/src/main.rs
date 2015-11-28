extern crate piston_window;
extern crate image as im;
extern crate vecmath;
extern crate rand;

use im::GenericImage;
use piston_window::*;
use vecmath::*;

use rand::{thread_rng, Rng};

fn clamp(x: u32, min: u32, max: u32) -> u32 {
    let mut result: u32 = x.clone();

    if result < min {
        result = min;
    } else if result > max {
        result = max;
    }

    result
}


fn main() {
    let mut rng = thread_rng();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const MAX: u32 = 20;
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

        // 0 = nothing
        // 1 = +1
        // 2 = -1
        let dx: u32 = rng.gen_range(0, 3);
        let dy: u32 = rng.gen_range(0, 3);
        println!("dx:{},\tdy:{}", dx, dy);

        if dx == 0 && last_pos.0 != 0  {
            last_pos.0 = last_pos.0 -1;
        } else if dx == 1 && last_pos.0 != MAX {
            last_pos.0 = last_pos.0 +1;
        }

        if dy == 0 && last_pos.1 != 0{
            last_pos.1 = last_pos.1 -1;
        } else if dy == 1 && last_pos.1 != MAX {
            last_pos.1 = last_pos.1 +1;
        }

        new_pos = (clamp(last_pos.0, 0, MAX), clamp(last_pos.1, 0, MAX));

        println!("{}, {}", new_pos.0,new_pos.1);

        canvas.put_pixel(new_pos.0, new_pos.1, im::Rgba([255, 0, 0, 255]));
        texture.update(&mut* e.factory.borrow_mut(), &canvas).unwrap();

        last_pos = new_pos;
    }
}
