use macroquad::prelude::*;


const PLAYER_WIDTH: f32 = 96.0;
const PLAYER_HEIGHT: f32 = 56.0;
const PLAYER_SPEED: f32 = 3.0;

#[macroquad::main("Zaxxon")]
async fn main() {
    let mut player_rect = Rect::new(
        screen_width() / 2.0 - PLAYER_WIDTH / 2.0,
        screen_height() - 100.0,
        PLAYER_WIDTH,
        PLAYER_HEIGHT,
    );

    let mut player_state = 0; // 0 = normal, 1 = W gedrückt
    //Sprites
    let texture = load_texture("assets/Zaxxon-Miscellaneous-General-Sprites.png").await.unwrap();
    //texture.set_filter(FilterMode::Nearest);
    
    //Background 
    let background = load_texture("assets/Zaxxon-Background.png").await.unwrap();
    texture.set_filter(FilterMode::Nearest);

    loop {
        //clear_background(SKYBLUE);
        //draw_texture(&texture, 100.0, 100.0, WHITE);
        
        //Background
        draw_texture_ex(
            &background,
            12630.0,
            0.0,
            WHITE,
            DrawTextureParams {
            //dest_size: Some(Vec2::new(5000.0, 5000.0)),
            ..Default::default()
            },    
        ); 

        //Player
        draw_texture_ex(
            &texture,
            player_rect.x,
            player_rect.y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(8.0, 18.0, 24.0, 14.0)),
                dest_size: Some(Vec2::new(96.0, 56.0)),
                ..Default::default()
            },
        );
        

        draw_text(&format!("FPS: {}", get_fps()), 20.0, 40.0, 20.0, DARKGRAY);

        // Bewegung
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
            player_state = 1;   
        
        } else {
            player_state = 0;
        } 

        if is_key_down(KeyCode::S) {
            player_rect.y += PLAYER_SPEED;
        }


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
