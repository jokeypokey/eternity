// A solver for the eternity 2 puzzle
// https://en.wikipedia.org/wiki/Eternity_II

// The puzzle is a 16x16 grid of 4 sided tiles
// THe goal is to fill the grid by aligning the tiles so that the pattersn match
// The patterns will be denoted with 1 through 22

use std::collections::HashMap;

#[derive(Copy, Clone)]
struct Tile {
    top: i8,
    right: i8,
    bottom: i8,
    left: i8,
}

// Empty tile used for non existent tiles
const EMPTY_TILE: Tile = Tile {top: -1, right: -1, bottom: -1, left: -1};

// make a global array of tiles
const tile_set: [Tile; 256] = load_tiles();

#[derive(Copy, Clone)]
struct OrientedTile {
    tileId: usize,
    orientation: u8, // clockwise: 0 = normal, 1 = 90 degrees , 2 = 180 degrees, 3 = 270 degrees
}


impl OrientedTile {
    // Easily get the top, right, bottom, left sides of a tile given its rotation
    fn tile_as_rotated(&self) -> Tile {
        match self.orientation {
            0 => TILE_SET[self.tileId],
            1 => Tile {
                top: TILE_SET[self.tileId].left,
                right: TILE_SET[self.tileId].top,
                bottom: TILE_SET[self.tileId].right,
                left: TILE_SET[self.tileId].bottom,
            },
            2 => Tile {
                top: TILE_SET[self.tileId].bottom,
                right: TILE_SET[self.tileId].left,
                bottom: TILE_SET[self.tileId].top,
                left: TILE_SET[self.tileId].right,
            },
            3 => Tile {
                top: TILE_SET[self.tileId].right,
                right: TILE_SET[self.tileId].bottom,
                bottom: TILE_SET[self.tileId].left,
                left: TILE_SET[self.tileId].top,
            },
            _ => panic!("Invalid orientation"),
        }
    }

    fn top(&self) -> i8 {
        self.tile_as_rotated().top
    }
    fn right(&self) -> i8 {
        self.tile_as_rotated().right
    }
    fn bottom(&self) -> i8 {
        self.tile_as_rotated().bottom
    }
    fn left(&self) -> i8 {
        self.tile_as_rotated().left
    }

    fn new(tileId: usize, orientation: u8) -> OrientedTile {
        OrientedTile {tileId, orientation}
    }
}

#[derive(Copy, Clone)]
struct MegaTile {
    // A mega tile is a 2x2 grid of tiles
    // ┌1 2┐
    // └3 4┘
    tile1: OrientedTile,
    tile2: OrientedTile,
    tile3: OrientedTile,
    tile4: OrientedTile,
}

fn load_tiles() -> [Tile; 256] {
    // Tiles are stored inside data.csv
    // Each row is a tile
    // The first  column is the tileID, the next 4 columns are the top, right, bottom, left sides

    // Load the data. data.csv is in the same directory as the the src
    let mut reader = csv::Reader::from_path("data.csv").unwrap();

    // Create a thing to store the tiles. We know the size to be 256, so can stack allocate
    let mut tiles = [EMPTY_TILE; 256];


    for (i, result) in reader.records().enumerate() {

        // Get the row
        let record = result.unwrap();

        // The first 4 tiles are corner tiles with no bottom or left side
        // The next 56 tiles are edge tiles with no left side
        // The next 196 tiles are normal tiles

        // The edges of tiles are stored as a number from 1 to 22. I want 0 indexing, with -1 as empty
        // Rust doesn't like going straight to an i8, so we have to do some weird stuff
        if i < 4 { // Corner tile
            tiles [i] = Tile {
                top: i8::from_str_radix(&record[1], 10).unwrap() - 1,
                right: i8::from_str_radix(&record[2], 10).unwrap() - 1,
                bottom: -1,
                left: -1,
            };
        } else if i < 60 { // Edge tile
            tiles [i] = Tile {
                top: i8::from_str_radix(&record[1], 10).unwrap() - 1,
                right: i8::from_str_radix(&record[2], 10).unwrap() - 1,
                bottom: i8::from_str_radix(&record[3], 10).unwrap() - 1,
                left: -1,
            };
        } else { // Normal tile
            tiles [i] = Tile {
                top: i8::from_str_radix(&record[1], 10).unwrap() - 1,
                right: i8::from_str_radix(&record[2], 10).unwrap() - 1,
                bottom: i8::from_str_radix(&record[3], 10).unwrap() - 1,
                left: i8::from_str_radix(&record[4], 10).unwrap() - 1,
            };
        }
    }
    tiles
}

fn fill_grid(grid: &mut [[OrientedTile; 16]; 16]) {
    fill_corners(grid);
    fill_edges(grid);
    fill_normal(grid);
}

