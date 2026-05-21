mod labrynth;
mod exercises;

use crate::exercises::{colored_doors_excercise, grid_labrynth_excercise};
use crate::labrynth::agent::LabrynthAgent;

fn main() {
    
    let second_ex = colored_doors_excercise();
    let agent = LabrynthAgent::new(second_ex);
    separator();
    println!("Laberinto con puertas y llaves");
    separator();

    println!("Laberinto:\n{}", agent.labrynth);
    println!("Estado inicial: {}", agent.state);
    println!("Camino: {:?}\n\n", agent.bfs().unwrap());


    let third_ex = grid_labrynth_excercise();
    let agent2 = LabrynthAgent::new(third_ex);

    separator();
    println!("Laberinto con obstáculos");
    separator();

    println!("Laberinto:\n{}", agent2.labrynth);
    println!("Estado inicial: {}", agent2.state);
    println!("Camino usando BFS: {:?}", agent2.bfs().unwrap());
    println!("Camino usando DFS: {:?}\n\n", agent2.dfs_recursive().unwrap());
}

fn separator() {
    println!("==========================");
}