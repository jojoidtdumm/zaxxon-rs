use macroquad::prelude::*;

const PLAYER_WIDTH: f32 = 96.0;
const PLAYER_HEIGHT: f32 = 56.0;
const PLAYER_SPEED: f32 = 10.0;

const GAME_WIDTH: f32 = 1280.0;
const GAME_HEIGHT: f32 = 720.0;

const MAX_ALTITUDE: f32 = 100.0;
const ALTITUDE_SPEED: f32 = 10.0;

const BG_SCALE: f32 = 4.0;

const AUTO_SCROLL_X: f32 = 1.8;
const AUTO_SCROLL_Y: f32 = -0.9;

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

    let mut player_altitude: f32 = 0.0;

    // VELOCITY
    let mut velocity = Vec2::ZERO;

    let shadow_source = Rect::new(352.0, 18.0, 22.0, 13.0);


    let mut world_offset = vec2(0.0, 0.0);



    loop {
        // CAMERA POSITION
        let cam_x = player_rect.x + player_rect.w / 2.0 + 220.0;
        // Kamera folgt der Höhe
        let cam_y = player_rect.y + player_rect.h / 2.0 - player_altitude * 0.6 - 100.0;

        // RENDER INS SPIEL


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
            bg_x - world_offset.x,
            bg_y - world_offset.y,
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

        let dt = get_frame_time();

        world_offset.x += AUTO_SCROLL_X * 60.0 * dt;
        world_offset.y += AUTO_SCROLL_Y * 60.0 * dt;

        // INPUT DIRECTION

        let mut move_dir = Vec2::ZERO;

        // LINKS

        if is_key_down(KeyCode::A) {
            move_dir.x -= 1.0;
            move_dir.y -= 0.5;
        }

        // RECHTS

        if is_key_down(KeyCode::D) {
            move_dir.x += 1.0;
            move_dir.y += 0.5;
        }

        // STEIGEN

        if is_key_down(KeyCode::W) {
            player_altitude += ALTITUDE_SPEED * dt;

            move_dir.y -= 0.8;
        }

        // SINKEN

        if is_key_down(KeyCode::S) {
            player_altitude -= ALTITUDE_SPEED * dt;

            move_dir.y += 0.5;
        }

        // NORMALIZE

        if move_dir.length() > 0.0 {
            move_dir = move_dir.normalize();
        }

        // AUTO SCROLL

        //let auto_scroll = vec2(AUTO_SCROLL_X, AUTO_SCROLL_Y);

        // TARGET VELOCITY

        //let target_velocity = auto_scroll + move_dir * PLAYER_SPEED;
        
        let target_velocity = move_dir * PLAYER_SPEED;

        // SMOOTH MOVEMENT

        velocity = velocity.lerp(target_velocity, 8.0 * dt);

        // MOVEMENT

        player_rect.x += velocity.x * 60.0 * dt;
        player_rect.y += velocity.y * 60.0* dt;


        // LANGSAMES SINKEN

        if !is_key_down(KeyCode::W) && !is_key_down(KeyCode::S) {
            player_altitude -= 0.08 * dt;
        }

        // ALTITUDE LIMIT

        player_altitude = player_altitude.clamp(0.0, MAX_ALTITUDE);

        // Y LIMITS

        player_rect.y = player_rect.y.clamp(-20000.0, 20000.0);

        // DIAGONALE FLUGZONE

        let center_x = -player_rect.y * 2.0;

        let min_x = center_x + 850.0;
        let max_x = center_x + 2500.0;

        // COLLISION

        if player_rect.x < min_x {
            player_rect.x = min_x;

            if velocity.x < 0.0 {
                velocity.x = 0.0;
            }
        }

        if player_rect.x + player_rect.w > max_x {
            player_rect.x = max_x - player_rect.w;

            if velocity.x > 0.0 {
                velocity.x = 0.0;
            }
        }

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

        // SHADOW
        let shadow_scale = 1.0 - player_altitude * 0.003;
        let shadow_offset_x = player_altitude * 0.25;
        let shadow_offset_y = player_altitude * 0.12;
        let base_x = player_rect.x;
        let base_y = player_rect.y;

        draw_texture_ex(
            &texture,
            base_x + shadow_offset_x,
            base_y + shadow_offset_y, 
            Color::new(0.0, 0.0, 0.0, 0.45),
            DrawTextureParams {
                source: Some(shadow_source),

                dest_size: Some(Vec2::new(
                    shadow_source.w * 4.0 * shadow_scale,
                    shadow_source.h * 4.0 * shadow_scale,
                )),

                ..Default::default()
            },
        );

        draw_texture_ex(
            &texture,
            player_rect.x,
            player_rect.y - player_altitude,
            WHITE,
            DrawTextureParams {
                source: Some(sprite_source),
                dest_size: Some(Vec2::new(sprite_source.w * 4.0, sprite_source.h * 4.0)),
                ..Default::default()
            },
        );

        // UI

        // FPS

        draw_text(
            &format!("FPS: {}", get_fps()),
            cam_x - GAME_WIDTH * 0.48,
            cam_y - GAME_HEIGHT * 0.45,
            30.0,
            WHITE,
        );

        // HEIGHT BAR

        let bar_x = cam_x - GAME_WIDTH * 0.48;
        let bar_y = cam_y - GAME_HEIGHT * 0.35;

        let bar_width = 24.0;
        let bar_height = 220.0;

        // OUTLINE

        draw_rectangle_lines(bar_x, bar_y, bar_width, bar_height, 4.0, WHITE);

        // FILL

        let fill_height = (player_altitude / 100.0) * bar_height;

        draw_rectangle(
            bar_x,
            bar_y + (bar_height - fill_height),
            bar_width,
            fill_height,
            GREEN,
        );

        // TEXT

        draw_text("ALT", bar_x - 8.0, bar_y - 10.0, 28.0, WHITE);

        // ZURÜCK ZUM BILDSCHIRM

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


