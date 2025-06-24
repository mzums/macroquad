use macroquad::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const PADDLE_WIDTH: f32 = 150.0;
const PADDLE_HEIGHT: f32 = 20.0;
const PADDLE_SPEED: f32 = 10.0;
const BALL_SIZE: f32 = 15.0;
const BALL_SPEED: f32 = 5.0;
const BRICK_WIDTH: f32 = 80.0;
const BRICK_HEIGHT: f32 = 30.0;
const BRICK_ROWS: usize = 1;
const BRICK_COLS: usize = 8;
const BRICK_GAP: f32 = 5.0;

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

        // Wall collisions
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

struct Brick {
    rect: Rect,
    active: bool,
}

impl Brick {
    fn new(x: f32, y: f32) -> Self {
        Self {
            rect: Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT),
            active: true,
        }
    }

    fn draw(&self) {
        if self.active {
            draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GREEN);
        }
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    if let Some(intersection) = a.intersect(*b) {
        if intersection.w > intersection.h {
            a.y = if vel.y > 0.0 {
                b.y - a.h
            } else {
                b.y + b.h
            };
            vel.y *= -1.0;
        } else {
            a.x = if vel.x > 0.0 {
                b.x - a.w
            } else {
                b.x + b.w
            };
            vel.x *= -1.0;
        }
        return true;
    }
    false
}

#[macroquad::main("Arkanoid")]
async fn main() {
    let mut paddle = Paddle::new();
    let mut ball = Ball::new();
    let mut bricks = Vec::new();

    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            bricks.push(Brick::new(
                col as f32 * (BRICK_WIDTH + BRICK_GAP) + BRICK_GAP,
                row as f32 * (BRICK_HEIGHT + BRICK_GAP) + BRICK_GAP + 150.0,
            ));
        }
    }

    loop {
        paddle.update();
        ball.update();

        resolve_collision(&mut ball.rect, &mut ball.vel, &paddle.rect);

        for brick in bricks.iter_mut() {
            if brick.active && resolve_collision(&mut ball.rect, &mut ball.vel, &brick.rect) {
                brick.active = false;
            }
        }

        clear_background(BLACK);
        
        paddle.draw();
        ball.draw();
        for brick in &bricks {
            brick.draw();
        }
        
        draw_text("Arkanoid", 10.0, 30.0, 20.0, WHITE);
        draw_text("Destroy all bricks", 10.0, 60.0, 20.0, WHITE);
        draw_text(&format!("Bricks left: {}", bricks.iter().filter(|b| b.active).count()), 
                 10.0, 90.0, 20.0, WHITE);

        next_frame().await;
    }
}
