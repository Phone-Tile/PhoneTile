mod bezier;
mod game;
mod vehicle;
use bezier::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bez1 = Bezier::new(Point(0., 0.), Point(0., 0.5), Point(0., 0.5), Point(1., 1.));
    let bez2 = Bezier::new(Point(1., 1.), Point(0.5, 0.), Point(0.5, 0.), Point(0., 0.));
    //vec![bez1, bez2]
    let mut game = game::Game::new(Vec::new(), 1);
    game.animate("animate.gif", 2)?;
    game.plot_game("test.png")?;
    // not working yet...
    // game.plot_map("map.png")?;
    Ok(())
}
