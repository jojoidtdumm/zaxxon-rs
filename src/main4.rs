use macroquad::prelude::*;


const PLAYER_WIDTH: f32 = 96.0;
const PLAYER_HEIGHT: f32 = 56.0;
const PLAYER_SPEED: f32 = 2.5;

#[macroquad::main("Zaxxon")]
async fn main() {
    let mut player_rect = Rect::new(
        screen_width() / 2.0 - PLAYER_WIDTH / 2.0,
        screen_height() - 100.0,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    );

    //Sprites
    let texture = load_texture("assets/Zaxxon-Miscellaneous-General-Sprites.png").await.unwrap();
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
    
    //Background
    let scale = 5.0;

    let bg_x = -500.0 * scale;
    let bg_y = -1375.0 * scale;
    
    loop {
        
        //Background
        draw_texture_ex(
            &background,
            bg_x,
            bg_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(Vec2::new(
                background.width() * scale,
                    background.height() * scale,
                )),
                ..Default::default()
            },
        );



        // Player
        //
        //let source_rect = if player_state == 1 {
        //    Rect::new(128.0, 13.0, 19.0, 19.0)
        //} else {
        //    Rect::new(8.0, 18.0, 24.0, 14.0)
        //};

        //let dest_size = if player_state == 1 {
        //    Vec2::new(76.0, 76.0)
        //} else {
        //    Vec2::new(96.0, 56.0)
        //};

               

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 40.0, 20.0, DARKGRAY);

        // INPUT

        if is_key_down(KeyCode::A) {
            player_rect.x -= PLAYER_SPEED;
            player_rect.y -= PLAYER_SPEED * 0.5;
        }

        if is_key_down(KeyCode::D) {
            player_rect.x += PLAYER_SPEED;
            player_rect.y += PLAYER_SPEED * 0.5;
        }

        if is_key_down(KeyCode::W) {
            player_rect.y -= PLAYER_SPEED;
        } 

        if is_key_down(KeyCode::S) {
            player_rect.y += PLAYER_SPEED;
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
            PlayerDirection::UpLeft => {
                Rect::new(160.0, 16.0, 16.0, 15.0)
            }
            
            PlayerDirection::UpRight => {
                Rect::new(8.0, 18.0, 24.0, 14.0)
            }

            PlayerDirection::DownLeft => {
                Rect::new(264.0, 18.0, 19.0, 13.0)
            }

            PlayerDirection::DownRight => {
                Rect::new(232.0, 17.0, 22.0, 14.0)
            }

            PlayerDirection::Up  => {
                Rect::new(128.0, 13.0, 19.0, 19.0)
            }    

            PlayerDirection::Down => {
                Rect::new(232.0, 17.0, 22.0, 14.0)
            }
            
            PlayerDirection::Left => {
                Rect::new(8.0, 18.0, 24.0, 14.0)
            }
            
            PlayerDirection::Right => {
                Rect::new(8.0, 18.0, 24.0, 14.0)
            }
             
            _ => {
                Rect::new(8.0, 18.0, 24.0, 14.0)
            }

        };

        draw_texture_ex(
            &texture,
            player_rect.x,
            player_rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(sprite_source),
                dest_size: Some(Vec2::new(
                    sprite_source.w * 4.0,
                    sprite_source.h * 4.0,
                )),
                ..Default::default()
            },
        );











        
        //Bildschirmgrenze links
        if player_rect.x < 0.0 {
            player_rect.x = 0.0;
        }
    
        //Bildschirmgrenze oben
        if player_rect.y < 0.0 { 
            player_rect.y = 0.0;
        }

        
        //Bildschirmgrenze rechts
        if player_rect.x + player_rect.w > screen_width() {
            player_rect.x = screen_width() - player_rect.w;
        }



        //Bildschirmgrenze unten
        if player_rect.y + player_rect.h > screen_height() {
            player_rect.y = screen_height() - player_rect.h;
        }

        // Spieler zeichnen
        //draw_rectangle(
        //    player_rect.x,
        //    player_rect.y,
        //    player_rect.w,
        //    player_rect.h,
        //    DARKGRAY,
        //);

        next_frame().await;
    }
}
