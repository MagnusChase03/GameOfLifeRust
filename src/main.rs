use rand::Rng;
use std::{thread, time};
use std::fmt;

#[derive(Clone, PartialEq)]
enum CellState {

    Dead,
    Alive

}

impl fmt::Debug for CellState {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        match *self {

            CellState::Dead => write!(f, "-"),
            CellState::Alive => write!(f, "A"),

        }

    }

}

// Generate a len x len matrix of cells
fn create_grid(len: usize) -> Vec<Vec<CellState>> {

    let mut array: Vec<Vec<CellState>> = vec![vec![CellState::Dead; len]; len];
    
    // At every spot in the array 33% change to have a cell alive
    let mut rng = rand::thread_rng();
    for y in 0..len {

        for x in 0..len {

            let num: u8 = rng.gen_range(0..3);

            if num == 0 {

                array[y][x] = CellState::Alive;

                
            }
        }

    }
    
    array

}

// Print the grid with newlines after every row
fn print_grid(grid: &Vec<Vec<CellState>>) {

    for y in grid {

        println!("{:?}", y);

    }
    println!("");

}

// Returns weither the entire grid is dead
fn is_dead(grid: &Vec<Vec<CellState>>) -> bool {

    for y in grid {

        for x in y {

            if *x == CellState::Alive {

                return false;

            }

        }

    }

    true

}

// Returns the number of neighbors to a cell
fn number_neighbors(x: usize, y: usize, grid: &Vec<Vec<CellState>>) -> u8 {

    let mut num: u8 = 0;

    if x > 0 && grid[y][x - 1] == CellState::Alive {

        num += 1;

    }

    if x < grid[0].len() - 1 && grid[y][x + 1] == CellState::Alive {

        num += 1;

    }

    if y > 0 && grid[y - 1][x] == CellState::Alive {

        num += 1;

    }

    if y < grid.len() - 1 && grid[y + 1][x] == CellState::Alive {

        num += 1;

    }

    // Diags
    if x > 0 && y > 0 && grid[y - 1][x - 1] == CellState::Alive {

        num += 1;

    }

    if x > 0 && y < grid.len() - 1 && grid[y + 1][x - 1] == CellState::Alive {

        num += 1;

    }

    if x < grid[0].len() - 1 && y > 0 && grid[y - 1][x + 1] == CellState::Alive {

        num += 1;

    }

    if x < grid[0].len() - 1 && y < grid.len() - 1 && grid[y + 1][x + 1] == CellState::Alive {

        num += 1;

    }

    num

}

// Conways algorithm
fn step(grid: &mut Vec<Vec<CellState>>) {

    let mut updates: Vec<(usize, usize, CellState)> = Vec::new();

    let mut y: usize = 0;
    while y < grid.len() {

        let mut x: usize = 0;
        while x < grid[y].len() {

            let num_neighbors: u8 = number_neighbors(x, y, &grid);

            // A cell with less than 2 or more than 3 dies
            if (num_neighbors < 2 || num_neighbors > 3) && grid[y][x] == CellState::Alive {

                updates.push((x, y, CellState::Dead));

            // Cell with 3 becomes alive
            } else if num_neighbors == 3 && grid[y][x] == CellState::Dead {

                updates.push((x, y, CellState::Alive));

            }
            x += 1;

        }

        y += 1;

    }
    
    // Update cells that fit previous descriptions
    for (x, y, state) in updates {

        grid[y][x] = state;

    }

}

fn main() {

    let mut grid = create_grid(15);

    while !is_dead(&grid) {

        print_grid(&grid);
        step(&mut grid);
        thread::sleep(time::Duration::from_secs(1));

    }

}