fn fill_corners(grid: &mut [[OrientedTile; 16]; 16]) {
    grid [0][0] = OrientedTile {tileId: 0, orientation: 1};
    grid [0][15] = OrientedTile {tileId: 1, orientation: 2};
    grid [15][0] = OrientedTile {tileId: 2, orientation: 0};
    grid [15][15] = OrientedTile {tileId: 3, orientation: 3};
}

fn fill_edges(grid: &mut [[OrientedTile; 16]; 16]) {
    for i in 1..15 {  // Fill the top edge. Use tiles 4 to 17
        grid[0][i] = OrientedTile {tileId: 4 + i - 1, orientation: 1};
    }
    for i in 1..15 {  // Fill the bottom edge. Use tiles 18 to 31
        grid[15][i] = OrientedTile {tileId: 18 + i - 1, orientation: 3};
    }
    for i in 1..15 { // Fill the left edge. Use tiles 32 to 45
        grid[i][0] = OrientedTile {tileId: 32 + i - 1, orientation: 0};
    }
    for i in 1..15 { // Fill the right edge. Use tiles 46 to 59
        grid[i][15] = OrientedTile {tileId: 46 + i - 1, orientation: 2};
    }
}

fn fill_normal(grid: &mut [[OrientedTile; 16]; 16]) {
    for i in 1..15 {
        for j in 1..15 {
            grid[i][j] = OrientedTile {tileId: 60 + (i - 1) * 14 + j - 1, orientation: 0};
        }
    }
}

fn colour_symbol(num: i8, matches: bool, vertical: bool) -> String {
    if num == -1 {
        return if vertical {
            String::from("│")
        } else {
            String::from("───")
        }
    }
    let tile_char = (num + 65) as u8 as char;
    if matches {
        if vertical {
            format!("\x1b[32m{}\x1b[0m", tile_char)
        } else {
            format!("\x1b[32m {} \x1b[0m", tile_char)
        }
    } else {
        if vertical {
            format!("\x1b[31m{}\x1b[0m", tile_char)
        } else {
            format!("\x1b[31m {} \x1b[0m", tile_char)
        }
    }
}

fn visualise_grid(grid: &[[OrientedTile; 16]; 16]) {
    /*
    ┌ I ┐
    R   J
    └ R ┘
    Correct matches are coloured green
    Bad matches are coloured red
    Only the number itself is coloured, not the whole tile.
    */
    let mut match_count = 0;

    for(i, row) in grid.iter().enumerate() {
        // The top line empty string we build on
        let mut top_line = String::new();
        let mut middle_line = String::new();
        let mut bottom_line = String::new();
        for(j, tile) in row.iter().enumerate() {
            let tile_as_rotated = tile.tile_as_rotated();
            let top_matches = tile_as_rotated.top != -1 && tile_as_rotated.top == grid[i - 1][j].tile_as_rotated().bottom;
            let right_matches = tile_as_rotated.right != -1 && tile_as_rotated.right == grid[i][j + 1].tile_as_rotated().left;
            let bottom_matches = tile_as_rotated.bottom != -1 && tile_as_rotated.bottom == grid[i + 1][j].tile_as_rotated().top;
            let left_matches = tile_as_rotated.left != -1 && tile_as_rotated.left == grid[i][j - 1].tile_as_rotated().right;

            match_count += (top_matches as i32) + (right_matches as i32) + (bottom_matches as i32) + (left_matches as i32);

            let top_coloured_symbol = colour_symbol(tile_as_rotated.top, top_matches, false);
            let right_coloured_symbol = colour_symbol(tile_as_rotated.right, right_matches, true);
            let bottom_coloured_symbol = colour_symbol(tile_as_rotated.bottom, bottom_matches, false);
            let left_coloured_symbol = colour_symbol(tile_as_rotated.left, left_matches, true);

            top_line.push_str(&format!("┌{}┐ ", top_coloured_symbol));
            middle_line.push_str(&format!("{}   {} ", left_coloured_symbol, right_coloured_symbol));
            bottom_line.push_str(&format!("└{}┘ ", bottom_coloured_symbol));
        }
        println!("{}", top_line);
        println!("{}", middle_line);
        println!("{}", bottom_line);
    }

    println!("");
    println!("Matches {}/480", match_count/2); // Matches are double counted
}

fn visualise_megatile (megatile: &MegaTile) {
    /*
    ┌ L F ┐
    U     U
    G     G
    └ M T ┘
     */
}

fn main() {
    // I'm just hijacking main so I can run the tile loader.
    // I want to use the tile loader, then use that data to make a rust constant I can use
    // instead of loading from the CSV every time.
    let tiles = load_tiles();

}


