use ggez::input::keyboard::KeyCode;
use ggez::*;

// define game window resolution (width x length)
const SCREEN_SIZE: (f32, f32) = (800., 600.);

// define ball size (width x height)
const BALL_SIZE: (f32, f32) = (8., 8.);

// define paddle size (width x height)
const PADDLE_SIZE: (f32, f32) = (8., 45.);

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Pong", "fonzy")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong").vsync(true))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = State::new();

    event::run(ctx, event_loop, state);
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

enum PlayerSide {
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Player {
    paddle: ggez::graphics::Rect,
    direction: Direction,
    pub score: u32,
}

impl Player {
    fn new(side: PlayerSide) -> Self {
        let paddle = match side {
            PlayerSide::Left => ggez::graphics::Rect::new(
                0. + 75.,
                SCREEN_SIZE.1 / 2. - PADDLE_SIZE.1 / 2.,
                PADDLE_SIZE.0,
                PADDLE_SIZE.1,
            ),
            PlayerSide::Right => ggez::graphics::Rect::new(
                SCREEN_SIZE.0 - 75. - PADDLE_SIZE.0,
                SCREEN_SIZE.1 / 2. - PADDLE_SIZE.1 / 2.,
                PADDLE_SIZE.0,
                PADDLE_SIZE.1,
            ),
        };

        let score = 0u32;

        Self {
            paddle,
            direction: Direction::None,
            score,
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        canvas.draw(
            &ggez::graphics::Quad,
            ggez::graphics::DrawParam::new().dest_rect(self.paddle),
        );
    }

    fn update(&mut self) {
        match self.direction {
            Direction::Up => {
                if self.paddle.y > 0. {
                    self.paddle.y -= 2.5
                } else {
                    self.paddle.y = 0.
                }
            }
            Direction::Down => {
                if self.paddle.y < SCREEN_SIZE.1 - PADDLE_SIZE.1 {
                    self.paddle.y += 2.5
                } else {
                    self.paddle.y = SCREEN_SIZE.1 - PADDLE_SIZE.1
                }
            }
            Direction::Left => (),
            Direction::Right => (),
            Direction::None => (),
        }
    }
}

struct Ball {
    ball: ggez::graphics::Rect,
}

impl Ball {
    fn new() -> Self {
        Self {
            ball: ggez::graphics::Rect::new(
                SCREEN_SIZE.0 / 2. + BALL_SIZE.0 / 2.,
                SCREEN_SIZE.1 / 2. - BALL_SIZE.1 / 2.,
                BALL_SIZE.0,
                BALL_SIZE.1,
            ),
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        canvas.draw(
            &ggez::graphics::Quad,
            ggez::graphics::DrawParam::new().dest_rect(self.ball),
        );
    }
}

struct State {
    player1: Player,
    player2: Player,
    ball: Ball,
    is_finished: bool,
}

impl State {
    fn new() -> Self {
        Self {
            player1: Player::new(PlayerSide::Left),
            player2: Player::new(PlayerSide::Right),
            ball: Ball::new(),
            is_finished: false,
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(165) {
            if self.is_finished {
                ctx.request_quit();
            }

            let k_ctx = &ctx.keyboard;

            self.player1.direction = Direction::None;
            self.player2.direction = Direction::None;

            if k_ctx.is_key_pressed(KeyCode::A) {
                self.player1.direction = Direction::Up;
            }

            if k_ctx.is_key_pressed(KeyCode::D) {
                self.player1.direction = Direction::Down;
            }

            if k_ctx.is_key_pressed(KeyCode::Left) {
                self.player2.direction = Direction::Up;
            }

            if k_ctx.is_key_pressed(KeyCode::Right) {
                self.player2.direction = Direction::Down;
            }

            self.player1.update();
            self.player2.update();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = ggez::graphics::Canvas::from_frame(ctx, ggez::graphics::Color::BLACK);

        self.player1.draw(&mut canvas);
        self.player2.draw(&mut canvas);
        self.ball.draw(&mut canvas);

        canvas.finish(ctx)?;
        Ok(())
    }
}
