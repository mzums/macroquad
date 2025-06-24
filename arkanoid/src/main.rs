use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 150.0;
const PADDLE_HEIGHT: f32 = 20.0;
const PADDLE_SPEED: f32 = 10.0;
const BALL_SIZE: f32 = 15.0;
const BALL_SPEED: f32 = 5.0;

struct Paddle {
    rect: Rect,
}

impl Paddle {
    fn new() -> Self {
        Self {
            rect: Rect::new(
                WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0,
                WINDOW_HEIGHT - 50.0,
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
        }
    }

    fn update(&mut self) {
        let x_move = match (is_key_down(KeyCode::Left), is_key_down(KeyCode::Right)) {
            (true, false) => -PADDLE_SPEED,
            (false, true) => PADDLE_SPEED,
            _ => 0.0,
        };

        self.rect.x += x_move;
        self.rect.x = self.rect.x.clamp(0.0, WINDOW_WIDTH - self.rect.w);
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    fn new() -> Self {
        Self {
            rect: Rect::new(
                WINDOW_WIDTH / 2.0 - BALL_SIZE / 2.0,
                WINDOW_HEIGHT / 2.0 - BALL_SIZE / 2.0,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: vec2(0.5, -1.0).normalize() * BALL_SPEED,
        }
    }

    fn update(&mut self) {
        self.rect.x += self.vel.x;
        self.rect.y += self.vel.y;

        if self.rect.x < 0.0 || self.rect.x > WINDOW_WIDTH - self.rect.w {
            self.vel.x *= -1.0;
        }
        if self.rect.y < 0.0 {
            self.vel.y *= -1.0;
        }
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }
}

#[macroquad::main("Arkanoid")]
async fn main() {
    let mut paddle = Paddle::new();
    let mut ball = Ball::new();

    loop {
        paddle.update();
        ball.update();

        if ball.rect.overlaps(&paddle.rect) {
            ball.vel.y = -ball.vel.y.abs();
        }

        clear_background(BLACK);
        
        paddle.draw();
        ball.draw();
        
        draw_line(0.0, 0.0, WINDOW_WIDTH, 0.0, 2.0, WHITE);
        draw_line(0.0, 0.0, 0.0, WINDOW_HEIGHT, 2.0, WHITE);
        draw_line(WINDOW_WIDTH, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, 2.0, WHITE);
        
        draw_text("Control: arrows left/right", 10.0, 60.0, 20.0, WHITE);

        next_frame().await;
    }
}