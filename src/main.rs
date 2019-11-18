use ggez::*;

struct State {
  pos_x: f32
}

impl ggez::event::EventHandler for State {
  fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
    // don't move yet
    // self.pos_x = self.pos_x % 800.0 + 1.0;
    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

    let shooter = graphics::Mesh::new_polygon(
        ctx,
        graphics::DrawMode::fill(),
        &[
          nalgebra::Point2::new(200.0, 200.0),
          nalgebra::Point2::new(185.0, 215.0),
          nalgebra::Point2::new(215.0, 215.0),
        ],
        graphics::Color::new(0.0, 204.0, 0.0, 1.0)
    )?;
    graphics::draw(ctx, &shooter, (nalgebra::Point2::new(self.pos_x, 380.0),))?;

    graphics::present(ctx)?;
    Ok(())
  }
}

fn main() {
  let state = &mut State { pos_x: 300.0 };

  let c = conf::Conf::new();
  let (ref mut ctx, ref mut event_loop) = ContextBuilder::new("hello_ggez", "awesome_person")
      .conf(c)
      .build()
      .unwrap();

  event::run(ctx, event_loop, state).unwrap();
}
