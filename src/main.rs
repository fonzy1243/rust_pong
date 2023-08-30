use ggez::*;

// define game window resolution (width x length)
const SCREEN_SIZE: (f32, f32) = (800., 600.);

// define ball size (width x height)
const BALL_SIZE: (f32, f32) = (20., 20.);

// define paddle size (width x height)
const PADDLE_SIZE: (f32, f32) = (5., 50.);

fn main() -> GameResult {
    let (ctx, event_loop) = ContextBuilder::new("Pong", "fonzy")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;

    let state = State {};

    event::run(ctx, event_loop, state);
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

enum PlayerSide {
    Left,
    Right,
}

struct Player {
    paddle: ggez::graphics::Rect,
    score: u32,
}

impl Player {
    fn new(side: PlayerSide) -> Self {
        let paddle = match side {
            PlayerSide::Left => todo!(),
            PlayerSide::Right => todo!(),
        };

        let score = 0u32;
    }
}

struct Ball {
    ball: ggez::graphics::Rect,
}

impl Ball {
    fn new() -> Self {
        Self {
            ball: ggez::graphics::Rect::new(
                SCREEN_SIZE.0 / 2.,
                SCREEN_SIZE.1 / 2.,
                BALL_SIZE.0,
                BALL_SIZE.1,
            ),
        }
    }
}

struct State {}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}
