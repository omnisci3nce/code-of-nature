use macroquad::prelude::*;

struct Walker {
    x: f32,
    y: f32,
}

#[macroquad::main("Chapter 1")]
async fn main() {
    let w = screen_width() as usize;
    let h = screen_height() as usize;

    let mut framebuffer = Image::gen_image_color(w as u16, h as u16, BLACK);
    let texture = Texture2D::from_image(&framebuffer);

    loop {
        clear_background(BLACK);

        for i in 0..100 {
            for j in 0..100 {
                framebuffer.set_pixel(i, j, RED);
            }
        }

        texture.update(&framebuffer);
        draw_texture(&texture, 0., 0., WHITE);

        next_frame().await
    }
}
