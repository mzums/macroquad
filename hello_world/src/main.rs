use macroquad::prelude::*;

#[macroquad::main("Hello world")]
async fn main() {
    let mut time: f32 = 0.0;
    let animation_speed = 2.0;

    let player_size = 50.0;
    let mut player_pos = Vec2::new(screen_width() / 2.0 + player_size, screen_height());
    let mut player_velocity = Vec2::ZERO;
    
    let move_speed = 5.0;
    let jump_force = -15.0;
    let gravity = 0.8;
    let ground_level = screen_height();
    let mut is_jumping = false;
    
    loop {
        clear_background(BLACK);

        player_velocity.x = 0.0;
        if is_key_down(KeyCode::Left) || is_key_down(KeyCode::A) {
            player_velocity.x = -move_speed;
        }
        if is_key_down(KeyCode::Right) || is_key_down(KeyCode::D) {
            player_velocity.x = move_speed;
        }
        if (is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space) || is_key_down(KeyCode::W)) && !is_jumping {
            player_velocity.y = jump_force;
            is_jumping = true;
        }
        
        player_velocity.y += gravity;
        player_pos += player_velocity;
        
        if player_pos.y > ground_level {
            player_pos.y = ground_level;
            player_velocity.y = 0.0;
            is_jumping = false;
        }
        
        player_pos.x = player_pos.x.clamp(0.0, screen_width() - player_size);
        
        draw_rectangle(player_pos.x, player_pos.y, player_size, player_size, RED);
        
        let text = "Hello Macroquad!";
        let font_size = 30.0;

        let dimensions = measure_text(text, None, font_size as u16, 1.0);
        
        let x = (screen_width() - dimensions.width) / 2.0;
        let y = screen_height() / 2.0 + dimensions.height / 4.0;
        
        draw_text(text, x, y, font_size, WHITE);

        let padding = 20.0 + 10.0 * (time * animation_speed).sin();
        
        draw_rectangle_lines(
            x - padding,
            y - dimensions.height - padding / 2.0,
            dimensions.width + 2.0 * padding,
            dimensions.height + padding,
            2.0,
            WHITE
        );
        
        time += get_frame_time();
        
        next_frame().await;
    }
}