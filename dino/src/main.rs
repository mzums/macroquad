use macroquad::prelude::*;
use macroquad::rand::*;
use std::fs;

fn save_high_score(score: i32) -> std::io::Result<()> {
    fs::write("highscore.txt", score.to_string())
}

#[macroquad::main("dino")]
async fn main() {
    let mut player_y = screen_height() - 50.0;
    let mut player_velocity = 0.0;
    let player_size = 50.0;
    let ground_level = screen_height() - player_size;
    let mut is_jumping = false;

    let gravity = 0.8;
    let jump_force = -15.0;
    
    let obstacle_min = 20.0;
    let obstacle_max = 70.0;
    let obstacle_width = 30.0;
    let mut obstacle_speed = 8.0;
    let mut obstacles: Vec<(Rect, bool)> = Vec::new();
    let mut obstacle_timer = 0.0;
    let mut obstacle_interval = 2.0;
    let mut obstacle_interval_min = 0.7;
    let mut obstacle_interval_max = 2.5;
    
    let mut game_over = false;
    let mut score = 0;

    let mut high_score = fs::read_to_string("highscore.txt")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);

    loop {
        clear_background(BLACK);
        
        draw_line(
            0.0,
            ground_level + player_size,
            screen_width(),
            ground_level + player_size,
            2.0,
            WHITE
        );
        
        draw_rectangle(50.0, player_y, player_size, player_size, RED);
        
        draw_text(&format!("Score: {}", score), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("High Score: {}", high_score), 20.0, 60.0, 30.0, WHITE);
        
        if !game_over {
            if is_key_down(KeyCode::Up) || is_key_down(KeyCode::Space) {
                if !is_jumping {
                    is_jumping = true;
                    player_velocity = jump_force;
                }
            }

            player_velocity += gravity;
            player_y += player_velocity;

            if player_y > ground_level {
                player_y = ground_level;
                player_velocity = 0.0;
                is_jumping = false;
            }

            if player_y < 0.0 {
                player_y = 0.0;
                player_velocity = 0.0;
            }

            obstacle_timer += get_frame_time();
            if obstacle_timer >= obstacle_interval {
                obstacle_timer = 0.0;
                let obstacle_height = gen_range(obstacle_min, obstacle_max);
                obstacles.push((Rect::new(
                    screen_width(),
                    ground_level + player_size - obstacle_height,
                    obstacle_width,
                    obstacle_height,
                ),
                false)
            );
                if obstacle_interval_min > 0.5 {
                    obstacle_interval_min -= 0.01;
                }
                if obstacle_interval_max > 1.2 {
                    obstacle_interval_max -= 0.03;
                }
                obstacle_interval = rand::gen_range(obstacle_interval_min, obstacle_interval_max);
                obstacle_speed += 0.2;
            }

            for obstacle in obstacles.iter_mut() {
                obstacle.0.x -= obstacle_speed;
                draw_rectangle(obstacle.0.x, obstacle.0.y, obstacle.0.w, obstacle.0.h, GREEN);
                
                let player_rect = Rect::new(50.0, player_y, player_size, player_size);
                if player_rect.overlaps(&obstacle.0) {
                    game_over = true;
                }
                
                if obstacle.0.x + obstacle.0.w < 50.0 && !obstacle.1 {
                    score += 1;
                    obstacle.1 = true;
                }
            }

            obstacles.retain(|obstacle| obstacle.0.x + obstacle.0.w > 0.0);
            
            score += (get_frame_time() * 10.0) as i32;
        } else {
            draw_text("GAME OVER", screen_width()/2.0 - 100.0, screen_height()/2.0 - 30.0, 50.0, RED);
            draw_text("Press SPACE to restart", screen_width()/2.0 - 150.0, screen_height()/2.0 + 30.0, 30.0, WHITE);
            
            if score > high_score {
                if let Err(e) = save_high_score(score) {
                    eprintln!("High score save failed: {}", e);
                }
                high_score = score;
            }

            if is_key_down(KeyCode::Space) {
                game_over = false;
                player_y = ground_level;
                player_velocity = 0.0;
                obstacles.clear();
                score = 0;
                obstacle_timer = 0.0;
            }
        }

        next_frame().await;
    }
}