extern crate rand;

use self::rand::distributions::{IndependentSample, Range};
use base::{Agent, State, Side};

pub struct AgentRandom;

impl Agent for AgentRandom {
    fn make_move(&self, state: &mut State, side: Side) {
        let mut rng = rand::thread_rng();
        let is: Vec<_> = (0..9)
            .into_iter()
            .filter(|&i| state.cells[i] == None)
            .collect();
        let i = is[Range::new(0, is.len()).ind_sample(&mut rng)];
        state.set(i, Some(side));
    }
}
