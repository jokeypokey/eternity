// A solver for the eternity 2 puzzle
// https://en.wikipedia.org/wiki/Eternity_II

// The puzzle is a 16x16 grid of 4 sided tiles
// THe goal is to fill the grid by aligning the tiles so that the pattersn match
// The patterns will be denoted with 1 through 22

use std::collections::HashMap;

// Another file called data.rs to load in
mod data;
use data::build_indices;
use data::TILE_SET;
use data::MINI_TILE_SET;
use data::Tile;
use data::OrientedTile;
use data::MegaTile;


// Empty tile used for non existent tiles
const EMPTY_TILE: Tile = Tile {top: -1, right: -1, bottom: -1, left: -1, id: usize::MAX};

fn create_filled_grid(tileset: &[Tile], grid_size: usize) -> Vec<Vec<Option<OrientedTile>>> {

    let mut grid= vec![vec![None; grid_size]; grid_size];

    // Corners
    grid[0][0] = Some(OrientedTile::new(0, tileset, 1));
    grid[0][15] = Some(OrientedTile::new(1, tileset, 2));
    grid[15][0] = Some(OrientedTile::new(2, tileset, 0));
    grid[15][15] = Some(OrientedTile::new(3, tileset, 3));

    // Edges
    for i in 1..15 {  // Fill the top edge. Use tiles 4 to 17
        grid [0][i] = Some(OrientedTile::new(4 + i - 1, tileset, 1));
    }
    for i in 1..15 {  // Fill the bottom edge. Use tiles 18 to 31
        grid [grid_size-1][i] = Some(OrientedTile::new(18 + i - 1, tileset, 3));
    }
    for i in 1..15 { // Fill the left edge. Use tiles 32 to 45
        grid [i][0] = Some(OrientedTile::new(32 + i - 1, tileset, 0));
    }
    for i in 1..15 { // Fill the right edge. Use tiles 46 to 59
        grid [i][grid_size-1] = Some(OrientedTile::new(46 + i - 1, tileset, 2));
    }

    // Center
    for i in 1..15 {
        for j in 1..15 {
            grid [i][j] = Some(OrientedTile::new(60 + (i - 1) * 14 + j - 1, tileset, 0));
        }
    }

    grid
}

fn colour_symbol(num: i8, matches: bool, vertical: bool, is_megatile: bool) -> String {
    if num == -1 {
        if is_megatile {
            return String::from(" ")
        } else if vertical {
            return String::from("│")
        } else {
            return String::from("───")
        }
    }
    let tile_char = (num + 65) as u8 as char;
    if matches {
        if vertical || is_megatile{
            format!("\x1b[32m{}\x1b[0m", tile_char)
        } else {
            format!("\x1b[32m {} \x1b[0m", tile_char)
        }
    } else {
        if vertical || is_megatile {
            format!("\x1b[31m{}\x1b[0m", tile_char)
        } else {
            format!("\x1b[31m {} \x1b[0m", tile_char)
        }
    }
}

fn print_grid(grid: &Vec<Vec<Option<OrientedTile>>>) {
    /*
    ┌ I ┐
    R   J
    └ R ┘
    Correct matches are coloured green
    Bad matches are coloured red
    Only the number itself is coloured, not the whole tile.
    */

    let grid_size = grid.len();
    let mut match_count = 0;
    let max_matches = grid_size * (grid_size-1) * 2;

    for(i, row) in grid.iter().enumerate() {
        // The top line empty string we build on
        let mut top_line = String::new();
        let mut middle_line = String::new();
        let mut bottom_line = String::new();
        for(j, tile) in row.iter().enumerate() {
            if let Some(oriented_tile) = tile {
                // Check whether a match exists on the top side
                let top_matches = i > 0 && grid[i - 1][j].is_some() && oriented_tile.top() == grid[i - 1][j].unwrap().bottom();
                let right_matches = j < grid_size - 1 && grid[i][j + 1].is_some() && oriented_tile.right() == grid[i][j + 1].unwrap().left();
                let bottom_matches = i < grid_size - 1 && grid[i + 1][j].is_some() && oriented_tile.bottom() == grid[i + 1][j].unwrap().top();
                let left_matches = j > 0 && grid[i][j - 1].is_some() && oriented_tile.left() == grid[i][j - 1].unwrap().right();

                match_count += (top_matches as i32) + (right_matches as i32) + (bottom_matches as i32) + (left_matches as i32);

                let top_coloured_symbol = colour_symbol(oriented_tile.top(), top_matches, false, false);
                let right_coloured_symbol = colour_symbol(oriented_tile.right(), right_matches, true, false);
                let bottom_coloured_symbol = colour_symbol(oriented_tile.bottom(), bottom_matches, false, false);
                let left_coloured_symbol = colour_symbol(oriented_tile.left(), left_matches, true, false);

                top_line.push_str(&format!("┌{}┐ ", top_coloured_symbol));
                middle_line.push_str(&format!("{}   {} ", left_coloured_symbol, right_coloured_symbol));
                bottom_line.push_str(&format!("└{}┘ ", bottom_coloured_symbol));
            } else {
                top_line.push_str("┌───┐ ");
                middle_line.push_str("│   │ ");
                bottom_line.push_str("└───┘ ");
            }
        }
        println!("{}", top_line);
        println!("{}", middle_line);
        println!("{}", bottom_line);
    }

    println!("");
    println!("Matches {}/{}", match_count/2, max_matches); // Matches are double counted
}

