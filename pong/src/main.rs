use macroquad::prelude::*;
use std::fs;

struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

struct Paddle {
    rect: Rect,
    speed: f32,
}

const PADDLE_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 700.0;

fn reset_ball(ball_speed: f32) -> Ball {
    Ball {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        vel: vec2(rand::gen_range(-1.0, 1.0), -1.0).normalize() * ball_speed,
        radius: 20.0,
    }
}

fn save_high_score(score: i32) -> std::io::Result<()> {
    fs::write("highscore.txt", score.to_string())
}

#[macroquad::main("pong")]
async fn main() {
    let mut paddle_speed = 550.0;
    let mut ball_speed = 700.0;

    let mut paddle = Paddle {
        rect: Rect::new(screen_width() / 2.0, 2.0 * screen_height() - 200.0, 150.0, 20.0),
        speed: paddle_speed,
    };
    let mut ball = reset_ball(ball_speed);
    let mut score = 0;
    let mut game_over = false;

    let mut high_score = fs::read_to_string("highscore.txt")
        .ok()
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);

    loop {
        clear_background(BLACK);

        if !game_over {
            if is_key_down(KeyCode::Left) {
                paddle.rect.x -= paddle.speed * get_frame_time();
            }
            if is_key_down(KeyCode::Right) {
                paddle.rect.x += paddle.speed * get_frame_time();
            }
            paddle.rect.x = paddle.rect.x.clamp(0.0, screen_width() - paddle.rect.w);
            ball.pos += ball.vel * get_frame_time();

            if ball.pos.x < ball.radius || ball.pos.x > screen_width() - ball.radius {
                ball.vel.x *= -1.0;
            }
            if ball.pos.y < ball.radius {
                ball.vel.y *= -1.0;
            }

            if ball.vel.y > 0.0 {
                let ball_rect = Rect::new(
                    ball.pos.x - ball.radius,
                    ball.pos.y - ball.radius,
                    ball.radius * 2.0,
                    ball.radius * 2.0,
                );
                
                if let Some(_intersection) = ball_rect.intersect(paddle.rect) {
                    let hit_pos = (ball.pos.x - paddle.rect.x) / paddle.rect.w;
                    let angle = (hit_pos * 2.0 - 1.0) * 1.2;
                    
                    ball.vel.x = angle * ball_speed;
                    ball.vel.y = -ball.vel.y.abs();

                    score += 1;
                    paddle_speed += 5.0;
                    ball_speed += 5.0;
                    
                    ball.pos.y = paddle.rect.y - ball.radius;
                }
            }
            if ball.pos.y > screen_height() - ball.radius {
                game_over = true;
            }
        }
        else {
            draw_text("GAME OVER", screen_width()/2.0 - 100.0, screen_height()/2.0 - 30.0, 50.0, RED);
            draw_text("Press SPACE to restart", screen_width()/2.0 - 150.0, screen_height()/2.0 + 30.0, 30.0, WHITE);

            if is_key_down(KeyCode::Space) {
                game_over = false;
                ball = reset_ball(ball_speed);
                ball_speed = BALL_SPEED;
                paddle_speed = PADDLE_SPEED;
            }

            if score > high_score {
                if let Err(e) = save_high_score(score) {
                    eprintln!("High score save failed: {}", e);
                }
                high_score = score;
            }
        }
        
        draw_circle(ball.pos.x, ball.pos.y, ball.radius, RED);
        draw_rectangle(paddle.rect.x, paddle.rect.y, paddle.rect.w, paddle.rect.h, WHITE);

        draw_text(&format!("Score: {}", score), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("High Score: {}", high_score), 20.0, 60.0, 30.0, WHITE);

        next_frame().await;
    }
}
