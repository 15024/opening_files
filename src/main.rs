extern crate ggez;


use ggez::{Context, GameResult, event, graphics::{self, Image}};
use std::fs::read_dir;

const MAX_LAYERS: usize = 10;


fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("opening_files", "someone")
    .add_resource_path("resources")
    .window_setup(ggez::conf::WindowSetup::default().title("Super Cool Title"))
    .window_mode(ggez::conf::WindowMode::default().borderless(true));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new(ctx)?;
    event::run(ctx, event_loop, state)
}

struct MainState {
    image_holder: ImageHolder,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let state = MainState { 
            image_holder: ImageHolder::new("resources/images", ctx)?,
            };
        Ok(state)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for image in &self.image_holder.images {
            graphics::draw(ctx, image, (ggez::nalgebra::Point2::new(0.0, 0.0),))?;
        }

        //actually puts everything on screen
        graphics::present(ctx)?;

        //returns an empty tuple, so that we can test if it worked.
        Ok(())
    }
}

struct ImageHolder {
    images: Vec<Image>
}

impl ImageHolder {
    fn new(images_path: &str, ctx: &mut Context) -> GameResult<ImageHolder> {
        let mut images = Vec::with_capacity(MAX_LAYERS);
        for number in 0..read_dir(&images_path)?.count() - 1 {
            let image_path = format!("{0}/{1}.png", images_path, number);
            images.push(graphics::Image::new(ctx, image_path)?);
        }
        let img_holder = ImageHolder { images: images };
        Ok(img_holder)
    }
}