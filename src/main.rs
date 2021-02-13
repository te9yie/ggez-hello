use ggez::{
    conf::{WindowMode, WindowSetup},
    event::{self, EventHandler},
    graphics::{self, Text, Image, DrawParam, spritebatch::SpriteBatch},
    Context, ContextBuilder, GameResult,
};
use glam::Vec2;
use graphics::Font;
use std::{env, path::PathBuf};

const SCREEN_SIZE: (f32, f32) = (16.0 * 40.0, 9.0 * 40.0);

struct State {
    text: Text,
    sprite: SpriteBatch,
}

impl State {
    fn new(ctx: &mut Context) -> GameResult<State> {
        let font = Font::new(ctx, "/mplus-1p-medium.ttf")?;
        let text = Text::new(("Hello", font, 48.0));
        let image = Image::new(ctx, "/kid_seikaku_kachiki_boy.png")?;
        let sprite = SpriteBatch::new(image);
        Ok(State { text, sprite })
    }
}

impl EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.17, 0.17, 0.17, 1.0].into());

        graphics::draw(ctx, &self.text, (Vec2::new(20.0, 20.0),))?;

        let p = DrawParam::new()
            .dest(Vec2::new(80.0, 20.0));
        self.sprite.add(p);
        graphics::draw(ctx, &self.sprite, p)?;
        self.sprite.clear();

        graphics::present(ctx)
    }
}

fn main() -> GameResult {
    let resource_dir = if let Ok(dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = PathBuf::from(dir);
        path.push("resources");
        path
    } else {
        PathBuf::from("resources")
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("hello", "te9yie")
        .window_setup(WindowSetup::default().title("Hello"))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .add_resource_path(resource_dir)
        .build()?;
    let mut state = State::new(&mut ctx)?;
    event::run(&mut ctx, &mut event_loop, &mut state)
}
