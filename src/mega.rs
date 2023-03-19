use std::collections::HashMap;
use rand::Rng;
use crate::data::*;
use crate::display::*;
use crate::helper::*;

pub fn create_mega_tiles() -> Vec<MegaTile> {
    // Creates a set of 49 mega-tiles that could potentially be used in a solution

    // Step 1 - Spawn a bunch of valid mega-tiles, trying to get good repetition in edge types
    let mega_tiles = generate_initial_mega_tiles();

    for mega_tile in mega_tiles.iter(){
        visualise_mega_tile(mega_tile);
        for tile in mega_tile.tiles.iter() {
            println!("{}x{}: {} {} {} {}", tile.id(), tile.orientation, tile.top(), tile.right(), tile.bottom(), tile.left());
        }
    }

    print_mega_tile_stats(&mega_tiles);

    // Verify mega tiles
    verify_mega_tiles(&mega_tiles);


    // Step 2 - Massage the mega-tiles to make them fit the constraints of the rim pieces


    Vec::new()
}

fn verify_mega_tiles(mega_tiles: &Vec<MegaTile>){
    // Verify that we have used all of the tiles
    let mut used_tiles = [false; 256];
    for mega_tile in mega_tiles.iter() {
        for tile in mega_tile.tiles.iter() {
            used_tiles[tile.id() as usize] = true;
        }
    }
    let mut used_tile_count = 0;
    let max_tile_count = 49 * 4;
    let mut unused_tile_list = Vec::new();
    for i in 60..256 { // Ignore the corner and edge pieces
        if used_tiles[i] {
            used_tile_count += 1;
        }else{
            println!("Tile {} was not used", i);
            unused_tile_list.push(i);
        }
    }
    if used_tile_count != max_tile_count {
        println!("Used {} tiles, expected {}", used_tile_count, max_tile_count);
        panic!("Not all tiles were used");
    }

}

fn print_mega_tile_stats(mega_tiles: &Vec<MegaTile>) {
    // We are most interested in the edge data
    let mut edge_counts: HashMap<(i8, i8), usize> = HashMap::new();
    for mega_tile in mega_tiles.iter() {
        for edge in [mega_tile.top(), mega_tile.right(), mega_tile.bottom(), mega_tile.left()].iter() {
            let count = edge_counts.entry(*edge).or_insert(0);
            *count += 1;
        }
    }

    // Print the edge counts
    println!("Edge counts:");
    for (edge, count) in edge_counts.iter() {
        // Print the edges as letters for easier reading
        // Need to add 65 to the char code to get the right letter
        println!("({}, {})x{:2}", (edge.0 + 65) as u8 as char, (edge.1 + 65) as u8 as char, count);
    }

    // Print the edge counts alongside their conjugates
    println!("Edge counts with conjugates:");
    let mut conjugates_already_printed = Vec::new();
    for (edge, count) in edge_counts.iter() {
        if conjugates_already_printed.contains(edge) {
            continue;
        }
        let conjugate = (edge.1, edge.0);
        // Check for symmetric tiles
        if edge.0 == edge.1{
            let mut line = format!("({}, {}):{:2}", (edge.0 + 65) as u8 as char, (edge.1 + 65) as u8 as char, count);
            // Red if green otherwise
            if count % 2 == 1 {
                line = format!("\x1b[31m{}\x1b[0m", line);
            } else {
                line = format!("\x1b[32m{}\x1b[0m", line);
            }
            println!("{}", line);

            continue;
        }
        let conjugate_count = edge_counts.get(&conjugate).unwrap_or(&0);


        // We will print them like (A, B)x2 (B, A)x1 - The line will be green if the counts match, otherwise red
        let mut line = format!("({}, {})x{:2} ({}, {})x{:2}", (edge.0 + 65) as u8 as char, (edge.1 + 65) as u8 as char, count, (edge.1 + 65) as u8 as char, (edge.0 + 65) as u8 as char, conjugate_count);
        if count == conjugate_count {
            line = format!("\x1b[32m{}\x1b[0m", line);
        } else {
            line = format!("\x1b[31m{}\x1b[0m", line);
        }
        println!("{}", line);


        conjugates_already_printed.push(conjugate);
    }
}

