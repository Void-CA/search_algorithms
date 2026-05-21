use core::fmt;

use crate::labrynth::agent::AgentState;

#[derive(Debug, Clone)]
pub enum LabrynthCell {
    Door { id: usize },
    Key { id: usize },
    Obstacle,
    Empty,
    Start,
    Goal
}

impl LabrynthCell {
    pub fn is_walkable(&self, state: &AgentState) -> bool {
        match self {
            LabrynthCell::Obstacle => false,

            LabrynthCell::Door { id } => {
                state.has_key(*id)
            }

            _ => true,
        }
    }

    pub fn interact(&self, state: &mut AgentState) {
        match self {
            LabrynthCell::Key { id } => {
                state.collect_key(*id);
            }

            _ => {}
        }
    }
}

pub type Position = (usize, usize);

#[derive(Debug)]
pub struct Labrynth {
    pub grid: Vec<Vec<LabrynthCell>>,
    start: Position,
    goal: Position,
}   

impl fmt::Display for Labrynth {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.grid {
            for cell in row {
                match cell {
                    LabrynthCell::Door { id } => write!(f, " D |")?,
                    LabrynthCell::Key { id } => write!(f, " K |")?,
                    LabrynthCell::Obstacle => write!(f, " O |")?,
                    LabrynthCell::Empty => write!(f, "   |")?,
                    LabrynthCell::Start => write!(f, " S |")?,
                    LabrynthCell::Goal => write!(f, " G |")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Labrynth {
    pub fn new(grid_size: usize,start: (usize, usize), goal: (usize, usize), doors: Vec<(usize, usize)>, keys: Vec<(usize, usize)>, obstacles: Vec<(usize, usize)>) -> Self {
        let mut grid = vec![vec![LabrynthCell::Empty; grid_size]; grid_size];
        for (i, door) in doors.iter().enumerate() {
            grid[door.0][door.1] = LabrynthCell::Door { id: i }
        }

        for (i, key) in keys.iter().enumerate() {
            grid[key.0][key.1] = LabrynthCell::Key { id: i }
        }

        grid[start.0][start.1] = LabrynthCell::Start;
        grid[goal.0][goal.1] = LabrynthCell::Goal;
        
        for obstacle in obstacles {
            grid[obstacle.0][obstacle.1] = LabrynthCell::Obstacle;
        }
        Labrynth { grid, start, goal }
    }

    pub fn get_start_position(&self) -> (usize, usize) {
        self.start
    }

    pub fn get_goal(&self) -> (usize, usize) {
        self.goal
    }

    pub fn neighbors(&self, position: (usize, usize)) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
        for dir in directions {
            let new_pos = ((position.0 as isize + dir.0) as usize, (position.1 as isize + dir.1) as usize);
            if new_pos.0 < self.grid.len() && new_pos.1 < self.grid[0].len() {
                if let LabrynthCell::Obstacle = self.grid[new_pos.0][new_pos.1] {
                    continue;
                }
                neighbors.push(new_pos);
            }
        }
        neighbors

    }
}

