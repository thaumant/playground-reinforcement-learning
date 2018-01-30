mod base;
mod agents;

use base::{Game};
use agents::{AgentRandom};

fn main()  {
    let agent = AgentRandom;
    let mut game = Game::create(&agent, &agent);
    game.next();
    game.next();
    game.next();
    println!("{}", game.state);
}