fn generate_needed_edge_sides() -> [i32; 22]{
    // Create some constraints for the mega_tiles based on the edge pieces
    let required_outside_matches = count_tile_types(&TILE_SET, true, true, false, false);
    let required_inside_matches = count_tile_types(&TILE_SET, false, false, true, false);

    // Matches that shouldn't touch any edge pieces
    let mut required_sides_for_edges = [0; 22];
    for i in 5..22 {
        required_sides_for_edges[i] =  required_inside_matches[i] - required_outside_matches[i];
    }
    println!("Required sides for edges: {:?}", required_sides_for_edges);
    panic!("Stop here");
    required_sides_for_edges

}

fn generate_initial_mega_tiles() -> Vec<MegaTile> {
    // Recursively try to make mega-tiles until we have a good set of them

    print_random_data();

    let tiles = &TILE_SET;
    let total_edge_count = count_tile_types(tiles, true, true, true, true);
    let mut available_mask = [true; 256];
    let mut mega_tiles: Vec<MegaTile> = Vec::new();

    let wanted_sides_for_hints = generate_wanted_hint_sides();
    let needed_sides_for_edges = count_tile_types(&TILE_SET, true, true, false, true);
    let tile_lookup = build_indices(tiles, 60); // Skip 60 because they are edge and corner.
    let mut edge_usage_count = [0; 22];

    // Track the count of each edge variant so we can focus on pairing and repetition
    let mut mega_tile_edge_count: HashMap<(i8, i8), usize> = HashMap::new();

    // Tier 1 - Unpaired edges
    let mut unpaired_edges: Vec<(i8, i8)> = Vec::new();

    // Tier 2 - Paired edges - Second priority
    let mut paired_edges: Vec<(i8, i8)> = Vec::new();

    // Tier 3 - Rim optimization - Try to include pieces that appear infrequently in the rim
    let mut rim_optimization: Vec<(i8, i8)> = Vec::new();

    // Tier 4 - If it works it works. Just get the internals going.

    // Before starting, remove the hint tiles from the available set
    for tile in tiles.iter() {
        if tile.id == 138 {
            available_mask[tile.id as usize] = false;
        }
        if IS_USING_HINTS && (tile.id == 207 || tile.id == 254 || tile.id == 180 || tile.id == 248){
            available_mask[tile.id as usize] = false;
        }
    }
    build_mega_tiles_recursive(&mut available_mask, &mut mega_tiles, &mut mega_tile_edge_count, &mut unpaired_edges, &mut paired_edges, &mut rim_optimization, &tile_lookup, &wanted_sides_for_hints, &needed_sides_for_edges, &mut edge_usage_count, &total_edge_count);
    mega_tiles
}

