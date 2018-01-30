use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Side {X, O}

#[derive(Clone, PartialEq, Eq)]
pub struct State {
    pub cells: [Option<Side>; 9],
}

impl State {
    pub fn new() -> State {
        State { cells: [None; 9] }
    }

    pub fn set(&mut self, i: usize, cell: Option<Side>) {
        self.cells[i] = cell;
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let c = self.cells;
        let d = |cell| match cell {
            Some(Side::X) => "X",
            Some(Side::O) => "O",
            None          => " ",
        };
        writeln!(f, " {} | {} | {} ", d(c[0]), d(c[1]), d(c[2]))?;
        writeln!(f, " {} | {} | {} ", d(c[3]), d(c[4]), d(c[5]))?;
        writeln!(f, " {} | {} | {} ", d(c[6]), d(c[7]), d(c[8]))?;
        Ok(())
    }
}

pub trait Agent {
    fn make_move(&self, &mut State, Side);
}

pub struct Game<'a, A: Agent + 'a> {
    pub agent_x:   &'a A,
    pub agent_o:   &'a A,
    pub state:     State,
    pub next_side: Side,
}

impl<'a, A: Agent + 'a> Game<'a, A> {
    pub fn create(agent_x: &'a A, agent_o: &'a A) -> Game<'a, A> {
        Game::<A> { state: State::new(), agent_x, agent_o, next_side: Side::X }
    }

    pub fn next(&mut self) {
        let (agent, next_side) = match self.next_side {
            Side::X => (self.agent_x, Side::O),
            Side::O => (self.agent_o, Side::X),
        };
        agent.make_move(&mut self.state, self.next_side);
        self.next_side = next_side;
    }
}
