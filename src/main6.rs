use macroquad::prelude::*;

const PLAYER_WIDTH: f32 = 96.0;
const PLAYER_HEIGHT: f32 = 56.0;
const PLAYER_SPEED: f32 = 3.0;

const VIRTUAL_WIDTH: f32 = 1280.0;
const VIRTUAL_HEIGHT: f32 = 720.0;

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;

const BG_SCALE: f32 = 4.0;

#[macroquad::main("Zaxxon")]
async fn main() {
    let mut player_rect = Rect::new(
        screen_width() / 2.0 - PLAYER_WIDTH / 2.0,
        screen_height() - 100.0,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    );

    //Sprites
    let texture = load_texture("assets/Zaxxon-Miscellaneous-General-Sprites.png")
        .await
        .unwrap();
    texture.set_filter(FilterMode::Nearest);

    //Background
    let background = load_texture("assets/Zaxxon-Background.png").await.unwrap();
    background.set_filter(FilterMode::Nearest);

    enum PlayerDirection {
        Up,
        UpLeft,
        Left,
        DownLeft,
        Down,
        DownRight,
        Right,
        UpRight,
        Idle,
    }

    let render_target = render_target(GAME_WIDTH as u32, GAME_HEIGHT as u32);

    render_target.texture.set_filter(FilterMode::Nearest);

    //Background

    let bg_x = -500.0 * BG_SCALE;
    let bg_y = -1375.0 * BG_SCALE;

    loop {
        let scale_x = screen_width() / VIRTUAL_WIDTH;
        let scale_y = screen_height() / VIRTUAL_HEIGHT;

        let _scale = scale_x.min(scale_y);

        //let player_height = (-player_rect.y / 100.0).clamp(0.0, 100.0);
        let mut player_attitude = 0.0

        // CAMERA POSITION

        let cam_x = player_rect.x + player_rect.w / 2.0;
        let cam_y = player_rect.y + player_rect.h / 2.0;

        // VELOCITY
        let mut vel_x: f32 = 0.0;
        let mut vel_y: f32 = 0.0;

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),

            zoom: vec2(2.0 / GAME_WIDTH, 2.0 / GAME_HEIGHT),

            target: vec2(cam_x, cam_y),

            ..Default::default()
        });

        // =========================
        // RENDER INS SPIEL
        // =========================

        set_camera(&Camera2D {
            render_target: Some(render_target.clone()),

            zoom: vec2(2.0 / GAME_WIDTH, 2.0 / GAME_HEIGHT),

            target: vec2(cam_x, cam_y),

            ..Default::default()
        });

        clear_background(BLACK);

        //Background
        draw_texture_ex(
            &background,
            bg_x,
            bg_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                    background.width() * BG_SCALE,
                    background.height() * BG_SCALE,
                )),
                ..Default::default()
            },
        );

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 40.0, 20.0, DARKGRAY);

        let old_x = player_rect.x;
        let old_y = player_rect.y;
        // INPUT
        if is_key_down(KeyCode::A) {
            vel_x -= PLAYER_SPEED;
            vel_y -= PLAYER_SPEED * 0.5;
        }

        if is_key_down(KeyCode::D) {
            vel_x += PLAYER_SPEED;
            vel_y += PLAYER_SPEED * 0.5;
        }

        if is_key_down(KeyCode::W) {
            vel_y -= PLAYER_SPEED;
        }

        if is_key_down(KeyCode::S) {
            vel_y += PLAYER_SPEED;
        }

        player_rect.x += vel_x;
        player_rect.y += vel_y;

        //STATE
        let up = is_key_down(KeyCode::W);
        let down = is_key_down(KeyCode::S);
        let left = is_key_down(KeyCode::A);
        let right = is_key_down(KeyCode::D);

        let direction = match (up, down, left, right) {
            (true, false, true, false) => PlayerDirection::UpLeft,
            (true, false, false, true) => PlayerDirection::UpRight,
            (false, true, true, false) => PlayerDirection::DownLeft,
            (false, true, false, true) => PlayerDirection::DownRight,
            (true, false, false, false) => PlayerDirection::Up,
            (false, true, false, false) => PlayerDirection::Down,
            (false, false, true, false) => PlayerDirection::Left,
            (false, false, false, true) => PlayerDirection::Right,

            _ => PlayerDirection::Idle,
        };

        let sprite_source = match direction {
            PlayerDirection::UpLeft => Rect::new(160.0, 16.0, 17.0, 15.0),

            PlayerDirection::UpRight => Rect::new(8.0, 18.0, 24.0, 14.0),

            PlayerDirection::DownLeft => Rect::new(264.0, 18.0, 19.0, 13.0),

            PlayerDirection::DownRight => Rect::new(232.0, 17.0, 22.0, 14.0),

            PlayerDirection::Up => Rect::new(128.0, 13.0, 19.0, 19.0),

            PlayerDirection::Down => Rect::new(232.0, 17.0, 22.0, 14.0),

            PlayerDirection::Left => Rect::new(8.0, 18.0, 24.0, 14.0),

            PlayerDirection::Right => Rect::new(8.0, 18.0, 24.0, 14.0),

            _ => Rect::new(8.0, 18.0, 24.0, 14.0),
        };

        draw_texture_ex(
            &texture,
            player_rect.x,
            player_rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(sprite_source),
                dest_size: Some(Vec2::new(sprite_source.w * 4.0, sprite_source.h * 4.0)),
                ..Default::default()
            },
        );

        // Y GRENZEN

        if player_rect.y < -20000.0 {
            player_rect.y = -20000.0;
        }

        if player_rect.y > 20000.0 {
            player_rect.y = 20000.0;
        }

        // DIAGONALE FLUGZONE
        // MAP: rechts unten -> links oben

        let center_x = -player_rect.y * 2.0;

        //let corridor_width = 5000.0;

        //let min_x = center_x - corridor_width * 0.5;
        //let max_x = center_x + corridor_width * 0.5;
        let min_x = center_x - -850.0;
        let max_x = center_x + 2500.0;

        if player_rect.x < min_x {
            player_rect.x = old_x;
            player_rect.y = old_y;
        }

        if player_rect.x + player_rect.w > max_x {
            player_rect.x = old_x;
            player_rect.y = old_y;
        }
        if player_rect.x < min_x {
            player_rect.x = min_x;
            //vel_x = 0.0;
        }

        if player_rect.x + player_rect.w > max_x {
            player_rect.x = max_x - player_rect.w;
            //vel_x = 0.0;
        }

        // =========================
        // UI
        // =========================

        // FPS

        draw_text(
            &format!("FPS: {}", get_fps()),
            cam_x - GAME_WIDTH * 0.48,
            cam_y - GAME_HEIGHT * 0.45,
            30.0,
            GREEN,
        );

        // HEIGHT BAR

        let bar_x = cam_x - GAME_WIDTH * 0.48;
        let bar_y = cam_y - GAME_HEIGHT * 0.35;

        let bar_width = 24.0;
        let bar_height = 220.0;

        // OUTLINE

        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 4.0, WHITE);

        // FILL

        let fill_height = (player_height / 100.0) * bar_height;

        draw_rectangle(
            bar_x,
            bar_y + (bar_height - fill_height),
            bar_width,
            fill_height,
            GREEN,
        );

        // TEXT

        draw_text("ALT", bar_x - 8.0, bar_y - 10.0, 28.0, WHITE);

        // =========================
        // ZURÜCK ZUM BILDSCHIRM
        // =========================

        set_default_camera();

        clear_background(BLACK);

        let scale = f32::min(screen_width() / GAME_WIDTH, screen_height() / GAME_HEIGHT);

        let draw_width = GAME_WIDTH * scale;
        let draw_height = GAME_HEIGHT * scale;

        let offset_x = (screen_width() - draw_width) * 0.5;
        let offset_y = (screen_height() - draw_height) * 0.5;

        draw_texture_ex(
            &render_target.texture,
            offset_x,
            offset_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(draw_width, draw_height)),
                //flip_y: false,
                ..Default::default()
            },
        );
        next_frame().await;
    }
}