fn check_healthy_tileset(mega_tiles: &Vec<MegaTile>,
                         mega_tile_edge_count: &HashMap<(i8, i8), usize>,
                         unpaired_edges: &Vec<(i8, i8)>,
                         paired_edges: &Vec<(i8, i8)>,
                         rim_optimization: &Vec<(i8, i8)>,
                         mega_tile_count: usize,
                         used_edge_count:&[i32; 22],
                         total_edge_count: &[i32; 22],
                         needed_sides_for_edges: &[i32; 22],
) -> bool {

    // Not super good
    // match mega_tile_count {
    //     0..=3 => if unpaired_edges.len() > 8 { return false; },
    //     4..=7 => if unpaired_edges.len() > 12 { return false; },
    //     8..=15 => if unpaired_edges.len() > 16 { return false; },
    //     16..=23 => if unpaired_edges.len() > 20 { return false; },
    //     24..=31 => if unpaired_edges.len() > 24 { return false; },
    //     32..=39 => if unpaired_edges.len() > 26 { return false; },
    //     40..=49 => if unpaired_edges.len() > 28 { return false; },
    //     _ => if unpaired_edges.len() > 28 { return false; },
    // }
    let total_unique_edge_types = mega_tile_edge_count.len();

    // Stricter count
    match mega_tile_count {
        0..=3 => if unpaired_edges.len() > 6 { return false; },
        4..=7 => if unpaired_edges.len() > 7 { return false; },
        8..=15 => if unpaired_edges.len() > 8 { return false; },
        16..=23 => if unpaired_edges.len() > 9 { return false; },
        24..=31 => if unpaired_edges.len() > 10 { return false; },
        32..=39 => if unpaired_edges.len() > 14 { return false; },
        40..=43 => if unpaired_edges.len() > 15 { return false; },
        44 => if unpaired_edges.len() > 15 { return false; },
        45 => if unpaired_edges.len() > 16 { return false; },
        46 => if unpaired_edges.len() > 17 { return false; },
        47 => if unpaired_edges.len() > 19 { return false; },
        48 => if unpaired_edges.len() > 20 { return false; },
        49 => if unpaired_edges.len() > 22 { return false; },
        _ => if unpaired_edges.len() > 30 { return false; },
    }

    // For this one I'm just gonna make stuff up

    match mega_tile_count {
        0..=3   => if total_unique_edge_types > 9 { return false; },
        4..=6  => if total_unique_edge_types > 12 + (((mega_tile_count-3) / 2) * 2) { return false; },
        7..=14  => if total_unique_edge_types > 13 + (((mega_tile_count-3) / 2) * 2) { return false; },
        15..=21  => if total_unique_edge_types > 13 + (((mega_tile_count-3) / 2) * 2) { return false; },
        22..=27  => if total_unique_edge_types > 13 + (((mega_tile_count-3) / 2) * 2) { return false; },
        28..=32  => if total_unique_edge_types > 13 + (((mega_tile_count-3) / 2) * 2) { return false; },
        33..=40  => if total_unique_edge_types > 13 + (((mega_tile_count-3) / 2) * 2) { return false; },
        41..=42  => if total_unique_edge_types > 16 + (((mega_tile_count-3) / 2) * 2) { return false; },
        43..=44  => if total_unique_edge_types > 22 + (((mega_tile_count-3) / 2) * 2) { return false; },
        45  => if total_unique_edge_types > 24 + (((mega_tile_count-3) / 2) * 2) { return false; },
        46  => if total_unique_edge_types > 25 + (((mega_tile_count-3) / 2) * 2) { return false; },
        47  => if total_unique_edge_types > 27 + (((mega_tile_count-3) / 2) * 2) { return false; },
        48  => if total_unique_edge_types > 29 + (((mega_tile_count-3) / 2) * 2) { return false; },
        49  => if total_unique_edge_types > 31 + (((mega_tile_count-3) / 2) * 2) { return false; },

        // 33..=45  => if total_unique_edge_types > 22 + ((mega_tile_count-3) / 3) * 2 { return false; },
        // 46  => if total_unique_edge_types > 23 + ((mega_tile_count-3) / 3) * 2 { return false; },
        // 47  => if total_unique_edge_types > 24 + ((mega_tile_count-3) / 3) * 2 { return false; },
        // 48  => if total_unique_edge_types > 25 + ((mega_tile_count-3) / 3) * 2 { return false; },
        // 49  => if total_unique_edge_types > 26 + ((mega_tile_count-3) / 3) * 2 { return false; },
        _ => if total_unique_edge_types > 30 { return false; },
    }

    // Make sure we have enough edges to fill the rim
    for i in 5..22 {
        if used_edge_count[i] + needed_sides_for_edges[i] > total_edge_count[i] {
            println!("Failed edge count check: {} + {} > {}", used_edge_count[i], needed_sides_for_edges[i], total_edge_count[i]);
            return false;
        }
    }
    println!("mega_tile_count: {:2}, Unique edge count: {:2}, Unpaired edge count: {:2}", mega_tile_count, total_unique_edge_types, unpaired_edges.len());
    true
}


