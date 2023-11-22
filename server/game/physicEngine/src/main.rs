mod bezier;
mod game;
mod vehicle;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // use bezier::*;
    // À partir d'un chemin prédéfini :
    // let bez1 = Bezier::new(Point(0., 0.), Point(0., 0.5), Point(0., 0.5), Point(1., 1.));
    // let bez2 = Bezier::new(Point(1., 1.), Point(0.5, 0.), Point(0.5, 0.), Point(0., 0.));
    // let mut game = game::Game::new(vec![bez1, bez2], 1, &dimensions)?; 

    // À partir d'une carte aléatoire
    let dimensions = vec![(0.3, 0.7), (0.5, 0.8), (0.4, 0.9)];
    let mut game = game::Game::new(Vec::new(), 1, &dimensions)?;
    game.animate("animate.gif", 25)?;
    // game.plot_game("test.png")?;
    // game.plot_map("map.png", &dimensions)?;
    Ok(())
}
