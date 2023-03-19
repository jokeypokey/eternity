// A solver for the eternity 2 puzzle
// https://en.wikipedia.org/wiki/Eternity_II

// The puzzle is a 16x16 grid of 4 sided tiles
// THe goal is to fill the grid by aligning the tiles so that the pattersn match
// The patterns will be denoted with 1 through 22

use std::collections::HashMap;

// Another file called data.rs to load in


mod data;
mod mega;
mod display;
mod helper;

use crate::data::*;
use crate::display::*;
use crate::helper::*;
use crate::mega::*;
use std::thread;

fn main() {
    // create_mega_tiles();
    // brute_force(7, &MINI_TILE_SET, OrientedTile::new(24, &MINI_TILE_SET, 0));

    // Spawn 8 different threads to create_mega_tiles();
    // If any one thread completes, stop execution

    create_mega_tiles();
}


fn get_adjacent_sides(grid: &Vec<Vec<Option<OrientedTile>>>, x: usize, y: usize) -> (i8, i8, i8, i8) {
    let mut adjacent_sides = (-1, -1, -1, -1);
    if y > 0 && grid[y - 1][x].is_some() { adjacent_sides.0 = grid[y - 1][x].unwrap().bottom(); }
    if x < grid[0].len() - 1 && grid[y][x + 1].is_some() { adjacent_sides.1 = grid[y][x + 1].unwrap().left(); }
    if y < grid.len() - 1 && grid[y + 1][x].is_some() { adjacent_sides.2 = grid[y + 1][x].unwrap().top(); }
    if x > 0 && grid[y][x - 1].is_some() { adjacent_sides.3 = grid[y][x - 1].unwrap().right(); }
    adjacent_sides
}

fn brute_force(grid_size: usize, tile_set: &[Tile], centre_tile: OrientedTile) {
    let mut grid = vec![vec![None; grid_size]; grid_size];
    let tile_lookup = build_indices(tile_set, 0);

    grid[grid_size / 2][grid_size / 2] = Some(centre_tile);

    let mut available_mask = Vec::new();
    for i in 0..tile_set.len() { available_mask.push(i == centre_tile.id()); }


    let search_order = generate_spiral_search_order(grid_size);

    if brute_force_recursive(&mut grid, &mut available_mask, tile_set, &tile_lookup, tile_set, &search_order, 1) {
        print_grid(&grid); // We did it!
    } else {
        println!("Failed to find a solution :(");
    }
}

fn brute_force_recursive(grid: &mut Vec<Vec<Option<OrientedTile>>>,
                         available_mask: &mut [bool], tileset: &[Tile],
                         tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>,
                         tile_set: &[Tile],
                         search_order: &[(usize, usize)],
                         search_index: usize) -> bool
{
    // Found a complete solution! return early
    if search_index == search_order.len() { return true; }

    let (x, y) = search_order[search_index];
    let possible_tiles_opt = tile_lookup.get(&get_adjacent_sides(grid, y, x));

    // We can't keep going... Backtrack time
    if possible_tiles_opt.is_none() { return false; }

    for oriented_tile in possible_tiles_opt.unwrap() {
        grid[x][y] = Some(*oriented_tile);
        available_mask[oriented_tile.id() as usize] = false;
        if brute_force_recursive(grid, available_mask, tileset, tile_lookup, tile_set, search_order, search_index + 1) {
            return true;
        }
        available_mask[oriented_tile.id() as usize] = true;
        grid[x][y] = None;
    }
    false // None of the branches worked
}

fn add_mega_tile_to_available_mask(mega_tile: &MegaTile, available_mask: &mut [bool; 256]) {
    available_mask[mega_tile.tiles[0].id()] = false;
    available_mask[mega_tile.tiles[1].id()] = false;
    available_mask[mega_tile.tiles[2].id()] = false;
    available_mask[mega_tile.tiles[3].id()] = false;
}