fn build_mega_tiles_recursive(available_mask: &mut [bool; 256],
                              mega_tiles: &mut Vec<MegaTile>,
                              mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                              unpaired_edges: &mut Vec<(i8, i8)>,
                              paired_edges: &mut Vec<(i8, i8)>,
                              rim_optimization: &mut Vec<(i8, i8)>,
                              tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>,
                              wanted_sides_for_hints: &[i32; 22],
                              needed_sides_for_edges: &[i32; 22],
                              used_edge_count: &mut [i32; 22],
                              total_edge_count: &[i32; 22]) -> bool
{
    let mega_tile_count = mega_tiles.len();

    // Check if we are within acceptable boundaries on the number of unpaired_edges
    if !check_healthy_tileset(mega_tiles, mega_tile_edge_count, unpaired_edges, paired_edges, rim_optimization, mega_tile_count, used_edge_count, total_edge_count, needed_sides_for_edges){
        return false;
    }

    // wasdwasd
    if mega_tile_count == 49 { // Stopping one early because holy heck its hard to compute
        return true;
    }

    // Get a list of all the valid seeds for this mega-tile
    let edge_type_usage_score = get_edge_type_usage_score(wanted_sides_for_hints, needed_sides_for_edges, used_edge_count, total_edge_count);
    let sorted_by_score_seeds = get_valid_mega_tile_seeds(available_mask, mega_tile_edge_count, unpaired_edges, paired_edges, rim_optimization, mega_tile_count, tile_lookup);
    let sorted_by_score_mega_tiles = get_all_possible_megatiles_sorted_by_score(&sorted_by_score_seeds, available_mask, mega_tile_edge_count, unpaired_edges, paired_edges, rim_optimization, mega_tile_count, tile_lookup, &edge_type_usage_score);

    // println!("{} Mega tiles so far, {} seeds this step, {} options this step", mega_tile_count, sorted_by_score_seeds.len(), sorted_by_score_mega_tiles.len());

    // Recurse
    for mega_tile in sorted_by_score_mega_tiles.iter() {

        // Add the mega tile to the list
        mega_tiles.push(mega_tile.clone());

        // Add the mega tile
        add_mega_tile_to_trackers(mega_tile, available_mask, mega_tile_edge_count, unpaired_edges, mega_tile_count, used_edge_count);
        recalculate_paired_and_unpaired_counts(mega_tile_edge_count, unpaired_edges, paired_edges);

        // Recurse
        if build_mega_tiles_recursive(available_mask, mega_tiles, mega_tile_edge_count, unpaired_edges, paired_edges, rim_optimization, tile_lookup, wanted_sides_for_hints, needed_sides_for_edges, used_edge_count, total_edge_count) {
            return true;
        }

        // Remove the mega tile from the list
        remove_mega_tiles_from_trackers(mega_tile, available_mask, mega_tile_edge_count, unpaired_edges, mega_tile_count, used_edge_count);
        recalculate_paired_and_unpaired_counts(mega_tile_edge_count, unpaired_edges, paired_edges);

        // Remove the mega tile from the list
        mega_tiles.pop();

        // Add the tiles back to the available set
        for tile in mega_tile.tiles.iter() {
            available_mask[tile.id() as usize] = true;
        }
    }

    return false;
}

fn get_edge_type_usage_score(wanted_sides_for_hints: &[i32; 22],
                             needed_sides_for_edges: &[i32; 22],
                             used_edge_count: &[i32; 22],
                             total_edge_count: &[i32; 22]) -> [i32; 22] {
    // Figure out which edges types we want to favour in the scoring system
    let mut edge_type_usage_score = [0; 22];


    // return total - used - wanted - needed
    for i in 0..22 {
        edge_type_usage_score[i] = total_edge_count[i] - used_edge_count[i] - wanted_sides_for_hints[i] - needed_sides_for_edges[i];
    }
    edge_type_usage_score
}

fn add_mega_tile_to_trackers(mega_tile: &MegaTile,
                             available_mask: &mut [bool; 256],
                             mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                             rim_optimization: &mut Vec<(i8, i8)>,
                             mega_tile_count: usize,
                             edge_usage_count: &mut [i32; 22]) {
    // Update available_mask
    for tile in mega_tile.tiles.iter() {
        available_mask[tile.id() as usize] = false;
        // edge usage count
        for edge in [tile.top(), tile.right(), tile.bottom(), tile.left()].iter() {
            edge_usage_count[*edge as usize] += 1;
        }
    }

    // Update mega_tile_edge_count
    for edge in [mega_tile.top(), mega_tile.right(), mega_tile.bottom(), mega_tile.left()].iter() {
        let count = mega_tile_edge_count.entry(*edge).or_insert(0);
        *count += 1;
    }
}

