use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);

#[macroquad::main("BasicShapes")]
async fn main() {
    let player_rect = Rect::new(
        screen_rect() * 0.5f32,
        screen_width() - 100f32,
        PLAYER_SIZE.x, 
        PLAYER_SIZE.y,
    );

    loop {
        clear_background(SKYBLUE);
        draw_rectangle(player_rect.x, player_rect.y, player_rect.w, player_rect.h, DARKGRAY);
        next_frame().await
    }
}