fn visualise_megatile (megatile: &MegaTile) {
    /*
    ┌ L F ┐
    U     U
    G     G
    └ M T ┘
    This function will just visualise one megatile for now
     */
    let mut top_line = String::new();
    let mut top_middle_line = String::new();
    let mut middle_line = String::new();
    let mut bottom_middle_line = String::new();
    let mut bottom_line = String::new();
    let mut info_line = String::new();

    let top_left_symbol = colour_symbol(megatile.tile1.top(), true, false, true);
    let top_right_symbol = colour_symbol(megatile.tile2.top(), true, false, true);
    let left_top_symbol = colour_symbol(megatile.tile1.left(), true, true, true);
    let right_top_symbol = colour_symbol(megatile.tile2.right(), true, true, true);
    let left_bottom_symbol = colour_symbol(megatile.tile3.left(), true, true, true);
    let right_bottom_symbol = colour_symbol(megatile.tile4.right(), true, true, true);
    let bottom_left_symbol = colour_symbol(megatile.tile3.bottom(), true, false, true);
    let bottom_right_symbol = colour_symbol(megatile.tile4.bottom(), true, false, true);

    // Check for internal matches
    let top_middle_matches = megatile.tile1.right() == megatile.tile2.left();
    let bottom_middle_matches = megatile.tile3.right() == megatile.tile4.left();
    let left_middle_matches = megatile.tile1.bottom() == megatile.tile3.top();
    let right_middle_matches = megatile.tile2.bottom() == megatile.tile4.top();

    // use info line to describe the differences between the tiles
    if !top_middle_matches {
        info_line.push_str(&format!("Top middle doesn't match: {} != {}", colour_symbol(megatile.tile1.right(), true, true, true), colour_symbol(megatile.tile2.left(), true, true, true)));
    }
    if !right_middle_matches {
        info_line.push_str(&format!("Right middle doesn't match: {} != {}", colour_symbol(megatile.tile2.bottom(), true, false, true), colour_symbol(megatile.tile4.top(), true, false, true)));
    }
    if !bottom_middle_matches {
        info_line.push_str(&format!("Bottom middle doesn't match: {} != {}", colour_symbol(megatile.tile3.right(), true, true, true), colour_symbol(megatile.tile4.left(), true, true, true)));
    }
    if !left_middle_matches {
        info_line.push_str(&format!("Left middle doesn't match: {} != {}", colour_symbol(megatile.tile1.bottom(), true, false, true), colour_symbol(megatile.tile3.top(), true, false, true)));
    }

    let top_middle_symbol = colour_symbol(megatile.tile1.right(), top_middle_matches, true, true);
    let bottom_middle_symbol = colour_symbol(megatile.tile3.right(), bottom_middle_matches, true, true);
    let left_middle_symbol = colour_symbol(megatile.tile1.bottom(), left_middle_matches, false, true);
    let right_middle_symbol = colour_symbol(megatile.tile2.bottom(), right_middle_matches, false, true);

    top_line.push_str(&format!("┌{}─{}┐ ", top_left_symbol, top_right_symbol));
    top_middle_line.push_str(&format!("{} {} {} ", left_top_symbol, top_middle_symbol, right_top_symbol));
    middle_line.push_str(&format!("|{} {}|", left_middle_symbol, right_middle_symbol));
    bottom_middle_line.push_str(&format!("{} {} {} ", left_bottom_symbol, bottom_middle_symbol, right_bottom_symbol));
    bottom_line.push_str(&format!("└{}─{}┘ ", bottom_left_symbol, bottom_right_symbol));

    println!("{}", top_line);
    println!("{}", top_middle_line);
    println!("{}", middle_line);
    println!("{}", bottom_middle_line);
    println!("{}", bottom_line);
    if info_line.len() > 0 {
        println!("{}", info_line);
    }
}

fn main() {
    // New main function that makes a 7x7 brute force search
    // brute_force(7, &MINI_TILE_SET, &OrientedTile { index: 24, orientation: 0 });
    megatile_time();
    // brute_force(7, &MINI_TILE_SET, OrientedTile::new(24, &MINI_TILE_SET, 0));
}



fn brute_force(grid_size: usize, tile_set: &[Tile], centre_tile: OrientedTile) {
    // Create a 7x7 grid
    let mut grid = vec![vec![None; grid_size]; grid_size];

    // Create some indices on the tileset. The tileset contains 49 tiles
    let tile_lookup = build_indices(tile_set, 0);

    // Set the centre tile. We have to use the centre tile in the centre. This is a requirement of the puzzle
    grid[grid_size / 2][grid_size / 2] = Some(centre_tile);

    // Create a mask of the tiles that are available
    let mut available_mask = Vec::new();
    for _ in 0..tile_set.len() { available_mask.push(true); }
    available_mask[centre_tile.id()] = false;

    let search_order = generate_spiral_search_order(grid_size);

    // Recursively search for a solution
    if brute_force_recursive(&mut grid, &mut available_mask, tile_set, &tile_lookup, tile_set, &search_order, 1) {
        // If we get here, we have found a solution
        print_grid(&grid);
    } else {
        // If we get here, we have failed to find a solution
        println!("Failed to find a solution");
    }

}

