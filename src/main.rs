use ggez::{
    conf::Conf,
    event::{self, EventHandler},
    Context, ContextBuilder, GameResult,
};

struct State {}

impl State {
    fn new() -> State {
        State {}
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
}

fn main() {
    let conf = Conf::new();
    let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello", "te9yie")
        .conf(conf)
        .build()
        .unwrap();
    let mut state = State::new();
    event::run(ctx, event_loop, &mut state).unwrap();
}
