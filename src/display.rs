use crate::data::*;

pub fn colour_symbol(num: i8, matches: bool, vertical: bool, is_megatile: bool) -> String {
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

pub fn print_grid(grid: &Vec<Vec<Option<OrientedTile>>>) {
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


pub fn visualise_mega_tile(megatile: &MegaTile) {
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

    let top_left_symbol = colour_symbol(megatile.tiles[0].top(), true, false, true);
    let top_right_symbol = colour_symbol(megatile.tiles[1].top(), true, false, true);
    let left_top_symbol = colour_symbol(megatile.tiles[0].left(), true, true, true);
    let right_top_symbol = colour_symbol(megatile.tiles[1].right(), true, true, true);
    let left_bottom_symbol = colour_symbol(megatile.tiles[2].left(), true, true, true);
    let right_bottom_symbol = colour_symbol(megatile.tiles[3].right(), true, true, true);
    let bottom_left_symbol = colour_symbol(megatile.tiles[2].bottom(), true, false, true);
    let bottom_right_symbol = colour_symbol(megatile.tiles[3].bottom(), true, false, true);

    // Check for internal matches
    let top_middle_matches = megatile.tiles[0].right() == megatile.tiles[1].left();
    let bottom_middle_matches = megatile.tiles[2].right() == megatile.tiles[3].left();
    let left_middle_matches = megatile.tiles[0].bottom() == megatile.tiles[2].top();
    let right_middle_matches = megatile.tiles[1].bottom() == megatile.tiles[3].top();

    // use info line to describe the differences between the tiles
    if !top_middle_matches {
        info_line.push_str(&format!("Top middle doesn't match: {} != {}", colour_symbol(megatile.tiles[0].right(), true, true, true), colour_symbol(megatile.tiles[1].left(), true, true, true)));
    }
    if !right_middle_matches {
        info_line.push_str(&format!("Right middle doesn't match: {} != {}", colour_symbol(megatile.tiles[1].bottom(), true, false, true), colour_symbol(megatile.tiles[3].top(), true, false, true)));
    }
    if !bottom_middle_matches {
        info_line.push_str(&format!("Bottom middle doesn't match: {} != {}", colour_symbol(megatile.tiles[2].right(), true, true, true), colour_symbol(megatile.tiles[3].left(), true, true, true)));
    }
    if !left_middle_matches {
        info_line.push_str(&format!("Left middle doesn't match: {} != {}", colour_symbol(megatile.tiles[0].bottom(), true, false, true), colour_symbol(megatile.tiles[2].top(), true, false, true)));
    }

    let top_middle_symbol = colour_symbol(megatile.tiles[0].right(), top_middle_matches, true, true);
    let bottom_middle_symbol = colour_symbol(megatile.tiles[2].right(), bottom_middle_matches, true, true);
    let left_middle_symbol = colour_symbol(megatile.tiles[0].bottom(), left_middle_matches, false, true);
    let right_middle_symbol = colour_symbol(megatile.tiles[1].bottom(), right_middle_matches, false, true);

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