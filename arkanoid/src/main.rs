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
const BRICK_ROWS: usize = 5;
const BRICK_COLS: usize = 8;
const BRICK_GAP: f32 = 5.0;
const PADDLE_Y_OFFSET: f32 = 40.0;
const BOTTOM_BOUNDARY: f32 = WINDOW_HEIGHT - 20.0;

struct Paddle {
    rect: Rect,
}

impl Paddle {
    fn new() -> Self {
        Self {
            rect: Rect::new(
                WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0,
                WINDOW_HEIGHT - PADDLE_Y_OFFSET,
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
    fn new(paddle: &Paddle) -> Self {
        Self {
            rect: Rect::new(
                paddle.rect.x + (paddle.rect.w - BALL_SIZE) / 2.0,
                paddle.rect.y - BALL_SIZE,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: vec2(0.0, 0.0),
        }
    }

    fn update(&mut self, paddle: &Paddle, waiting_to_start: bool) {
        if waiting_to_start {
            self.rect.x = paddle.rect.x + (paddle.rect.w - self.rect.w) / 2.0;
            return;
        }

        self.rect.x += self.vel.x;
        self.rect.y += self.vel.y;

        if self.rect.x < 0.0 {
            self.rect.x = 0.0;
            self.vel.x *= -1.0;
        }
        if self.rect.x > WINDOW_WIDTH - self.rect.w {
            self.rect.x = WINDOW_WIDTH - self.rect.w;
            self.vel.x *= -1.0;
        }
        if self.rect.y < 0.0 {
            self.rect.y = 0.0;
            self.vel.y *= -1.0;
        }
    }

    fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, RED);
    }

    fn launch(&mut self) {
        self.vel = vec2(rand::gen_range(-1.0, 1.0), -1.0).normalize() * BALL_SPEED;
    }
}

struct Brick {
    rect: Rect,
    color: Color,
    health: i32,
}

impl Brick {
    fn new(x: f32, y: f32) -> Self {
        Self {
            rect: Rect::new(x, y, BRICK_WIDTH, BRICK_HEIGHT),
            color: Color::from_rgba(
                rand::gen_range(100, 255),
                rand::gen_range(100, 255),
                rand::gen_range(100, 255),
                255,
            ),
            health: 1,
        }
    }

    fn draw(&self) {
        if self.health > 0 {
            draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, self.color);
            draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 2.0, DARKGRAY);
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

fn create_bricks() -> Vec<Brick> {
    let total_width = BRICK_COLS as f32 * BRICK_WIDTH + (BRICK_COLS - 1) as f32 * BRICK_GAP;
    let start_x = (WINDOW_WIDTH - total_width) / 2.0;
    let start_y = 50.0;
    
    let mut bricks = Vec::new();
    for row in 0..BRICK_ROWS {
        for col in 0..BRICK_COLS {
            bricks.push(Brick::new(
                start_x + col as f32 * (BRICK_WIDTH + BRICK_GAP),
                start_y + row as f32 * (BRICK_HEIGHT + BRICK_GAP),
            ));
        }
    }
    bricks
}

#[macroquad::main("Arkanoid")]
async fn main() {
    rand::srand(miniquad::date::now() as u64);
    
    let mut paddle = Paddle::new();
    let mut ball = Ball::new(&paddle);
    let mut bricks = create_bricks();
    let mut game_over = false;
    let mut score = 0;
    let mut lives = 3;
    let mut waiting_to_start = true;
    let mut game_won = false;

    loop {
        paddle.update();
        ball.update(&paddle, waiting_to_start);

        if !game_over && !waiting_to_start {
            if resolve_collision(&mut ball.rect, &mut ball.vel, &paddle.rect) {
                let hit_position = (ball.rect.center().x - paddle.rect.center().x) / (paddle.rect.w / 2.0);
                ball.vel.x = hit_position * BALL_SPEED;
            }

            for brick in bricks.iter_mut() {
                if brick.health > 0 && resolve_collision(&mut ball.rect, &mut ball.vel, &brick.rect) {
                    brick.health -= 1;
                    score += 10; 
                    break;
                }
            }

            if ball.rect.y > BOTTOM_BOUNDARY {
                lives -= 1;
                if lives <= 0 {
                    game_over = true;
                    game_won = false;
                } else {
                    waiting_to_start = true;
                    ball = Ball::new(&paddle);
                }
            }

            if bricks.iter().all(|b| b.health <= 0) {
                game_over = true;
                game_won = true;
            }
        }

        if is_key_pressed(KeyCode::Space) {
            if game_over {
                // Reset game
                paddle = Paddle::new();
                ball = Ball::new(&paddle);
                bricks = create_bricks();
                game_over = false;
                score = 0;
                lives = 3;
                waiting_to_start = true;
            } else if waiting_to_start {
                waiting_to_start = false;
                ball.launch();
            }
        }

        clear_background(BLACK);

        draw_line(0.0, BOTTOM_BOUNDARY, WINDOW_WIDTH, BOTTOM_BOUNDARY, 2.0, GRAY);

        paddle.draw();
        ball.draw();
        for brick in &bricks {
            brick.draw();
        }

        draw_text(&format!("SCORE: {}", score), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("LIVES: {}", lives), WINDOW_WIDTH - 120.0, 30.0, 30.0, WHITE);

        if waiting_to_start && !game_over {
            let text = "PRESS SPACE TO LAUNCH";
            let text_size = measure_text(text, None, 40, 1.0);
            draw_text(
                text,
                WINDOW_WIDTH / 2.0 - text_size.width / 2.0,
                WINDOW_HEIGHT / 2.0 + 50.0,
                40.0,
                YELLOW,
            );
            
            let instructions = "CONTROLS: LEFT/RIGHT ARROWS";
            let inst_size = measure_text(instructions, None, 30, 1.0);
            draw_text(
                instructions,
                WINDOW_WIDTH / 2.0 - inst_size.width / 2.0,
                WINDOW_HEIGHT / 2.0 + 100.0,
                30.0,
                LIGHTGRAY,
            );
        }

        if game_over {
            let text = if game_won {
                "YOU WIN!"
            } else {
                "GAME OVER!"
            };
            
            let text_size = measure_text(text, None, 60, 1.0);
            draw_text(
                text,
                WINDOW_WIDTH / 2.0 - text_size.width / 2.0,
                WINDOW_HEIGHT / 2.0 - 30.0,
                60.0,
                if game_won { GREEN } else { RED },
            );
            
            let restart_text = "PRESS SPACE TO PLAY AGAIN";
            let restart_size = measure_text(restart_text, None, 40, 1.0);
            draw_text(
                restart_text,
                WINDOW_WIDTH / 2.0 - restart_size.width / 2.0,
                WINDOW_HEIGHT / 2.0 + 40.0,
                40.0,
                YELLOW,
            );
        }

        next_frame().await;
    }
}