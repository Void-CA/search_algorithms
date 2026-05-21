use crate::labrynth::Labrynth;

pub fn colored_doors_excercise() -> Labrynth {    
    Labrynth::new(
        5, 
        (0, 0), 
        (4, 4), 
        vec![(2, 2)], 
        vec![(0, 4)], 
        vec![(2, 0), (2, 1), (2, 3), (2, 4)]
    )
}


pub fn grid_labrynth_excercise() -> Labrynth {
    Labrynth::new(
        4, 
        (0, 0), 
        (3, 3), 
        vec![], 
        vec![], 
        vec![(1,1), (1,2), (2,1)]
    )
}