fn remove_mega_tiles_from_trackers(mega_tile: &MegaTile,
                                   available_mask: &mut [bool; 256],
                                   mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                                   rim_optimization: &mut Vec<(i8, i8)>,
                                   mega_tile_count: usize,
                                   edge_usage_count: &mut [i32; 22]) {
    // Update available_mask
    for tile in mega_tile.tiles.iter() {
        // Don't add the hint tiles back to the available set
        if tile.id() == 138 || IS_USING_HINTS && (tile.id() == 207 || tile.id() == 254 || tile.id() == 180 || tile.id() == 248) {
            continue;
        }
        available_mask[tile.id() as usize] = true;
        // edge usage count
        for edge in [tile.top(), tile.right(), tile.bottom(), tile.left()].iter() {
            edge_usage_count[*edge as usize] -= 1;
        }
    }

    // Update mega_tile_edge_count
    for edge in [mega_tile.top(), mega_tile.right(), mega_tile.bottom(), mega_tile.left()].iter() {
        // Remove the entry from the hashmap if 1, otherwise decrement
        let count = mega_tile_edge_count.get_mut(edge).unwrap();
        if *count == 1 {
            mega_tile_edge_count.remove(edge);
        } else {
            *count -= 1;
        }
    }
}

fn recalculate_paired_and_unpaired_counts(mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                                          unpaired_edges: &mut Vec<(i8, i8)>,
                                          paired_edges: &mut Vec<(i8, i8)>)
{
    unpaired_edges.clear();
    paired_edges.clear();
    for edge in mega_tile_edge_count.keys() {
        let count = mega_tile_edge_count.get(edge).unwrap_or(&0);
        if edge.0 == edge.1 {
            if count % 2 == 0 {
                paired_edges.push(*edge);
                continue;
            } else {
                unpaired_edges.push(*edge);
                continue;
            }
        }
        let conjugate = (edge.1, edge.0);
        // Get the conjugate count or 0 if it doesn't exist
        let conjugate_count = mega_tile_edge_count.get(&conjugate).unwrap_or(&0);
        if count > conjugate_count {
            unpaired_edges.push(*edge);
        }
        if count == conjugate_count {
            paired_edges.push(*edge);
            paired_edges.push(conjugate);
        }
    }
}

fn check_oriented_tile_fits_in_with_unpaired_edges_as_seed(oriented_tile: &OrientedTile,
                                                           unpaired_edges: &mut Vec<(i8, i8)>) -> bool{
    // Assuming this oriented tile is in the top-left position, check if it fits with the unpaired edges
    // println!("Checking if the oriented tile fits in with sides: {}, {}, {}, {}", oriented_tile.top(), oriented_tile.right(), oriented_tile.bottom(), oriented_tile.left());
    let mut fits_left = false;
    let mut fits_top = false;
    for edge in unpaired_edges.iter() {
        // println!("Checking edge: {}, {}", edge.0, edge.1);
        if edge.0 != oriented_tile.left() {
            fits_left = true;
            continue; // We don't want to match both the left and top to the same edge
        }
        if edge.1 != oriented_tile.top() { fits_top = true; }
    }
    return fits_top && fits_left;
}



