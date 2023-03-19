use crate::data::*;
use crate::display::*;



pub fn print_random_data() {
    // Fill the grid
    let grid = create_filled_grid(&TILE_SET, 16);

    // Lets print some data on all the various tile types
    println!("There are {} tiles", TILE_SET.len());
    println!("There are {} corner tiles", TILE_SET.iter().filter(|t| t.bottom == -1 && t.left == -1).count());
    println!("There are {} edge tiles", TILE_SET.iter().filter(|t| t.bottom != -1 && t.left == -1).count());
    println!("There are {} normal tiles", TILE_SET.iter().filter(|t| t.bottom != -1 && t.left != -1).count());

    print_grid(&grid);

    // Now try to do a merge kinda thing. We want to make a bunch of 2x2 MegaTiles
    // Let's only worry about the middle 14x14 grid for now.

    // Create some constraints for the megatiles based on the edge pieces
    let required_outside_matches = count_tile_types( &TILE_SET, true, true, false, false);
    println!("Required outside matches: {:?}", required_outside_matches);

    let required_inside_matches = count_tile_types( &TILE_SET, false, false, true, false);
    println!("Required inside matches:       {:?}", required_inside_matches);

    // Matches that shouldn't touch any edge pieces
    let mut required_matches = [0; 22];
    for i in 5..22 {
        required_matches[i] =  required_inside_matches[i] - required_outside_matches[i];
    }
    println!("Required matches:              {:?}", required_matches);
}

pub fn generate_wanted_hint_sides() -> [i32; 22] {
    let hints = [
        OrientedTile::new(207, &TILE_SET, 3),
        OrientedTile::new(254, &TILE_SET, 2),
        OrientedTile::new(180, &TILE_SET, 0),
        OrientedTile::new(248, &TILE_SET, 2),
    ];

    let mut hint_requirements = [0; 22];
    for hint in hints.iter() {
        // Want to make sure the bottom and left pieces required by the hints are available.
        hint_requirements[hint.bottom() as usize] += 1;
        hint_requirements[hint.left() as usize] += 1;
    }
    hint_requirements
}

pub fn count_tile_types(tileset: &[Tile], corners: bool, edges: bool, center: bool, trim_edge_only: bool) -> [i32; 22] {
    let mut tile_counts = [0; 22];
    for tile in tileset.iter() {
        if corners && tile.bottom == -1 && tile.left == -1 {
            tile_counts[tile.top as usize] += 1;
            tile_counts[tile.right as usize ] += 1;
        } else if edges && tile.bottom != -1 && tile.left == -1 {
            tile_counts[tile.top as usize] += 1;
            tile_counts[tile.right as usize] += 1;
            tile_counts[tile.bottom as usize] += 1;
        } else if center && tile.bottom != -1 && tile.left != -1 {
            tile_counts[tile.top as usize] += 1;
            tile_counts[tile.right as usize] += 1;
            tile_counts[tile.bottom as usize] += 1;
            tile_counts[tile.left as usize] += 1;
        }
    }
    if trim_edge_only {
        tile_counts[0] = 0;
        tile_counts[1] = 0;
        tile_counts[2] = 0;
        tile_counts[3] = 0;
        tile_counts[4] = 0;
    }
    tile_counts
}

pub fn generate_spiral_search_order(grid_size: usize) -> Vec<(usize, usize)> {
    let mut search_order = Vec::new();
    let mut x = grid_size / 2;
    let mut y = grid_size / 2;

    // Going to start from the top left and go downwards. We will reverse the list before returning
    let mut direction = 2;
    let mut steps = 1;
    let mut steps_taken = 0;
    while search_order.len() < grid_size * grid_size {
        search_order.push((x, y));

        // Move in the current direction
        match direction {
            0 => y -= 1,
            1 => x += 1,
            2 => y += 1,
            3 => x -= 1,
            _ => panic!("Invalid direction"),
        }
        steps_taken += 1;

        // If we have taken the required number of steps, change direction
        if steps_taken == steps {
            steps_taken = 0;
            direction = (direction + 1) % 4;

            // If we have changed direction twice, we need to take an extra step
            if direction % 2 == 0 {
                steps += 1;
            }
        }
    }

    // pretty print the search order onto a 2d grid with 2 digit numbers displaying the index
    let mut grid = [[0; 7]; 7];
    for (i, (x, y)) in search_order.iter().enumerate() {
        grid[*x][*y] = i;
    }
    for row in grid.iter() {
        for cell in row.iter() {
            print!("{:02} ", cell);
        }
        println!();
    }
    search_order
}