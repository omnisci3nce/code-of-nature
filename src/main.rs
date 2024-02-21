use macroquad::prelude::*;

struct Walker {
    x: u32,
    y: u32,
}

#[macroquad::main("Chapter 1")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;
    let mut walker = Walker {
        x: w as u32 / 2,
        y: h as u32 / 2,
    };

    let mut framebuffer = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&framebuffer);

    loop {
        clear_background(BLACK);

        let choice = macroquad::rand::gen_range(0.0_f32, 4.0_f32).floor() as u32;
        match choice {
            0 => walker.x += 1,
            1 => walker.x -= 1,
            2 => walker.y += 1,
            3 => walker.y -= 1,
            _ => unreachable!(),
        };

        framebuffer.set_pixel(walker.x, walker.y, RED);

        texture.update(&framebuffer);
        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}