fn get_all_possible_megatiles_sorted_by_score(seeds: &Vec<OrientedTile>,
                                              available_mask: &mut [bool; 256],
                                              mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                                              unpaired_edges: &mut Vec<(i8, i8)>,
                                              paired_edges: &mut Vec<(i8, i8)>,
                                              rim_optimization: &mut Vec<(i8, i8)>,
                                              mega_tile_count: usize,
                                              tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>,
                                              edge_type_usage_score: &[i32; 22]) -> Vec<MegaTile> {
    // Builds all possible mega-tiles that start from the list of seeds
    // We need to score them as we generate them
    let mut mega_tiles_with_score: Vec<(i32, MegaTile)> = Vec::new();

    for top_left_oriented_tile in seeds.iter() {
        if let Some(top_right_tiles) = tile_lookup.get(&(-1, -1, -1, top_left_oriented_tile.right())) {
            for top_right_tile in top_right_tiles.iter() {
                if available_mask[top_right_tile.id() as usize] && top_right_tile.id() != top_left_oriented_tile.id(){
                    if let Some(bottom_right_tiles) = tile_lookup.get(&(top_right_tile.bottom(), -1, -1, -1)) {
                        for bottom_right_tile in bottom_right_tiles.iter() {
                            if available_mask[bottom_right_tile.id() as usize] && bottom_right_tile.id() != top_left_oriented_tile.id() && bottom_right_tile.id() != top_right_tile.id(){
                                if let Some(bottom_left_tiles) = tile_lookup.get(&(top_left_oriented_tile.bottom(), bottom_right_tile.left(), -1, -1)) {
                                    for bottom_left_tile in bottom_left_tiles.iter() {
                                        if available_mask[bottom_left_tile.id() as usize] && bottom_left_tile.id() != top_left_oriented_tile.id() && bottom_left_tile.id() != top_right_tile.id() && bottom_left_tile.id() != bottom_right_tile.id(){
                                            // We have a valid mega-tile
                                            let mega_tile = MegaTile{
                                                tiles: [top_left_oriented_tile.clone(), top_right_tile.clone(), bottom_left_tile.clone(), bottom_right_tile.clone()]
                                            };
                                            let score = score_mega_tile(&mega_tile, mega_tile_edge_count, unpaired_edges, paired_edges, rim_optimization, mega_tile_count, edge_type_usage_score);
                                            mega_tiles_with_score.push((score, mega_tile));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

        }
    }

    // Sort by score -> Highest score at the start, then return
    mega_tiles_with_score.sort_by(|a, b| b.0.cmp(&a.0));
    let mut mega_tiles: Vec<MegaTile> = Vec::new();
    for mega_tile_with_score in mega_tiles_with_score.iter() {
        // only clone if the score is positive
        if mega_tile_with_score.0 >= 0 {
            mega_tiles.push(mega_tile_with_score.1.clone());
        }
    }
    return mega_tiles;
}

fn score_mega_tile(mega_tile: &MegaTile,
                   mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                   unpaired_edges: &mut Vec<(i8, i8)>,
                   paired_edges: &mut Vec<(i8, i8)>,
                   rim_optimization: &mut Vec<(i8, i8)>,
                   mega_tile_count: usize,
                   edge_type_usage_score: &[i32; 22]) -> i32
{
    // We use the scoring mechanism to control which tiles are prioritized based on the tile count
    // For the first 5 tiles, we don't really care so long as they match one of the sides.
    let mut score = 0;

    let top = mega_tile.top();
    let right = mega_tile.right();
    let bottom = mega_tile.bottom();
    let left = mega_tile.left();

    // Little bonus for pieces that are symmetrical
    for side in [top, right, bottom, left].iter() {
        if side.0 == side.1 { score += 100; }
    }

    if mega_tile_count < 3 {
        // We don't care about the score for the first 5 tiles
        // Literally just generate some random number lol
        return rand::thread_rng().gen_range(1..1000);
    }

    // Add score for edges that match with unpaired edges
    for edge in unpaired_edges.iter() {
        let mut has_matched_this_edge = false;
        if edge.0 == top.1 && edge.1 == top.0 {
            score += 1000;
            has_matched_this_edge = true;
        }
        if edge.0 == left.1 && edge.1 == left.0 {
            if !has_matched_this_edge { score += 1000; }
            else { score -= 10000; }
            has_matched_this_edge = true;
        }
        if edge.0 == right.1 && edge.1 == right.0 {
            if !has_matched_this_edge { score += 1000; }
            else { score -= 10000; } // Matching the edge on two sides doesn't help
            has_matched_this_edge = true;
        }
        if edge.0 == bottom.1 && edge.1 == bottom.0 {
            if !has_matched_this_edge { score += 1000; }
            else { score -= 10000; }
            has_matched_this_edge = true;
        }
    }

    // Add score for edges that match with paired edges
    for edge in paired_edges.iter() {
        let mut has_matched_this_edge = false;
        if edge.0 == top.1 && edge.1 == top.0 {
            score += 300;
            has_matched_this_edge = true;
        }
        if edge.0 == left.1 && edge.1 == left.0 {
            if !has_matched_this_edge { score += 300; }
            else { score -= 10000; }
            has_matched_this_edge = true;
        }
        if edge.0 == bottom.1 && edge.1 == bottom.0 {
            if !has_matched_this_edge { score += 300; }
            else { score -= 10000; }
            has_matched_this_edge = true;
        }
        if edge.0 == right.1 && edge.1 == right.0 {
            if !has_matched_this_edge { score += 300; }
            else { score -= 10000; } // Matching the edge on two sides doesn't help
            has_matched_this_edge = true;
        }

    }
    // clone the usage score
    let mut remaining_available_usages = edge_type_usage_score.clone();

    // Want to make small adjustments to the score depending on what edges are used in this mega tile
    for tile in mega_tile.tiles.iter() {
        for edge in [tile.top(), tile.right(), tile.bottom(), tile.left()].iter() {
            score += remaining_available_usages[*edge as usize];
            remaining_available_usages[*edge as usize] -= 1;
            if remaining_available_usages[*edge as usize] < 0 {
                score -= 12000;
            }
        }
    }
    // println!("edge scores {:?}", remaining_available_usages);
    // println!("Score: {}", score);
    score
}

// Seed pieces are all the pieces that can be used to start a mega-tile.
// Return a vector of all possible seeds in order of best to worst
fn get_valid_mega_tile_seeds(available_mask: &mut [bool; 256],
                             mega_tile_edge_count: &mut HashMap<(i8, i8), usize>,
                             unpaired_edges: &mut Vec<(i8, i8)>,
                             paired_edges: &mut Vec<(i8, i8)>,
                             rim_optimization: &mut Vec<(i8, i8)>,
                             mega_tile_count: usize,
                             tile_lookup: &HashMap<(i8, i8, i8, i8), Vec<OrientedTile>>) -> Vec<(OrientedTile)> {
    // For the first 5 mega-tiles, we want to use the starter piece and then the 4 hint pieces
    if IS_USING_HINTS {
        match mega_tile_count {
            0 => return vec![(OrientedTile::new(138, &TILE_SET, 1))],
            // The final 4 pieces should be hint pieces
            45=> return if check_oriented_tile_fits_in_with_unpaired_edges_as_seed(&OrientedTile::new(207, &TILE_SET, 3), unpaired_edges)
            { vec![(OrientedTile::new(207, &TILE_SET, 3))] } else { Vec::new() },
            46 => return if check_oriented_tile_fits_in_with_unpaired_edges_as_seed(&OrientedTile::new(254, &TILE_SET, 2), unpaired_edges)
            { vec![(OrientedTile::new(254, &TILE_SET, 2))] } else { Vec::new() },
            47 => return if check_oriented_tile_fits_in_with_unpaired_edges_as_seed(&OrientedTile::new(180, &TILE_SET, 0), unpaired_edges)
            { vec![(OrientedTile::new(180, &TILE_SET, 0))] } else { Vec::new() },
            48 => return if check_oriented_tile_fits_in_with_unpaired_edges_as_seed(&OrientedTile::new(248, &TILE_SET, 2), unpaired_edges)
            { vec![(OrientedTile::new(248, &TILE_SET, 2))] } else { Vec::new() },
            _ => {}
        }
    } else {
        match mega_tile_count {
            0 => return vec![(OrientedTile::new(138, &TILE_SET, 1))],
            _ => {}
        }
    }

    
    // Otherwise, the seed piece must match with two different unpaired edges on its top and left
    let mut seed_pieces: Vec<(OrientedTile)> = Vec::new();
    let mut last_inner: i8 = 0;
    let mut last_outer: i8 = 0;

    // Create a cross of unpaired_edges.0 and unpaired_edges.1 excluding the actual pairs themselves
    for (i, edge) in unpaired_edges.iter().enumerate() {
        // skip this loop if we have already checked this edge
        if edge.1 == last_outer { continue; }
        last_outer = edge.1;
        for (j, other_edge) in unpaired_edges.iter().enumerate() {
            // skip this loop if we have already checked this edge
            if other_edge.0 == last_inner { continue; }
            last_inner = other_edge.0;
            if i == j { continue; }
            if let Some(oriented_tiles) = tile_lookup.get(&(edge.1, -1, -1, other_edge.0)) {
                for oriented_tile in oriented_tiles.iter() {
                    if available_mask[oriented_tile.id() as usize] {
                        seed_pieces.push(*oriented_tile);
                    }
                }
            }
        }
    }
    seed_pieces
}
