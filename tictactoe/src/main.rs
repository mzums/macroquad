use macroquad::prelude::*;

const BOARD_SIZE: usize = 3;
const CELL_SIZE: f32 = 150.0;
const BOARD_PADDING: f32 = 50.0;
const LINE_THICKNESS: f32 = 5.0;
const STATUS_HEIGHT: f32 = 60.0;

#[derive(Clone, Copy, PartialEq)]
enum Player {
    X,
    O,
}

#[derive(Clone, Copy, PartialEq)]
enum CellState {
    Empty,
    Occupied(Player),
}

enum GameState {
    Playing,
    Win(Player),
    Draw,
}

struct Game {
    board: [[CellState; BOARD_SIZE]; BOARD_SIZE],
    current_player: Player,
    game_state: GameState,
}

impl Game {
    fn new() -> Self {
        Self {
            board: [[CellState::Empty; BOARD_SIZE]; BOARD_SIZE],
            current_player: Player::X,
            game_state: GameState::Playing,
        }
    }

    fn make_move(&mut self, row: usize, col: usize) {
        if let GameState::Playing = self.game_state {
            if let CellState::Empty = self.board[row][col] {
                self.board[row][col] = CellState::Occupied(self.current_player);
                self.check_winner();
                self.current_player = match self.current_player {
                    Player::X => Player::O,
                    Player::O => Player::X,
                };
            }
        }
    }

    fn check_winner(&mut self) {
        for row in 0..BOARD_SIZE {
            if self.board[row][0] == self.board[row][1] && 
               self.board[row][1] == self.board[row][2] {
                if let CellState::Occupied(p) = self.board[row][0] {
                    self.game_state = GameState::Win(p);
                    return;
                }
            }
        }

        for col in 0..BOARD_SIZE {
            if self.board[0][col] == self.board[1][col] && 
               self.board[1][col] == self.board[2][col] {
                if let CellState::Occupied(p) = self.board[0][col] {
                    self.game_state = GameState::Win(p);
                    return;
                }
            }
        }

        if self.board[0][0] == self.board[1][1] && 
           self.board[1][1] == self.board[2][2] {
            if let CellState::Occupied(p) = self.board[1][1] {
                self.game_state = GameState::Win(p);
                return;
            }
        }

        if self.board[0][2] == self.board[1][1] && 
           self.board[1][1] == self.board[2][0] {
            if let CellState::Occupied(p) = self.board[1][1] {
                self.game_state = GameState::Win(p);
                return;
            }
        }

        if self.board.iter().flatten().all(|&cell| {
            !matches!(cell, CellState::Empty)
        }) {
            self.game_state = GameState::Draw;
        }
    }

    fn reset(&mut self) {
        *self = Game::new();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tic Tac Toe".to_owned(),
        window_width: (CELL_SIZE * BOARD_SIZE as f32 + BOARD_PADDING * 2.0) as i32,
        window_height: (CELL_SIZE * BOARD_SIZE as f32 + BOARD_PADDING * 2.0 + STATUS_HEIGHT) as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();

    loop {
        clear_background(LIGHTGRAY);

        let board_start_x = BOARD_PADDING;
        let board_start_y = BOARD_PADDING;
        let board_width = CELL_SIZE * BOARD_SIZE as f32;

        for i in 0..=BOARD_SIZE {
            let pos = board_start_x + i as f32 * CELL_SIZE;
            draw_line(pos, board_start_y, pos, board_start_y + board_width, LINE_THICKNESS, BLACK);
            draw_line(board_start_x, board_start_y + i as f32 * CELL_SIZE, 
                     board_start_x + board_width, board_start_y + i as f32 * CELL_SIZE, 
                     LINE_THICKNESS, BLACK);
        }

        for row in 0..BOARD_SIZE {
            for col in 0..BOARD_SIZE {
                let x = board_start_x + col as f32 * CELL_SIZE;
                let y = board_start_y + row as f32 * CELL_SIZE;
                
                match game.board[row][col] {
                    CellState::Occupied(Player::X) => {
                        draw_line(x + 20.0, y + 20.0, x + CELL_SIZE - 20.0, y + CELL_SIZE - 20.0, 10.0, BLUE);
                        draw_line(x + CELL_SIZE - 20.0, y + 20.0, x + 20.0, y + CELL_SIZE - 20.0, 10.0, BLUE);
                    }
                    CellState::Occupied(Player::O) => {
                        draw_circle_lines(x + CELL_SIZE / 2.0, y + CELL_SIZE / 2.0, CELL_SIZE / 3.0, 10.0, RED);
                    }
                    CellState::Empty => {}
                }
            }
        }

        let status = match game.game_state {
            GameState::Playing => format!(
                "Player: {}",
                match game.current_player {
                    Player::X => "X (Blue)",
                    Player::O => "O (Red)",
                }
            ),
            GameState::Win(player) => format!(
                "Player {} wins!",
                match player {
                    Player::X => "X (Blue)",
                    Player::O => "O (Red)",
                }
            ),
            GameState::Draw => "Draw!".to_string(),
        };

        let status_color = match game.game_state {
            GameState::Win(Player::X) => BLUE,
            GameState::Win(Player::O) => RED,
            _ => DARKGRAY,
        };

        draw_text(
            &status,
            board_start_x,
            board_start_y + board_width + 40.0,
            40.0,
            status_color,
        );

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();
            if mouse_y >= board_start_y && mouse_y <= board_start_y + board_width &&
               mouse_x >= board_start_x && mouse_x <= board_start_x + board_width {
                let col = ((mouse_x - board_start_x) / CELL_SIZE) as usize;
                let row = ((mouse_y - board_start_y) / CELL_SIZE) as usize;
                if row < BOARD_SIZE && col < BOARD_SIZE {
                    game.make_move(row, col);
                }
            }
        }

        if is_key_pressed(KeyCode::Space) {
            game.reset();
        }

        next_frame().await;
    }
}