// fn add_to_megatile_side_counts(mega_tile: &MegaTile, megatile_edges_count: &mut HashMap<(i8, i8), i32>) {
//     // Add entry for top side
//     let top_side = (mega_tile.tiles[0].top(), mega_tile.tile2.top());
//     let count = megatile_edges_count.entry(top_side).or_insert(0);
//     *count += 1;
//
//     // Add entry for right side
//     let right_side = (mega_tile.tile2.right(), mega_tile.tile4.right());
//     let count = megatile_edges_count.entry(right_side).or_insert(0);
//     *count += 1;
//
//     // Add entry for bottom side
//     let bottom_side = (mega_tile.tile4.bottom(), mega_tile.tile3.bottom());
//     let count = megatile_edges_count.entry(bottom_side).or_insert(0);
//     *count += 1;
//
//     // Add entry for left side
//     let left_side = (mega_tile.tile3.left(), mega_tile.tile1.left());
//     let count = megatile_edges_count.entry(left_side).or_insert(0);
//     *count += 1;
// }
//
// fn create_mega_tiles(required_matches: &[i32; 22],
//                      tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>) -> Vec<MegaTile>
// {
//     // Not trying to get a working solution.
//     // Just trying to get a solution that doesn't take too long. Can improve later
//
//     return Vec::new();
//     //
//     // let mut mega_tiles = Vec::with_capacity(49);
//     // let mut available_mask = [true; 256];
//     //
//     // // This is a map between the bi-gram edges of the megatiles encoded as i16 and the number of them we have found so far
//     // // The goal is to prioritize making megatiles that match the edge types that don't have a pair yet.
//     // // We want to minimise the number of unpaired bi-grams, but we don't need to find all of them since the edges help.
//     // let mut megatile_edges_count = HashMap::new();
//     //
//     // // The code below will crash sometimes. THats how we test for bad solutions. Lets just run it
//     // // A few times and ignore any crashes :)
//     //
//     // for i in 0..49 { // will go up to 49 soon
//     //     println!("Building mega tile {}", i);
//     //     // Filter the bi-gram
//     //
//     //     let megatile_edges_that_need_matching = filter_to_unmatched_megatile_edges(&megatile_edges_count);
//     //     println!("Found {} unpaired mega tile edges that need friends", megatile_edges_that_need_matching.len());
//     //
//     //     // Priority level number 1 - find a tile that matches two mega tile edges
//     //     let oriented_tiles_that_could_match_two_mega_tile_edges = find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(&megatile_edges_that_need_matching, index_by_bigram, &available_mask);
//     //     println!("Found {} tiles that could match two mega tile edges", oriented_tiles_that_could_match_two_mega_tile_edges.len());
//     //
//     //     for oriented_tile in oriented_tiles_that_could_match_two_mega_tile_edges {
//     //         let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&oriented_tile, &megatile_edges_count, &index_by_edge, &index_by_bigram, &available_mask, &megatile_edges_that_need_matching);
//     //         if let Some(mega_tile) = maybe_mega_tile {
//     //             mega_tiles.push(mega_tile);
//     //             break;
//     //         }
//     //     }
//     //     // Exit early if we just added one
//     //     if mega_tiles.len() > i {
//     //         add_mega_tile_to_available_mask(&mega_tiles[i], &mut available_mask);
//     //         add_to_megatile_side_counts(&mega_tiles[i], &mut megatile_edges_count);
//     //         continue;
//     //     }
//     //
//     //     // Priority level number 2 - find a tile that matches one mega tile edge
//     //     let oriented_tiles_that_could_match_one_mega_tile_one = find_top_left_oriented_tiles_that_matches_one_mega_tile_edge(&megatile_edges_that_need_matching, index_by_edge, &available_mask);
//     //     // println!("Found {} tiles that could match two mega tile edges", oriented_tiles_that_could_match_two_mega_tile_edges.len());
//     //
//     //     for oriented_tile in oriented_tiles_that_could_match_one_mega_tile_one {
//     //         let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&oriented_tile, &megatile_edges_count, &index_by_edge, &index_by_bigram, &available_mask, &megatile_edges_that_need_matching);
//     //         if let Some(mega_tile) = maybe_mega_tile {
//     //             mega_tiles.push(mega_tile);
//     //             break;
//     //         }
//     //     }
//     //
//     //     // Exit early if we just added one
//     //     if mega_tiles.len() > i {
//     //         add_mega_tile_to_available_mask(&mega_tiles[i], &mut available_mask);
//     //         add_to_megatile_side_counts(&mega_tiles[i], &mut megatile_edges_count);
//     //         continue;
//     //     }
//     //
//     //     println!("Making a mega tile the hard way :(");
//     //
//     //     // Priority level number 3 - just use some other tile :(
//     //     // Start at index 60 to avoid the edge and corner pieces
//     //     for j in 60..256 {
//     //         if available_mask[j] {
//     //             for orientation in 0..4 {
//     //                 // If piece 139 is available. We want it to be the starting piece. 139 is the mandatory starter for the puzzle
//     //                 // We actually store it as 138 though since 1 indexing is a nightmare :)
//     //                 if available_mask[138] == true && (j != 138 || orientation != 1) {
//     //                     continue;
//     //                 }
//     //
//     //                 let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&OrientedTile::new(j, &TILE_SET, orientation), &megatile_edges_count, &index_by_edge, &index_by_bigram, &available_mask, &megatile_edges_that_need_matching);
//     //                 if let Some(mega_tile) = maybe_mega_tile {
//     //                     mega_tiles.push(mega_tile);
//     //                     break;
//     //                 }
//     //             }
//     //             // Exit early if we just added one
//     //             if mega_tiles.len() > i {
//     //                 add_mega_tile_to_available_mask(&mega_tiles[i], &mut available_mask);
//     //                 add_to_megatile_side_counts(&mega_tiles[i], &mut megatile_edges_count);
//     //                 break;
//     //             }
//     //         }
//     //     }
//     //     // We couldn't find a tile to add to the mega tile. This is a bad solution
//     //     if mega_tiles.len() == i {
//     //         println!("Couldn't find a tile to add to the mega tile. This is a bad solution");
//     //         return Vec::new();
//     //     }
//     // }
//     // // Holy crap It worked!
//     // // Now to verify if this set of megatiles is usable...
//     //
//     // // How many edges without a counterpart are there? filter_to_unmatched_megatile_edges
//     // let megatile_edges_that_need_matching = filter_to_unmatched_megatile_edges(&megatile_edges_count);
//     // let count = megatile_edges_that_need_matching.len();
//     // println!("Found {} unpaired mega tile edges that need friends", count);
//     //
//     // if count > 30 {
//     //     println!("This is a bad solution");
//     //     return Vec::new();
//     // }
//     //
//     // mega_tiles
// }

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

