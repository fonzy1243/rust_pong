use ggez::input::keyboard::KeyCode;
use ggez::*;
use rand::prelude::*;
use std::f32::consts::PI;

// define game window resolution (width x length)
const SCREEN_SIZE: (f32, f32) = (800., 600.);

// define ball size (width x height)
const BALL_SIZE: (f32, f32) = (8., 8.);

// define paddle size (width x height)
const PADDLE_SIZE: (f32, f32) = (8., 45.);

// define paddle offset from screen side
const PADDLE_OFFSET: f32 = 55.;

// define paddle speed
const PADDLE_SPEED: f32 = 2.5;

// target fps for physics
const TARGET_FPS: u32 = 165;

// define default horizontal ball velocity
const BALL_X_VEL: f32 = 1.5;

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
                0. + PADDLE_OFFSET,
                SCREEN_SIZE.1 / 2. - PADDLE_SIZE.1 / 2.,
                PADDLE_SIZE.0,
                PADDLE_SIZE.1,
            ),
            PlayerSide::Right => ggez::graphics::Rect::new(
                SCREEN_SIZE.0 - PADDLE_OFFSET - PADDLE_SIZE.0,
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
                    self.paddle.y -= PADDLE_SPEED
                } else {
                    self.paddle.y = 0.
                }
            }
            Direction::Down => {
                if self.paddle.y < SCREEN_SIZE.1 - PADDLE_SIZE.1 {
                    self.paddle.y += PADDLE_SPEED
                } else {
                    self.paddle.y = SCREEN_SIZE.1 - PADDLE_SIZE.1
                }
            }
            Direction::None => (),
        }
    }
}

struct Ball {
    ball: ggez::graphics::Rect,
    velocity: ggez::glam::Vec2,
}

impl Ball {
    fn new() -> Self {
        let mut rng = rand::thread_rng();
        let rand_y: f32 = rng.gen_range(-1.0..=1.0);

        let velocity = ggez::glam::Vec2::new(BALL_X_VEL, rand_y);

        Self {
            ball: ggez::graphics::Rect::new(
                SCREEN_SIZE.0 / 2. + BALL_SIZE.0 / 2.,
                SCREEN_SIZE.1 / 2. - BALL_SIZE.1 / 2.,
                BALL_SIZE.0,
                BALL_SIZE.1,
            ),
            velocity,
        }
    }

    fn draw(&self, canvas: &mut ggez::graphics::Canvas) {
        canvas.draw(
            &ggez::graphics::Quad,
            ggez::graphics::DrawParam::new().dest_rect(self.ball),
        );
    }

    fn update(&mut self, players: (&mut Player, &mut Player)) {
        // check if ball collided with the top or bottom of the screen
        if self.ball.y <= 0. || self.ball.y >= SCREEN_SIZE.1 - BALL_SIZE.1 {
            self.velocity.y = -self.velocity.y;
        }

        if self.ball.x <= 0. {
            // update player 1 score
            players.0.score += 1;

            self.ball.x = SCREEN_SIZE.0 / 2. + BALL_SIZE.0 / 2.;
            self.ball.y = SCREEN_SIZE.1 / 2. - BALL_SIZE.1 / 2.;

            let mut rng = rand::thread_rng();
            let rand_y = rng.gen_range(-1.0..=1.0);

            self.velocity = ggez::glam::Vec2::new(-BALL_X_VEL, rand_y);
        } else if self.ball.x >= SCREEN_SIZE.0 - BALL_SIZE.0 {
            players.1.score += 1;

            self.ball.x = SCREEN_SIZE.0 / 2. + BALL_SIZE.0 / 2.;
            self.ball.y = SCREEN_SIZE.1 / 2. - BALL_SIZE.1 / 2.;

            let mut rng = rand::thread_rng();
            let rand_y = rng.gen_range(-1.0..=1.0);

            self.velocity = ggez::glam::Vec2::new(BALL_X_VEL, rand_y);
        }

        self.ball.x += self.velocity.x;
        self.ball.y += self.velocity.y;
    }

    fn check_collision(&mut self, paddle: ggez::graphics::Rect) {
        if self.ball.overlaps(&paddle) {
            let offset = (self.ball.y + BALL_SIZE.1 - paddle.y) / (PADDLE_SIZE.1 + BALL_SIZE.1);
            let phi = 0.25 * PI * (2. * offset - 1.);

            let speed = self.velocity.length();

            self.velocity.x *= -1.;
            self.velocity.y = speed * phi.sin();
        }
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
        while ctx.time.check_update_time(TARGET_FPS) {
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
            self.ball.update((&mut self.player1, &mut self.player2));

            // check for ball-paddle collisions
            self.ball.check_collision(self.player1.paddle);
            self.ball.check_collision(self.player2.paddle);
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
