use crate::labrynth::labrynth::{Labrynth, Position};
use core::fmt;
use std::collections::{HashMap, VecDeque, HashSet};


pub type SearchResult = Option<Vec<Position>>;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct AgentState {
    pub position: Position,
    pub keys_collected: Vec<usize>,
}

impl AgentState {
    pub fn has_key(&self, id: usize) -> bool {
        self.keys_collected.contains(&id)
    }

    pub fn collect_key(&mut self, id: usize) {
        if !self.has_key(id) {
            self.keys_collected.push(id);
            self.keys_collected.sort();
        }
    }
}


pub struct LabrynthAgent {
    pub labrynth: Labrynth,
    pub state: AgentState
}

impl fmt::Display for AgentState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Position: {:?}, Keys: {:?}", self.position, self.keys_collected)
    }
}
impl LabrynthAgent {
    pub fn new(labrynth: Labrynth) -> Self {
        let state = AgentState {
            position: labrynth.get_start_position(),
            keys_collected: Vec::new(),
        };
        LabrynthAgent { labrynth, state }
    }
    

    pub fn bfs(&self) -> SearchResult {
        let start = self.state.clone();
        let goal = self.labrynth.get_goal();

        let mut explored = HashSet::new();
        explored.insert(start.clone());

        let mut queue = VecDeque::new();
        queue.push_back(start.clone());

        let mut parents: HashMap<AgentState, AgentState> = HashMap::new();

        while let Some(current) = queue.pop_front() {

            // condición de éxito
            if current.position == goal {
                let mut path = vec![current.position];

                let mut node = current;

                while node != start {
                    node = parents[&node].clone();
                    path.push(node.position);
                }

                path.reverse();

                return Some(path);
            }

            let neighbors = self.labrynth.neighbors(current.position);

            for neighbor in neighbors {
                let cell = &self.labrynth.grid[neighbor.0][neighbor.1];

                if !cell.is_walkable(&current) {
                    continue;
                }

                let mut next_state = current.clone();
                next_state.position = neighbor;

                cell.interact(&mut next_state);

                if !explored.contains(&next_state) {
                    explored.insert(next_state.clone());

                    parents.insert(
                        next_state.clone(),
                        current.clone(),
                    );

                    queue.push_back(next_state);
                }
            }
        }

        None
    }


    pub fn dfs_recursive(&self) -> SearchResult {
        let start = self.state.clone();
        let goal = self.labrynth.get_goal();

        let mut explored = HashSet::new();

        self.dfs_visit(
            start.clone(),
            goal,
            &mut explored,
        )
    }

    fn dfs_visit(
        &self,
        current: AgentState,
        goal: Position,
        explored: &mut HashSet<AgentState>,
    ) -> SearchResult {

        explored.insert(current.clone());

        // condición de éxito
        if current.position == goal {
            return Some(vec![current.position]);
        }

        let neighbors = self.labrynth.neighbors(current.position);

        for neighbor in neighbors {

            let cell = &self.labrynth.grid[neighbor.0][neighbor.1];

            if !cell.is_walkable(&current) {
                continue;
            }

            let mut next_state = current.clone();
            next_state.position = neighbor;

            cell.interact(&mut next_state);

            // evitar revisitar estados
            if explored.contains(&next_state) {
                continue;
            }

            if let Some(mut path) =
                self.dfs_visit(next_state, goal, explored)
            {
                path.insert(0, current.position);
                return Some(path);
            }
        }

        None
    }
}