fn get_adjacent_sides(grid: &Vec<Vec<Option<OrientedTile>>>, x: usize, y: usize) -> (i8, i8, i8, i8) {
    // Return -1 if nothing
    // Top, right, bottom, left
    let mut adjacent_sides = (-1, -1, -1, -1);
    if y > 0 && grid[y - 1][x].is_some() {
        adjacent_sides.0 = grid[y - 1][x].unwrap().bottom();
    }
    if x < grid[0].len() - 1 && grid[y][x + 1].is_some() {
        adjacent_sides.1 = grid[y][x + 1].unwrap().left();
    }
    if y < grid.len() - 1 && grid[y + 1][x].is_some() {
        adjacent_sides.2 = grid[y + 1][x].unwrap().top();
    }
    if x > 0 && grid[y][x - 1].is_some() {
        adjacent_sides.3 = grid[y][x - 1].unwrap().right();
    }

    adjacent_sides
}

fn find_oriented_tiles_that_fit_neighbours(uni_gram_to_tile_index: &HashMap<i8, Vec<usize>>,
                                           bi_gram_to_tile_index: &HashMap<(i8, i8), Vec<usize>>,
                                           tile_set: &[Tile],
                                           adjacent_sides: &[Option<i8>; 4],
                                           available_mask: &[bool]) -> Vec<OrientedTile> {
    let mut oriented_tiles = Vec::new();

    if let Some(top) = adjacent_sides[0]{
        // A bi_gram if either the left or right sides are also set
        if let Some(left) = adjacent_sides[3] {
            let bi_gram = (left, top);
            if let Some(tile_indices) = bi_gram_to_tile_index.get(&bi_gram) {
                for tile_index in tile_indices {
                    if available_mask[*tile_index as usize] {
                        // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 3 });
                        oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 3));
                    }
                }
                return oriented_tiles;
            }
        }
        if let Some(right) = adjacent_sides[1] {
            let bi_gram = (top, right);
            if let Some(tile_indices) = bi_gram_to_tile_index.get(&bi_gram) {
                for tile_index in tile_indices {
                    if available_mask[*tile_index as usize] {
                        // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 1 });
                        oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 1));
                    }
                }
                return oriented_tiles;
            }
        }
        // A uni_gram if the left and right sides are not set
        if let Some(tile_indices) = uni_gram_to_tile_index.get(&top) {
            for tile_index in tile_indices {
                if available_mask[*tile_index as usize] {
                    // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 0 });
                    oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 0));
                }
            }
            return oriented_tiles;
        }
    }
    if let Some(bottom) = adjacent_sides[2]{
        // A bi_gram if either the left or right sides are also set
        if let Some(left) = adjacent_sides[3] {
            let bi_gram = (bottom, left);
            if let Some(tile_indices) = bi_gram_to_tile_index.get(&bi_gram) {
                for tile_index in tile_indices {
                    if available_mask[*tile_index as usize] {
                        // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 2 });
                        oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 2));
                    }
                }
                return oriented_tiles;
            }
        }
        if let Some(right) = adjacent_sides[1] {
            let bi_gram = (right, bottom);
            if let Some(tile_indices) = bi_gram_to_tile_index.get(&bi_gram) {
                for tile_index in tile_indices {
                    if available_mask[*tile_index as usize] {
                        // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 1 });
                        oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 1));
                    }
                }
                return oriented_tiles;
            }
        }
        // A uni_gram if the left and right sides are not set
        if let Some(tile_indices) = uni_gram_to_tile_index.get(&bottom) {
            for tile_index in tile_indices {
                if available_mask[*tile_index as usize] {
                    // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 2 });
                    oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 2));
                }
            }
            return oriented_tiles;
        }
    }
    if let Some(left) = adjacent_sides[3]{
        println!("left: {}", left);
        // Definitely a unigram since we have already explored top and bottom. We dont care about right
        if let Some(tile_indices) = uni_gram_to_tile_index.get(&left) {
            for tile_index in tile_indices {
                if available_mask[*tile_index as usize] {
                    // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 3 });
                    // Print the unoriented tile
                    println!("tile: {} {} {} {}", tile_set[*tile_index as usize].top, tile_set[*tile_index as usize].right, tile_set[*tile_index as usize].bottom, tile_set[*tile_index as usize].left);
                    oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 3));
                }
            }
            return oriented_tiles;
        }
    }
    if let Some(right) = adjacent_sides[1]{
        // Definitely a unigram since we have already explored top and bottom. We dont care about left
        if let Some(tile_indices) = uni_gram_to_tile_index.get(&right) {
            for tile_index in tile_indices {
                if available_mask[*tile_index as usize] {
                    // oriented_tiles.push(OrientedTile { index: *tile_index, orientation: 1 });
                    oriented_tiles.push(OrientedTile::new(*tile_index, tile_set, 1));
                }
            }
            return oriented_tiles;
        }
    }
    Vec::new()
}


