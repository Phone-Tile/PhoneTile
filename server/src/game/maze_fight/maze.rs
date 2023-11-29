use std::vec;

use plotters::element;



struct Cell {
    compute: bool,

}


struct Maze {

}

impl Maze {
    pub fn new(matrix: Vec<Vec<bool>>) {



    }
}


pub fn maze_from_graph(elements: usize) -> Vec<bool> {
    let mut ckecked: Vec<bool> = vec::Vec::with_capacity(elements);
    let mut res: Vec<bool> = vec::Vec::with_capacity(elements*elements);
    for i in 0..elements*elements {
        res.push(false);
    }

    let mut stack: Vec<usize> = vec::Vec::with_capacity(elements);
    stack.push(0);
    while !stack.is_empty() {
        let vert = stack.pop();
        let neighbours_to_visit = vec::Vec::new();
        for i in )0..elements*elements {
            if 
        }
    }
}





// struct Cell {
//     walls: [bool; 4],
// }

// struct Grid {
//     height: u16,
//     width: u16,
//     cells: Box<[Cell]>,
// }

// struct Maze {
//     grid: Vec<Grid>,
// }

// impl Maze {
//     pub fn new(grid: Vec<Grid>) -> Self {
        

//         Maze { grid }
//     }
// }