fn main2() {
    // Create a grid to store the tiles. They can be oriented in 4 different ways
    let mut grid = [[OrientedTile {tileId: 0, orientation: 0}; 16]; 16];

    // Print the tiles
    // for tile in tiles.iter() {
    //     println!("{} {} {} {}", tile.top, tile.right, tile.bottom, tile.left);
    // }

    // Lets print some data on all the various tile types
    println!("There are {} tiles", TILE_SET.len());
    println!("There are {} corner tiles", TILE_SET.iter().filter(|t| t.bottom == -1 && t.left == -1).count());
    println!("There are {} edge tiles", TILE_SET.iter().filter(|t| t.bottom != -1 && t.left == -1).count());
    println!("There are {} normal tiles", TILE_SET.iter().filter(|t| t.bottom != -1 && t.left != -1).count());

    // Fill the grid
    fill_grid(&mut grid);

    visualise_grid(&grid);

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
    let (index_by_edge, index_by_bigram) = build_indices();

    // Now try to find a bunch of 2x2 MegaTiles
    let mega_tiles = create_mega_tiles(&required_matches, &index_by_edge, &index_by_bigram);

}

fn build_indices() -> (HashMap<i8, Vec<usize>>, HashMap<(i8, i8), Vec<usize>>) {
    let mut index_unigram = HashMap::new();
    let mut index_bigram = HashMap::new();

    for (i, tile) in TILE_SET.iter().enumerate() {
        index_unigram.entry(tile.top).or_insert(Vec::new()).push(i);
        index_unigram.entry(tile.right).or_insert(Vec::new()).push(i);
        index_unigram.entry(tile.bottom).or_insert(Vec::new()).push(i);
        index_unigram.entry(tile.left).or_insert(Vec::new()).push(i);

        index_bigram.entry((tile.top, tile.right)).or_insert(Vec::new()).push(i);
        index_bigram.entry((tile.right, tile.bottom)).or_insert(Vec::new()).push(i);
        index_bigram.entry((tile.bottom, tile.left)).or_insert(Vec::new()).push(i);
        index_bigram.entry((tile.left, tile.top)).or_insert(Vec::new()).push(i);
    }
    (index_unigram, index_bigram)
}

fn create_mega_tiles(required_matches: &[i32; 22],
                     index_by_edge: &HashMap<i8, Vec<usize>>,
                     index_by_bigram: &HashMap<(i8, i8), Vec<usize>>) -> Vec<MegaTile>
{
    // Not trying to get a working solution.
    // Just trying to get a solution that doesn't take too long. Can improve later

    let mut mega_tiles = Vec::with_capacity(49);
    let mut available_mask = [true; 256];

    // This is a map between the bi-gram edges of the megatiles encoded as i16 and the number of them we have found so far
    // The goal is to prioritize making megatiles that match the edge types with an odd number of matches.
    // We want to minimise the number of unpaired bi-grams, but we don't need to find all of them since the edges help.
    let mut bigram_count = HashMap::new();

    for i in 0..49 {
        // Filter the bigram
        let odd_mega_tile_edges_that_need_friends = find_filtered_to_odd_count_mega_tile_sides(&bigram_count);
        let oriented_tiles_that_could_match_two_mega_tile_edges = find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(&odd_mega_tile_edges_that_need_friends, index_by_bigram, &available_mask);

        // Priority level number 1 - find a tile that matches two mega tile edges
        for oriented_tile in oriented_tiles_that_could_match_two_mega_tile_edges {
            // Try to build a mega tile from this
            println!("Trying to build a mega tile from tile");
        }

        // Priority level number 2 - find a tile that matches one mega tile edge
    }
    mega_tiles
}

fn find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(odd_mega_tile_edges_that_need_friends: &Vec<(i8, i8)>,
                                                       index_by_bigram: &HashMap<(i8, i8), Vec<usize>>,
                                                       available_mask: &[bool; 256]) -> Vec<OrientedTile>
{
    // The goal here is to find a tile that can go in the top left of a mega tile
    // Since this tile is in the top left, the top side will have to match the first part of a bi-gram
    // and the left side will have to match the second part of a bi-gram
    // We want to find a tile that matches some combination of the bi-grams that are currently odd

    // bi-gram combinations that we should search for in the index
    let mut bigrams_to_search = Vec::new();

    // Cross multiply the two parts of the odd_mega_tile_edges_that_need_friends, exclude i = j
    for i in 0..odd_mega_tile_edges_that_need_friends.len() {
        for j in 0..odd_mega_tile_edges_that_need_friends.len() {
            if i != j {
                bigrams_to_search.push((odd_mega_tile_edges_that_need_friends[i].0, odd_mega_tile_edges_that_need_friends[j].1));
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
                        let oriented_tile = OrientedTile::new(*tile_id, orientation);
                        tiles_that_could_match_two_mega_tile_edges.push(oriented_tile);
                    }
                }
            }
        }
    }

    tiles_that_could_match_two_mega_tile_edges
}

fn find_filtered_to_odd_count_mega_tile_sides(bigram_count: &HashMap<(i8, i8), i32>) -> Vec<(i8, i8)> {
    let mut odd_mega_tile_edges_that_need_friends = Vec::new();
    for (edge, count) in bigram_count {
        if count % 2 == 1 {
            odd_mega_tile_edges_that_need_friends.push(*edge);
        }
    }
    odd_mega_tile_edges_that_need_friends
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