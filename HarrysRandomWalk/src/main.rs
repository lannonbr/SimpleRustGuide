extern crate piston_window;
extern crate image as im;
extern crate vecmath;
extern crate rand;

use im::GenericImage;
use piston_window::*;
use vecmath::*;
use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const MAX: u32 = 100;
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

        if (new_pos.0 == win_pos.0 -1) && (new_pos.1 == win_pos.1 -1) {
            println!("Done!");
            std::process::exit(0);
        }

        // 0 = nothing
        // 1 = -1
        // 2 = +1
        let dx: u32 = rng.gen_range(0, 3);
        let dy: u32 = rng.gen_range(0, 3);

        if dx == 1 && last_pos.0 > 0  {
            last_pos.0 = last_pos.0 -1;
        } else if dx == 2 && last_pos.0 < MAX-1 { //MAX - 1 is due to an off-by-one error.
            last_pos.0 = last_pos.0 +1;
        }

        if dy == 1 && last_pos.1 > 0{
            last_pos.1 = last_pos.1 -1;
        } else if dy == 2 && last_pos.1 < MAX-1 {
            last_pos.1 = last_pos.1 +1;
        }

        new_pos = (last_pos.0, last_pos.1);

        println!("{}, {}", new_pos.0,new_pos.1);

        let r = rng.gen_range(1,255);
        let g = rng.gen_range(1,255);
        let b = rng.gen_range(1,255);

        canvas.put_pixel(new_pos.0, new_pos.1, im::Rgba([r, g, b, 255]));
        texture.update(&mut* e.factory.borrow_mut(), &canvas).unwrap();

        last_pos = new_pos;
    }
}