fn brute_force_recursive(grid: &mut Vec<Vec<Option<OrientedTile>>>,
                         available_mask: &mut [bool], tileset: &[Tile],
                         tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>,
                         tile_set: &[Tile],
                         search_order: &[(usize, usize)],
                         search_index: usize) -> bool
{
    // If we have reached the end of the search order, we have found a solution
    if search_index == search_order.len() {
        return true;
    }

    // Get the next position to search
    let (x, y) = search_order[search_index];

    // Find the adjacent edges the new tile will need to match. In y,x order because I spent time debugging and this fixed it ¯\_(ツ)_/¯
    let adjacent_sides = get_adjacent_sides(grid, y, x);

    println!("Adjacent sides: {} {} {} {}", adjacent_sides.0, adjacent_sides.1, adjacent_sides.2, adjacent_sides.3);

    // Find the tiles that match the adjacent edges. If the lookup fails just give an empty vec
    let possible_tiles_opt = tile_lookup.get(&adjacent_sides);
    if possible_tiles_opt.is_none() {
        return false;
    }
    let possible_tiles = possible_tiles_opt.unwrap();

    println!("{} possible tiles for ({}, {})", possible_tiles.len(), x, y);


    // Try each possible tile
    for oriented_tile in possible_tiles {
        // Set the tile in the grid
        grid[x][y] = Some(*oriented_tile);

        print_grid(grid);

        println!("Oriented tile: {} {} - {} {} {} {}", oriented_tile.id(), oriented_tile.orientation, oriented_tile.tile.top, oriented_tile.tile.right, oriented_tile.tile.bottom, oriented_tile.tile.left);
        println!("First tile:    {} {} - {} {} {} {}", grid[3][3].unwrap().id(), grid[3][3].unwrap().orientation, grid[3][3].unwrap().top(), grid[3][3].unwrap().right(), grid[3][3].unwrap().bottom(), grid[3][3].unwrap().left());

        // Mark the tile as unavailable
        available_mask[oriented_tile.id() as usize] = false;

        // Recurse
        if brute_force_recursive(grid, available_mask, tileset, tile_lookup, tile_set, search_order, search_index + 1) {
            return true;
        }

        // Unset the tile in the grid
        grid[x][y] = None;

        // Mark the tile as available
        available_mask[oriented_tile.id() as usize] = true;
    }

    // If we get here, we have failed to find a solution
    false
}

fn get_direction(x: usize, y: usize, next_x: usize, next_y: usize) -> usize {
    if next_x == x && next_y < y { return 0; } // up
    if next_x > x && next_y == y { return 1; } // right
    if next_x == x && next_y > y { return 2; } // down
    if next_x < x && next_y == y { return 3; } // left
    panic!("Invalid direction");
}


fn generate_spiral_search_order(grid_size: usize) -> Vec<(usize, usize)> {
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

fn megatile_time() { // This is the old main I can swap to lol

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
    let required_outside_matches = count_tile_types( true, true, false);
    println!("Required outside matches: {:?}", required_outside_matches);

    let required_inside_matches = count_tile_types( false, false, true);
    println!("Required inside matches:       {:?}", required_inside_matches);

    // Matches that shouldn't touch any edge pieces
    let mut required_matches = [0; 22];
    for i in 5..22 {
        required_matches[i] =  required_inside_matches[i] - required_outside_matches[i];
    }
    println!("Required matches:              {:?}", required_matches);

    // Build an index of the tiles to allow searching by their edges
    let tile_lookup = build_indices(&TILE_SET, 60);

    let mut mega_tiles = Vec::new();

    // Now try to find a bunch of 2x2 MegaTiles. Try 1000 times
    for i in 0..100000 {
        mega_tiles = create_mega_tiles(&required_matches, &tile_lookup);
        if mega_tiles.len() > 0 {
            println!("Found {} MegaTiles after {} iterations", mega_tiles.len(), i);
            break;
        }
    }
    // Print the MegaTiles
    println!("");
    println!("Found {} MegaTiles", mega_tiles.len());
    println!("");
    // for mega_tile in mega_tiles.iter() {
    //     visualise_megatile(mega_tile);
    // }

}

fn add_mega_tile_to_available_mask(mega_tile: &MegaTile, available_mask: &mut [bool; 256]) {
    available_mask[mega_tile.tile1.id()] = false;
    available_mask[mega_tile.tile2.id()] = false;
    available_mask[mega_tile.tile3.id()] = false;
    available_mask[mega_tile.tile4.id()] = false;
}

fn add_to_megatile_side_counts(mega_tile: &MegaTile, megatile_edges_count: &mut HashMap<(i8, i8), i32>) {
    // Add entry for top side
    let top_side = (mega_tile.tile1.top(), mega_tile.tile2.top());
    let count = megatile_edges_count.entry(top_side).or_insert(0);
    *count += 1;

    // Add entry for right side
    let right_side = (mega_tile.tile2.right(), mega_tile.tile4.right());
    let count = megatile_edges_count.entry(right_side).or_insert(0);
    *count += 1;

    // Add entry for bottom side
    let bottom_side = (mega_tile.tile4.bottom(), mega_tile.tile3.bottom());
    let count = megatile_edges_count.entry(bottom_side).or_insert(0);
    *count += 1;

    // Add entry for left side
    let left_side = (mega_tile.tile3.left(), mega_tile.tile1.left());
    let count = megatile_edges_count.entry(left_side).or_insert(0);
    *count += 1;
}

fn create_mega_tiles(required_matches: &[i32; 22],
                     tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>) -> Vec<MegaTile>
{
    // Not trying to get a working solution.
    // Just trying to get a solution that doesn't take too long. Can improve later

    return Vec::new();
    //
    // let mut mega_tiles = Vec::with_capacity(49);
    // let mut available_mask = [true; 256];
    //
    // // This is a map between the bi-gram edges of the megatiles encoded as i16 and the number of them we have found so far
    // // The goal is to prioritize making megatiles that match the edge types that don't have a pair yet.
    // // We want to minimise the number of unpaired bi-grams, but we don't need to find all of them since the edges help.
    // let mut megatile_edges_count = HashMap::new();
    //
    // // The code below will crash sometimes. THats how we test for bad solutions. Lets just run it
    // // A few times and ignore any crashes :)
    //
    // for i in 0..49 { // will go up to 49 soon
    //     println!("Building mega tile {}", i);
    //     // Filter the bi-gram
    //
    //     let megatile_edges_that_need_matching = filter_to_unmatched_megatile_edges(&megatile_edges_count);
    //     println!("Found {} unpaired mega tile edges that need friends", megatile_edges_that_need_matching.len());
    //
    //     // Priority level number 1 - find a tile that matches two mega tile edges
    //     let oriented_tiles_that_could_match_two_mega_tile_edges = find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(&megatile_edges_that_need_matching, index_by_bigram, &available_mask);
    //     println!("Found {} tiles that could match two mega tile edges", oriented_tiles_that_could_match_two_mega_tile_edges.len());
    //
    //     for oriented_tile in oriented_tiles_that_could_match_two_mega_tile_edges {
    //         let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&oriented_tile, &megatile_edges_count, &index_by_edge, &index_by_bigram, &available_mask, &megatile_edges_that_need_matching);
    //         if let Some(mega_tile) = maybe_mega_tile {
    //             mega_tiles.push(mega_tile);
    //             break;
    //         }
    //     }
    //     // Exit early if we just added one
    //     if mega_tiles.len() > i {
    //         add_mega_tile_to_available_mask(&mega_tiles[i], &mut available_mask);
    //         add_to_megatile_side_counts(&mega_tiles[i], &mut megatile_edges_count);
    //         continue;
    //     }
    //
    //     // Priority level number 2 - find a tile that matches one mega tile edge
    //     let oriented_tiles_that_could_match_one_mega_tile_one = find_top_left_oriented_tiles_that_matches_one_mega_tile_edge(&megatile_edges_that_need_matching, index_by_edge, &available_mask);
    //     // println!("Found {} tiles that could match two mega tile edges", oriented_tiles_that_could_match_two_mega_tile_edges.len());
    //
    //     for oriented_tile in oriented_tiles_that_could_match_one_mega_tile_one {
    //         let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&oriented_tile, &megatile_edges_count, &index_by_edge, &index_by_bigram, &available_mask, &megatile_edges_that_need_matching);
    //         if let Some(mega_tile) = maybe_mega_tile {
    //             mega_tiles.push(mega_tile);
    //             break;
    //         }
    //     }
    //
    //     // Exit early if we just added one
    //     if mega_tiles.len() > i {
    //         add_mega_tile_to_available_mask(&mega_tiles[i], &mut available_mask);
    //         add_to_megatile_side_counts(&mega_tiles[i], &mut megatile_edges_count);
    //         continue;
    //     }
    //
    //     println!("Making a mega tile the hard way :(");
    //
    //     // Priority level number 3 - just use some other tile :(
    //     // Start at index 60 to avoid the edge and corner pieces
    //     for j in 60..256 {
    //         if available_mask[j] {
    //             for orientation in 0..4 {
    //                 // If piece 139 is available. We want it to be the starting piece. 139 is the mandatory starter for the puzzle
    //                 // We actually store it as 138 though since 1 indexing is a nightmare :)
    //                 if available_mask[138] == true && (j != 138 || orientation != 1) {
    //                     continue;
    //                 }
    //
    //                 let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&OrientedTile::new(j, &TILE_SET, orientation), &megatile_edges_count, &index_by_edge, &index_by_bigram, &available_mask, &megatile_edges_that_need_matching);
    //                 if let Some(mega_tile) = maybe_mega_tile {
    //                     mega_tiles.push(mega_tile);
    //                     break;
    //                 }
    //             }
    //             // Exit early if we just added one
    //             if mega_tiles.len() > i {
    //                 add_mega_tile_to_available_mask(&mega_tiles[i], &mut available_mask);
    //                 add_to_megatile_side_counts(&mega_tiles[i], &mut megatile_edges_count);
    //                 break;
    //             }
    //         }
    //     }
    //     // We couldn't find a tile to add to the mega tile. This is a bad solution
    //     if mega_tiles.len() == i {
    //         println!("Couldn't find a tile to add to the mega tile. This is a bad solution");
    //         return Vec::new();
    //     }
    // }
    // // Holy crap It worked!
    // // Now to verify if this set of megatiles is usable...
    //
    // // How many edges without a counterpart are there? filter_to_unmatched_megatile_edges
    // let megatile_edges_that_need_matching = filter_to_unmatched_megatile_edges(&megatile_edges_count);
    // let count = megatile_edges_that_need_matching.len();
    // println!("Found {} unpaired mega tile edges that need friends", count);
    //
    // if count > 30 {
    //     println!("This is a bad solution");
    //     return Vec::new();
    // }
    //
    // mega_tiles
}

fn try_build_mega_tile(top_left_tile: &OrientedTile,
                       bigram_count: &HashMap<(i8, i8), i32>,
                       index_by_edge: &HashMap<i8, Vec<usize>>,
                       index_by_bigram: &HashMap<(i8, i8), Vec<usize>>,
                       available_mask: &[bool; 256],
                       mega_tile_edges_without_counterpart: &Vec<(i8, i8)>) -> Option<MegaTile>
{
    None
    //
    // // // For debugging see whats in those index by things
    // // for (k, v) in index_by_bigram.iter() {
    // //     if v.len() > 1 {
    // //         println!("({},{}) {:?}", k.0, k.1, v);
    // //     }
    // // }
    // //
    // // for (k, v) in index_by_edge.iter() {
    // //     if v.len() > 1 {
    // //         println!("{}: {:?}", k, v);
    // //     }
    // // }
    //
    // // Available mask shouldn't include the top left tile
    // let mut available_mask = available_mask.clone();
    // available_mask[top_left_tile.id()] = false;
    //
    // // The mega tile will need a combined top edge that matches something in the mega_tile_edges_without_counterpart but backwards
    // // Filter the edges that need friends down to ones whose second element matches the top edge of the top left tile
    // let mega_tile_top_edges = mega_tile_edges_without_counterpart.iter()
    //     .filter(|(a, b)| *b == top_left_tile.top())
    //     .collect::<Vec<_>>();
    //
    // // oriented tiles that could be the top right tile
    // let mut top_right_candidates = Vec::new();
    //
    // for top_edge_to_match in mega_tile_top_edges {
    //     // top_edge_to_match.1 should be the same as top_left_tile.top()
    //     // top_edge_to_match.0 is the one we want on the top of the next tile
    //     let bigram_needed_for_top_right_tile: (i8, i8) = (top_left_tile.right(), top_edge_to_match.0);
    //     if !index_by_bigram.contains_key(&bigram_needed_for_top_right_tile) {
    //         continue;
    //     }
    //     for top_right_tile_index in index_by_bigram.get(&bigram_needed_for_top_right_tile).unwrap() {
    //         if !available_mask[*top_right_tile_index] {
    //             continue;
    //         }
    //         let mask: Tile = Tile { top: top_edge_to_match.0, right: -1, bottom: -1, left: top_left_tile.right(), id: usize::MAX };
    //         panic!("FIX THIS");
    //         // let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*top_right_tile_index, &mask);
    //         // for top_right_tile in tiles_that_match_mask {
    //         //     top_right_candidates.push(top_right_tile);
    //         // }
    //     }
    // }
    //
    // // println!("Found {} Tier 1 top right candidates", top_right_candidates.len());
    //
    // // Fall back and don't worry about the top edge if we haven't found something
    // if top_right_candidates.len() == 0 {
    //     if !index_by_edge.contains_key(&top_left_tile.right()) {
    //         return None;
    //     }
    //     for top_right_tile_index in index_by_edge.get(&top_left_tile.right()).unwrap() {
    //         if !available_mask[*top_right_tile_index] {
    //             continue;
    //         }
    //         let mask: Tile = Tile { top: -1, right: -1, bottom: -1, left: top_left_tile.right(), id: usize::MAX };
    //         panic!("FIX THIS");
    //         // let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*top_right_tile_index, &mask);
    //         // for top_right_tile in tiles_that_match_mask {
    //         //     top_right_candidates.push(top_right_tile);
    //         // }
    //     }
    //     // println!("Found {} Tier 2 top right candidates", top_right_candidates.len());
    // }
    //
    // // The mega tile will need a combined left edge that matches something in the mega_tile_edges_without_counterpart but backwards
    // // Filter the edges that need friends down to ones whose first element matches the left edge of the top left tile
    // let mega_tile_left_edges = mega_tile_edges_without_counterpart.iter()
    //     .filter(|(a, b)| *a == top_left_tile.left())
    //     .collect::<Vec<_>>();
    //
    // // oriented tiles that could be the bottom left tile
    // let mut bottom_left_candidates = Vec::new();
    //
    // for left_edge_to_match in mega_tile_left_edges {
    //     // left_edge_to_match.0 should be the same as top_left_tile.left()
    //     // left_edge_to_match.1 is the one we want on the left of the next tile
    //     let bigram_needed_for_bottom_left_tile: (i8, i8) = (left_edge_to_match.1, top_left_tile.bottom());
    //     if !index_by_bigram.contains_key(&bigram_needed_for_bottom_left_tile) {
    //         continue;
    //     }
    //     for bottom_left_tile_index in index_by_bigram.get(&bigram_needed_for_bottom_left_tile).unwrap() {
    //         if !available_mask[*bottom_left_tile_index] {
    //             continue;
    //         }
    //         let mask: Tile = Tile { top: top_left_tile.bottom(), right: -1, bottom: -1, left: left_edge_to_match.1, id: usize::MAX };
    //         panic!("FIX THIS");
    //         // let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*bottom_left_tile_index, &mask);
    //         // for bottom_left_tile in tiles_that_match_mask {
    //         //     bottom_left_candidates.push(bottom_left_tile);
    //         // }
    //     }
    // }
    //
    // // println!("Found {} Tier 1 bottom left candidates", bottom_left_candidates.len());
    //
    // // Fall back and don't worry about the left edge if we haven't found something
    //
    // if bottom_left_candidates.len() == 0 {
    //     if !index_by_edge.contains_key(&top_left_tile.bottom()) {
    //         return None;
    //     }
    //     for bottom_left_tile_index in index_by_edge.get(&top_left_tile.bottom()).unwrap() {
    //         if !available_mask[*bottom_left_tile_index] {
    //             continue;
    //         }
    //         let mask: Tile = Tile { top: top_left_tile.bottom(), right: -1, bottom: -1, left: -1, id: usize::MAX };
    //         panic!("FIX THIS");
    //         // let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*bottom_left_tile_index, &mask);
    //         // for bottom_left_tile in tiles_that_match_mask {
    //         //     bottom_left_candidates.push(bottom_left_tile);
    //         // }
    //     }
    //     // println!("Found {} Tier 2 bottom left candidates", bottom_left_candidates.len());
    // }
    //
    // // Now we have a list of top right and bottom left tiles that could work
    // // We need to find a pair that works
    // // Tier 1 - We find a fourth tile that matches with both a top right and bottom left tile
    // for top_right_tile in top_right_candidates {
    //     for bottom_left_tile in &bottom_left_candidates {
    //         // skip over cases where the tiles are the same
    //         if top_right_tile.id() == bottom_left_tile.id() {
    //             continue;
    //         }
    //         // Find a tile to go in the bottom right
    //         let bi_gram_needed_for_bottom_right_tile: (i8, i8) = (bottom_left_tile.right(), top_right_tile.bottom());
    //         if !index_by_bigram.contains_key(&bi_gram_needed_for_bottom_right_tile) {
    //             continue;
    //         }
    //         for bottom_right_tile_index in index_by_bigram.get(&bi_gram_needed_for_bottom_right_tile).unwrap() {
    //             if !available_mask[*bottom_right_tile_index] || *bottom_right_tile_index == top_right_tile.id() || *bottom_right_tile_index == bottom_left_tile.id(){
    //                 continue;
    //             }
    //             let mask: Tile = Tile { top: top_right_tile.bottom(), right: -1, bottom: -1, left: bottom_left_tile.right(), id: usize::MAX };
    //             panic!("FIX THIS");
    //             // let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*bottom_right_tile_index, &mask);
    //             // for bottom_right_tile in tiles_that_match_mask {
    //             //     let mega_tile = MegaTile { tile1: *top_left_tile, tile2: top_right_tile, tile3: *bottom_left_tile, tile4: bottom_right_tile };
    //             //     visualise_megatile(&mega_tile);
    //             //     return Some(mega_tile);
    //             // }
    //         }
    //     }
    // }
    // // For now, just return None
    // None
}


fn find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(mega_tile_edges_without_counterpart: &Vec<(i8, i8)>,
                                                                 index_by_bigram: &HashMap<(i8, i8), Vec<usize>>,
                                                                 available_mask: &[bool; 256]) -> Vec<OrientedTile>
{
    // The goal here is to find a tile that can go in the top left of a mega tile
    // Since this tile is in the top left, the top side will have to match the first part of an edge
    // and the left side will have to match the second part of an edge
    // We want to find a tile that matches some combination of the edges that are currently unmatched

    // bi-gram combinations that we should search for in the index
    let mut bigrams_to_search = Vec::new();

    // Cross multiply the two parts of the mega_tile_edges_without_counterpart, exclude i = j
    for i in 0..mega_tile_edges_without_counterpart.len() {
        for j in 0..mega_tile_edges_without_counterpart.len() {
            if i != j {
                bigrams_to_search.push((mega_tile_edges_without_counterpart[i].0, mega_tile_edges_without_counterpart[j].1));
            }
        }
    }

    // Now search the index for the tiles that match the bigram combinations
    let mut tiles_that_could_match_two_mega_tile_edges = Vec::new();
    for bigram in bigrams_to_search {
        if let Some(matching_tile_ids) = index_by_bigram.get(&bigram) {
            for tile_id in matching_tile_ids {
                if available_mask[*tile_id] {
                    // Try out all of the four possible orientations
                    for orientation in 0..4 {
                        let oriented_tile = OrientedTile::new(*tile_id, &TILE_SET, orientation);
                        if oriented_tile.left() == bigram.0 && oriented_tile.top() == bigram.1 {
                            tiles_that_could_match_two_mega_tile_edges.push(oriented_tile);
                        }
                    }
                }
            }
        }
    }

    tiles_that_could_match_two_mega_tile_edges
}

fn find_top_left_oriented_tiles_that_matches_one_mega_tile_edge(mega_tile_edges_without_counterpart: &Vec<(i8, i8)>,
                                                                index_by_edge: &HashMap<i8, Vec<usize>>,
                                                                available_mask: &[bool; 256]) -> Vec<OrientedTile>
{
    // The goal here is to find a tile that can go in the top left of a mega tile
    // Since this tile is in the top left, it's left side will match the first part of an edge
    // and it's top side will match the second part of a bigram

    // Check firest for matches to the first part of the edge
    let mut tiles_that_could_match_one_mega_tile_edge = Vec::new();
    for edge in mega_tile_edges_without_counterpart {
        if let Some(matching_tile_ids) = index_by_edge.get(&edge.0) {
            for tile_id in matching_tile_ids {
                if available_mask[*tile_id] {
                    // Try out all of the four possible orientations
                    for orientation in 0..4 {
                        let oriented_tile = OrientedTile::new(*tile_id, &TILE_SET, orientation);
                        if oriented_tile.left() == edge.0 {
                            tiles_that_could_match_one_mega_tile_edge.push(oriented_tile);
                        } else if oriented_tile.top() == edge.1 {
                            tiles_that_could_match_one_mega_tile_edge.push(oriented_tile);
                        }
                    }
                }
            }
        }
    }
    tiles_that_could_match_one_mega_tile_edge
}


fn filter_to_unmatched_megatile_edges(megatile_edges_count: &HashMap<(i8, i8), i32>) -> Vec<(i8, i8)> {
    // This function finds megatile edges that don't have a counterpart.
    // A counterpart for the edge (F,G) is the edge (G,F)

    // Print the megatile edges and their counts
    for (edge, count) in megatile_edges_count {
        // Print the edge as a char + 65
        // println!("Edge: {}, {}, Count: {}", (edge.0 + 65) as u8 as char, (edge.1 + 65) as u8 as char, count);
    }

    let mut edges_without_counterpart = Vec::new();
    for (edge, count) in megatile_edges_count {
        // Check for identical twins e.g (T,T)
        if edge.0 == edge.1 && count % 2 == 1 {
            edges_without_counterpart.push(*edge);
            continue;
        }

        // Check for the edge's counterpart
        let counterpart = (edge.1, edge.0);
        // We will flag the current edge as unmatched if there are fewer of its counterpart than the current edge
        if let Some(counterpart_count) = megatile_edges_count.get(&counterpart) {
            if count > counterpart_count {
                edges_without_counterpart.push(*edge);
            }
        } else {
            edges_without_counterpart.push(*edge);
        }
    }
    // Build a string to print the edges without a counterpart
    let mut edges_without_counterpart_string = String::new();
    for edge in &edges_without_counterpart {
        edges_without_counterpart_string.push_str(&format!("({}, {}), ", (edge.0 + 65) as u8 as char, (edge.1 + 65) as u8 as char));
    }
    println!("Edges without a counterpart: {}", edges_without_counterpart_string);
    edges_without_counterpart
}

// fn find_tile_index_with_side(side: i8, index_by_edge: &HashMap<i8, Vec<usize>>, available_mask: &[bool; 256]) -> usize {
//     // For now just find the first tile to match in the index
//     let mut index = -1;
//     for i in index_by_edge.get(&side).unwrap() {
//         if available_mask[*i] {
//             index = *i;
//             break;
//         }
//     }
//     index
// }

// fn find_most_needed_tile(required_matches: &[i32; 22]) -> i8 {
//     let mut max = 0;
//     let mut max_index = 0;
//     for i in 0..22 {
//         if required_matches[i] > max {
//             max = required_matches[i];
//             max_index = i;
//         }
//     }
//     max_index as i8
// }

fn find_required_outside_matches(grid: &[[OrientedTile; 16]; 16]) -> [i32; 22]{
    let mut required_outside_matches = [0; 22];
    for i in 1..15 {
        required_outside_matches[grid[0][i].tile_as_rotated().bottom as usize - 1] += 1;
        required_outside_matches[grid[15][i].tile_as_rotated().top as usize - 1] += 1;
        required_outside_matches[grid[i][0].tile_as_rotated().right as usize - 1] += 1;
        required_outside_matches[grid[i][15].tile_as_rotated().left as usize - 1] += 1;
    }
    required_outside_matches
}

fn count_tile_types(corners: bool, edges: bool, center: bool) -> [i32; 22] {
    let mut tile_counts = [0; 22];
    for tile in TILE_SET.iter() {
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
    tile_counts
}