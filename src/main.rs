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
const TILE_SET: [Tile; 256] = [
    Tile { top: 0, right: 1, bottom: -1, left: -1 },
    Tile { top: 0, right: 2, bottom: -1, left: -1 },
    Tile { top: 3, right: 1, bottom: -1, left: -1 },
    Tile { top: 1, right: 3, bottom: -1, left: -1 },
    Tile { top: 0, right: 5, bottom: 0, left: -1 },
    Tile { top: 0, right: 6, bottom: 3, left: -1 },
    Tile { top: 0, right: 7, bottom: 0, left: -1 },
    Tile { top: 0, right: 7, bottom: 4, left: -1 },
    Tile { top: 0, right: 8, bottom: 1, left: -1 },
    Tile { top: 0, right: 9, bottom: 2, left: -1 },
    Tile { top: 0, right: 10, bottom: 3, left: -1 },
    Tile { top: 0, right: 11, bottom: 2, left: -1 },
    Tile { top: 0, right: 11, bottom: 4, left: -1 },
    Tile { top: 0, right: 12, bottom: 2, left: -1 },
    Tile { top: 3, right: 6, bottom: 0, left: -1 },
    Tile { top: 3, right: 13, bottom: 1, left: -1 },
    Tile { top: 3, right: 14, bottom: 4, left: -1 },
    Tile { top: 3, right: 15, bottom: 4, left: -1 },
    Tile { top: 3, right: 9, bottom: 3, left: -1 },
    Tile { top: 3, right: 10, bottom: 3, left: -1 },
    Tile { top: 3, right: 16, bottom: 2, left: -1 },
    Tile { top: 3, right: 17, bottom: 0, left: -1 },
    Tile { top: 3, right: 17, bottom: 4, left: -1 },
    Tile { top: 3, right: 18, bottom: 0, left: -1 },
    Tile { top: 3, right: 12, bottom: 0, left: -1 },
    Tile { top: 1, right: 5, bottom: 3, left: -1 },
    Tile { top: 1, right: 5, bottom: 1, left: -1 },
    Tile { top: 1, right: 6, bottom: 1, left: -1 },
    Tile { top: 1, right: 13, bottom: 1, left: -1 },
    Tile { top: 1, right: 9, bottom: 4, left: -1 },
    Tile { top: 1, right: 10, bottom: 3, left: -1 },
    Tile { top: 1, right: 18, bottom: 1, left: -1 },
    Tile { top: 1, right: 11, bottom: 3, left: -1 },
    Tile { top: 1, right: 11, bottom: 2, left: -1 },
    Tile { top: 1, right: 19, bottom: 4, left: -1 },
    Tile { top: 1, right: 20, bottom: 2, left: -1 },
    Tile { top: 2, right: 13, bottom: 0, left: -1 },
    Tile { top: 2, right: 21, bottom: 4, left: -1 },
    Tile { top: 2, right: 8, bottom: 4, left: -1 },
    Tile { top: 2, right: 15, bottom: 3, left: -1 },
    Tile { top: 2, right: 15, bottom: 1, left: -1 },
    Tile { top: 2, right: 10, bottom: 0, left: -1 },
    Tile { top: 2, right: 10, bottom: 3, left: -1 },
    Tile { top: 2, right: 10, bottom: 1, left: -1 },
    Tile { top: 2, right: 16, bottom: 0, left: -1 },
    Tile { top: 2, right: 18, bottom: 2, left: -1 },
    Tile { top: 2, right: 11, bottom: 2, left: -1 },
    Tile { top: 2, right: 19, bottom: 2, left: -1 },
    Tile { top: 4, right: 5, bottom: 4, left: -1 },
    Tile { top: 4, right: 6, bottom: 0, left: -1 },
    Tile { top: 4, right: 6, bottom: 3, left: -1 },
    Tile { top: 4, right: 7, bottom: 0, left: -1 },
    Tile { top: 4, right: 9, bottom: 2, left: -1 },
    Tile { top: 4, right: 16, bottom: 2, left: -1 },
    Tile { top: 4, right: 16, bottom: 4, left: -1 },
    Tile { top: 4, right: 11, bottom: 1, left: -1 },
    Tile { top: 4, right: 19, bottom: 0, left: -1 },
    Tile { top: 4, right: 19, bottom: 4, left: -1 },
    Tile { top: 4, right: 12, bottom: 3, left: -1 },
    Tile { top: 4, right: 20, bottom: 1, left: -1 },
    Tile { top: 5, right: 5, bottom: 7, left: 13 },
    Tile { top: 5, right: 5, bottom: 14, left: 9 },
    Tile { top: 5, right: 6, bottom: 6, left: 21 },
    Tile { top: 5, right: 13, bottom: 5, left: 11 },
    Tile { top: 5, right: 13, bottom: 13, left: 20 },
    Tile { top: 5, right: 13, bottom: 14, left: 14 },
    Tile { top: 5, right: 13, bottom: 8, left: 6 },
    Tile { top: 5, right: 13, bottom: 18, left: 7 },
    Tile { top: 5, right: 13, bottom: 20, left: 11 },
    Tile { top: 5, right: 21, bottom: 21, left: 9 },
    Tile { top: 5, right: 21, bottom: 9, left: 17 },
    Tile { top: 5, right: 8, bottom: 14, left: 13 },
    Tile { top: 5, right: 8, bottom: 10, left: 16 },
    Tile { top: 5, right: 8, bottom: 18, left: 10 },
    Tile { top: 5, right: 8, bottom: 11, left: 21 },
    Tile { top: 5, right: 15, bottom: 14, left: 10 },
    Tile { top: 5, right: 15, bottom: 15, left: 10 },
    Tile { top: 5, right: 9, bottom: 21, left: 19 },
    Tile { top: 5, right: 9, bottom: 18, left: 21 },
    Tile { top: 5, right: 9, bottom: 19, left: 12 },
    Tile { top: 5, right: 10, bottom: 15, left: 13 },
    Tile { top: 5, right: 16, bottom: 13, left: 13 },
    Tile { top: 5, right: 16, bottom: 8, left: 16 },
    Tile { top: 5, right: 17, bottom: 13, left: 15 },
    Tile { top: 5, right: 17, bottom: 7, left: 14 },
    Tile { top: 5, right: 17, bottom: 11, left: 17 },
    Tile { top: 5, right: 17, bottom: 19, left: 18 },
    Tile { top: 5, right: 18, bottom: 5, left: 12 },
    Tile { top: 5, right: 18, bottom: 7, left: 20 },
    Tile { top: 5, right: 18, bottom: 16, left: 19 },
    Tile { top: 5, right: 11, bottom: 8, left: 17 },
    Tile { top: 5, right: 11, bottom: 15, left: 10 },
    Tile { top: 5, right: 11, bottom: 15, left: 16 },
    Tile { top: 5, right: 11, bottom: 16, left: 12 },
    Tile { top: 5, right: 11, bottom: 17, left: 14 },
    Tile { top: 5, right: 12, bottom: 12, left: 21 },
    Tile { top: 5, right: 20, bottom: 16, left: 15 },
    Tile { top: 5, right: 20, bottom: 18, left: 11 },
    Tile { top: 5, right: 20, bottom: 12, left: 7 },
    Tile { top: 5, right: 20, bottom: 20, left: 12 },
    Tile { top: 6, right: 6, bottom: 17, left: 10 },
    Tile { top: 6, right: 6, bottom: 17, left: 19 },
    Tile { top: 6, right: 6, bottom: 19, left: 15 },
    Tile { top: 6, right: 6, bottom: 20, left: 7 },
    Tile { top: 6, right: 13, bottom: 16, left: 10 },
    Tile { top: 6, right: 7, bottom: 21, left: 11 },
    Tile { top: 6, right: 7, bottom: 15, left: 11 },
    Tile { top: 6, right: 7, bottom: 16, left: 10 },
    Tile { top: 6, right: 7, bottom: 19, left: 8 },
    Tile { top: 6, right: 14, bottom: 10, left: 17 },
    Tile { top: 6, right: 14, bottom: 17, left: 10 },
    Tile { top: 6, right: 21, bottom: 18, left: 15 },
    Tile { top: 6, right: 21, bottom: 18, left: 19 },
    Tile { top: 6, right: 8, bottom: 14, left: 16 },
    Tile { top: 6, right: 8, bottom: 9, left: 17 },
    Tile { top: 6, right: 8, bottom: 17, left: 8 },
    Tile { top: 6, right: 8, bottom: 20, left: 19 },
    Tile { top: 6, right: 15, bottom: 21, left: 12 },
    Tile { top: 6, right: 9, bottom: 19, left: 17 },
    Tile { top: 6, right: 10, bottom: 11, left: 20 },
    Tile { top: 6, right: 16, bottom: 14, left: 20 },
    Tile { top: 6, right: 18, bottom: 7, left: 19 },
    Tile { top: 6, right: 18, bottom: 14, left: 15 },
    Tile { top: 6, right: 18, bottom: 18, left: 10 },
    Tile { top: 6, right: 11, bottom: 17, left: 20 },
    Tile { top: 6, right: 11, bottom: 12, left: 10 },
    Tile { top: 6, right: 19, bottom: 14, left: 7 },
    Tile { top: 6, right: 19, bottom: 15, left: 12 },
    Tile { top: 6, right: 19, bottom: 16, left: 21 },
    Tile { top: 6, right: 19, bottom: 18, left: 11 },
    Tile { top: 6, right: 12, bottom: 7, left: 18 },
    Tile { top: 6, right: 12, bottom: 17, left: 14 },
    Tile { top: 6, right: 20, bottom: 14, left: 19 },
    Tile { top: 6, right: 20, bottom: 8, left: 16 },
    Tile { top: 6, right: 20, bottom: 16, left: 21 },
    Tile { top: 6, right: 20, bottom: 19, left: 18 },
    Tile { top: 13, right: 13, bottom: 18, left: 9 },
    Tile { top: 13, right: 7, bottom: 7, left: 21 },
    Tile { top: 13, right: 7, bottom: 7, left: 8 },
    Tile { top: 13, right: 7, bottom: 7, left: 17 },
    Tile { top: 13, right: 7, bottom: 15, left: 12 },
    Tile { top: 13, right: 7, bottom: 10, left: 7 },
    Tile { top: 13, right: 7, bottom: 19, left: 17 },
    Tile { top: 13, right: 7, bottom: 12, left: 12 },
    Tile { top: 13, right: 14, bottom: 21, left: 16 },
    Tile { top: 13, right: 21, bottom: 13, left: 17 },
    Tile { top: 13, right: 21, bottom: 13, left: 20 },
    Tile { top: 13, right: 21, bottom: 21, left: 14 },
    Tile { top: 13, right: 21, bottom: 10, left: 17 },
    Tile { top: 13, right: 15, bottom: 7, left: 8 },
    Tile { top: 13, right: 15, bottom: 16, left: 20 },
    Tile { top: 13, right: 9, bottom: 8, left: 8 },
    Tile { top: 13, right: 9, bottom: 8, left: 15 },
    Tile { top: 13, right: 9, bottom: 20, left: 19 },
    Tile { top: 13, right: 16, bottom: 9, left: 9 },
    Tile { top: 13, right: 16, bottom: 9, left: 17 },
    Tile { top: 13, right: 16, bottom: 20, left: 9 },
    Tile { top: 13, right: 18, bottom: 9, left: 19 },
    Tile { top: 13, right: 18, bottom: 11, left: 7 },
    Tile { top: 13, right: 11, bottom: 12, left: 12 },
    Tile { top: 13, right: 19, bottom: 7, left: 18 },
    Tile { top: 13, right: 19, bottom: 14, left: 18 },
    Tile { top: 13, right: 20, bottom: 10, left: 8 },
    Tile { top: 13, right: 20, bottom: 16, left: 19 },
    Tile { top: 7, right: 14, bottom: 21, left: 16 },
    Tile { top: 7, right: 14, bottom: 16, left: 11 },
    Tile { top: 7, right: 8, bottom: 21, left: 21 },
    Tile { top: 7, right: 15, bottom: 8, left: 10 },
    Tile { top: 7, right: 15, bottom: 15, left: 12 },
    Tile { top: 7, right: 9, bottom: 16, left: 11 },
    Tile { top: 7, right: 9, bottom: 18, left: 19 },
    Tile { top: 7, right: 9, bottom: 12, left: 8 },
    Tile { top: 7, right: 10, bottom: 10, left: 10 },
    Tile { top: 7, right: 10, bottom: 17, left: 18 },
    Tile { top: 7, right: 16, bottom: 9, left: 12 },
    Tile { top: 7, right: 17, bottom: 9, left: 15 },
    Tile { top: 7, right: 18, bottom: 14, left: 16 },
    Tile { top: 7, right: 11, bottom: 17, left: 19 },
    Tile { top: 7, right: 11, bottom: 11, left: 10 },
    Tile { top: 7, right: 19, bottom: 9, left: 19 },
    Tile { top: 7, right: 12, bottom: 8, left: 19 },
    Tile { top: 7, right: 12, bottom: 9, left: 8 },
    Tile { top: 14, right: 14, bottom: 15, left: 11 },
    Tile { top: 14, right: 21, bottom: 20, left: 9 },
    Tile { top: 14, right: 8, bottom: 15, left: 17 },
    Tile { top: 14, right: 8, bottom: 11, left: 11 },
    Tile { top: 14, right: 15, bottom: 12, left: 9 },
    Tile { top: 14, right: 9, bottom: 14, left: 12 },
    Tile { top: 14, right: 9, bottom: 21, left: 15 },
    Tile { top: 14, right: 9, bottom: 19, left: 15 },
    Tile { top: 14, right: 10, bottom: 21, left: 21 },
    Tile { top: 14, right: 10, bottom: 10, left: 18 },
    Tile { top: 14, right: 16, bottom: 8, left: 9 },
    Tile { top: 14, right: 17, bottom: 12, left: 8 },
    Tile { top: 14, right: 17, bottom: 12, left: 20 },
    Tile { top: 14, right: 17, bottom: 20, left: 10 },
    Tile { top: 14, right: 18, bottom: 8, left: 20 },
    Tile { top: 14, right: 18, bottom: 15, left: 11 },
    Tile { top: 14, right: 18, bottom: 18, left: 18 },
    Tile { top: 14, right: 11, bottom: 15, left: 21 },
    Tile { top: 14, right: 19, bottom: 12, left: 11 },
    Tile { top: 14, right: 19, bottom: 20, left: 9 },
    Tile { top: 14, right: 12, bottom: 17, left: 15 },
    Tile { top: 14, right: 12, bottom: 17, left: 11 },
    Tile { top: 14, right: 12, bottom: 19, left: 21 },
    Tile { top: 14, right: 12, bottom: 20, left: 12 },
    Tile { top: 21, right: 21, bottom: 20, left: 9 },
    Tile { top: 21, right: 8, bottom: 15, left: 20 },
    Tile { top: 21, right: 8, bottom: 11, left: 10 },
    Tile { top: 21, right: 9, bottom: 8, left: 15 },
    Tile { top: 21, right: 9, bottom: 19, left: 10 },
    Tile { top: 21, right: 10, bottom: 21, left: 19 },
    Tile { top: 21, right: 16, bottom: 11, left: 11 },
    Tile { top: 21, right: 17, bottom: 21, left: 18 },
    Tile { top: 21, right: 17, bottom: 16, left: 20 },
    Tile { top: 21, right: 17, bottom: 20, left: 12 },
    Tile { top: 21, right: 18, bottom: 15, left: 10 },
    Tile { top: 21, right: 19, bottom: 16, left: 17 },
    Tile { top: 21, right: 12, bottom: 8, left: 16 },
    Tile { top: 21, right: 20, bottom: 8, left: 19 },
    Tile { top: 21, right: 20, bottom: 12, left: 12 },
    Tile { top: 21, right: 20, bottom: 12, left: 20 },
    Tile { top: 8, right: 8, bottom: 17, left: 20 },
    Tile { top: 8, right: 8, bottom: 18, left: 9 },
    Tile { top: 8, right: 8, bottom: 19, left: 10 },
    Tile { top: 8, right: 9, bottom: 15, left: 10 },
    Tile { top: 8, right: 9, bottom: 17, left: 17 },
    Tile { top: 8, right: 16, bottom: 15, left: 11 },
    Tile { top: 8, right: 18, bottom: 9, left: 20 },
    Tile { top: 8, right: 18, bottom: 19, left: 11 },
    Tile { top: 8, right: 11, bottom: 17, left: 18 },
    Tile { top: 8, right: 11, bottom: 17, left: 12 },
    Tile { top: 15, right: 15, bottom: 15, left: 18 },
    Tile { top: 15, right: 9, bottom: 19, left: 16 },
    Tile { top: 15, right: 16, bottom: 9, left: 16 },
    Tile { top: 15, right: 16, bottom: 9, left: 18 },
    Tile { top: 15, right: 16, bottom: 17, left: 10 },
    Tile { top: 15, right: 17, bottom: 16, left: 19 },
    Tile { top: 15, right: 18, bottom: 10, left: 20 },
    Tile { top: 15, right: 18, bottom: 12, left: 10 },
    Tile { top: 15, right: 11, bottom: 9, left: 12 },
    Tile { top: 15, right: 11, bottom: 16, left: 12 },
    Tile { top: 9, right: 10, bottom: 10, left: 17 },
    Tile { top: 9, right: 10, bottom: 18, left: 11 },
    Tile { top: 9, right: 16, bottom: 20, left: 18 },
    Tile { top: 9, right: 12, bottom: 19, left: 20 },
    Tile { top: 10, right: 10, bottom: 12, left: 20 },
    Tile { top: 10, right: 16, bottom: 17, left: 16 },
    Tile { top: 10, right: 17, bottom: 16, left: 12 },
    Tile { top: 10, right: 18, bottom: 19, left: 12 },
    Tile { top: 16, right: 16, bottom: 20, left: 11 },
    Tile { top: 16, right: 17, bottom: 11, left: 17 },
    Tile { top: 17, right: 11, bottom: 19, left: 18 },
    Tile { top: 18, right: 19, bottom: 12, left: 19 },
    Tile { top: 18, right: 20, bottom: 19, left: 20 },
    Tile { top: 11, right: 20, bottom: 12, left: 20 },
];

#[derive(Copy, Clone)]
struct OrientedTile {
    index: usize,
    orientation: u8, // clockwise: 0 = normal, 1 = 90 degrees , 2 = 180 degrees, 3 = 270 degrees
}


impl OrientedTile {
    // Easily get the top, right, bottom, left sides of a tile given its rotation
    fn tile_as_rotated(&self) -> Tile {
        match self.orientation {
            0 => TILE_SET[self.index],
            1 => Tile {
                top: TILE_SET[self.index].left,
                right: TILE_SET[self.index].top,
                bottom: TILE_SET[self.index].right,
                left: TILE_SET[self.index].bottom,
            },
            2 => Tile {
                top: TILE_SET[self.index].bottom,
                right: TILE_SET[self.index].left,
                bottom: TILE_SET[self.index].top,
                left: TILE_SET[self.index].right,
            },
            3 => Tile {
                top: TILE_SET[self.index].right,
                right: TILE_SET[self.index].bottom,
                bottom: TILE_SET[self.index].left,
                left: TILE_SET[self.index].top,
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
        OrientedTile { index: tileId, orientation}
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
    grid [0][0] = OrientedTile { index: 0, orientation: 1};
    grid [0][15] = OrientedTile { index: 1, orientation: 2};
    grid [15][0] = OrientedTile { index: 2, orientation: 0};
    grid [15][15] = OrientedTile { index: 3, orientation: 3};
}

fn fill_edges(grid: &mut [[OrientedTile; 16]; 16]) {
    for i in 1..15 {  // Fill the top edge. Use tiles 4 to 17
        grid[0][i] = OrientedTile { index: 4 + i - 1, orientation: 1};
    }
    for i in 1..15 {  // Fill the bottom edge. Use tiles 18 to 31
        grid[15][i] = OrientedTile { index: 18 + i - 1, orientation: 3};
    }
    for i in 1..15 { // Fill the left edge. Use tiles 32 to 45
        grid[i][0] = OrientedTile { index: 32 + i - 1, orientation: 0};
    }
    for i in 1..15 { // Fill the right edge. Use tiles 46 to 59
        grid[i][15] = OrientedTile { index: 46 + i - 1, orientation: 2};
    }
}

fn fill_normal(grid: &mut [[OrientedTile; 16]; 16]) {
    for i in 1..15 {
        for j in 1..15 {
            grid[i][j] = OrientedTile { index: 60 + (i - 1) * 14 + j - 1, orientation: 0};
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
    This function will just visualise one megatile for now
     */
    let mut top_line = String::new();
    let mut top_middle_line = String::new();
    let mut bottom_middle_line = String::new();
    let mut bottom_line = String::new();

    let top_left_symbol = colour_symbol(megatile.tile1.top(), true, false);
    let top_right_symbol = colour_symbol(megatile.tile2.top(), true, false);
    let left_top_symbol = colour_symbol(megatile.tile1.left(), true, true);
    let right_top_symbol = colour_symbol(megatile.tile2.right(), true, true);
    let left_bottom_symbol = colour_symbol(megatile.tile3.left(), true, true);
    let right_bottom_symbol = colour_symbol(megatile.tile4.right(), true, true);
    let bottom_left_symbol = colour_symbol(megatile.tile3.bottom(), true, false);
    let bottom_right_symbol = colour_symbol(megatile.tile4.bottom(), true, false);

    top_line.push_str(&format!("┌{}{}┐ ", top_left_symbol, top_right_symbol));
    top_middle_line.push_str(&format!("{}     {} ", left_top_symbol, right_top_symbol));
    bottom_middle_line.push_str(&format!("{}     {} ", left_bottom_symbol, right_bottom_symbol));
    bottom_line.push_str(&format!("└{}{}┘ ", bottom_left_symbol, bottom_right_symbol));

    println!("{}", top_line);
    println!("{}", top_middle_line);
    println!("{}", bottom_middle_line);
    println!("{}", bottom_line);
}

fn main() {
    // Create a grid to store the tiles. They can be oriented in 4 different ways
    let mut grid = [[OrientedTile { index: 0, orientation: 0}; 16]; 16];

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

    // Print the MegaTiles
    for mega_tile in mega_tiles.iter() {
        visualise_megatile(mega_tile);
    }

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

    // Remove duplicates
    for (_, v) in index_unigram.iter_mut() {
        v.dedup();
    }
    for (_, v) in index_bigram.iter_mut() {
        v.dedup();
    }
    (index_unigram, index_bigram)
}


fn find_orientations_for_tile_id_from_mask(tile_id: usize, mask: &Tile) -> Vec<OrientedTile> {
    let mut result = Vec::new();

    for orientation in 0..4 {
        let oriented_tile = OrientedTile::new(tile_id, orientation);

        // If the mask has a -1 in a position, we don't care what the tile has there

        if mask.top != -1 && mask.top != oriented_tile.top() { continue; }
        if mask.right != -1 && mask.right != oriented_tile.right() { continue; }
        if mask.bottom != -1 && mask.bottom != oriented_tile.bottom() { continue; }
        if mask.left != -1 && mask.left != oriented_tile.left() { continue; }

        result.push(oriented_tile);
    }
    result
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

    for i in 0..1 { // will go up to 49 soon
        // Filter the bi-gram
        let odd_mega_tile_edges_that_need_friends = find_filtered_to_odd_count_mega_tile_sides(&bigram_count);
        println!("Found {} odd mega tile edges that need friends", odd_mega_tile_edges_that_need_friends.len());

        // Priority level number 1 - find a tile that matches two mega tile edges
        let oriented_tiles_that_could_match_two_mega_tile_edges = find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(&odd_mega_tile_edges_that_need_friends, index_by_bigram, &available_mask);
        println!("Found {} tiles that could match two mega tile edges", oriented_tiles_that_could_match_two_mega_tile_edges.len());

        for oriented_tile in oriented_tiles_that_could_match_two_mega_tile_edges {
            let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&oriented_tile, &bigram_count, &index_by_edge, &index_by_bigram, &available_mask, &odd_mega_tile_edges_that_need_friends);
            if let Some(mega_tile) = maybe_mega_tile {
                mega_tiles.push(mega_tile);
                break;
            }
        }
        // Exit early if we just added one
        if mega_tiles.len() > i {
            continue;
        }

        // Priority level number 2 - find a tile that matches one mega tile edge
        let oriented_tiles_that_could_match_one_mega_tile_one = find_top_left_oriented_tiles_that_matches_one_mega_tile_edge(&odd_mega_tile_edges_that_need_friends, index_by_edge, &available_mask);
        // println!("Found {} tiles that could match two mega tile edges", oriented_tiles_that_could_match_two_mega_tile_edges.len());

        for oriented_tile in oriented_tiles_that_could_match_one_mega_tile_one {
            let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&oriented_tile, &bigram_count, &index_by_edge, &index_by_bigram, &available_mask, &odd_mega_tile_edges_that_need_friends);
            if let Some(mega_tile) = maybe_mega_tile {
                mega_tiles.push(mega_tile);
                break;
            }
        }

        // Exit early if we just added one
        if mega_tiles.len() > i {
            continue;
        }

        println!("Making a mega tile the hard way :(");

        // Priority level number 3 - just use some other tile :(
        // Start at index 60 to avoid the edge and corner pieces
        for j in 60..256 {
            if available_mask[j] {
                for orientation in 0..4 {
                    let maybe_mega_tile: Option<MegaTile> = try_build_mega_tile(&OrientedTile::new(j, orientation), &bigram_count, &index_by_edge, &index_by_bigram, &available_mask, &odd_mega_tile_edges_that_need_friends);
                    if let Some(mega_tile) = maybe_mega_tile {
                        mega_tiles.push(mega_tile);
                        break;
                    }
                }
                // Exit early if we just added one
                if mega_tiles.len() > i {
                    break;
                }
            }
        }
    }
    mega_tiles
}

fn try_build_mega_tile(top_left_tile: &OrientedTile,
                       bigram_count: &HashMap<(i8, i8), i32>,
                       index_by_edge: &HashMap<i8, Vec<usize>>,
                       index_by_bigram: &HashMap<(i8, i8), Vec<usize>>,
                       available_mask: &[bool; 256],
                       odd_mega_tile_edges_that_need_friends: &Vec<(i8, i8)>) -> Option<MegaTile>
{

    // For debugging see whats in those index by things
    for (k, v) in index_by_bigram.iter() {
        if v.len() > 1 {
            println!("({},{}) {:?}", k.0, k.1, v);
        }
    }

    for (k, v) in index_by_edge.iter() {
        if v.len() > 1 {
            println!("{}: {:?}", k, v);
        }
    }

    // Available mask shouldn't include the top left tile
    let mut available_mask = available_mask.clone();
    available_mask[top_left_tile.index] = false;

    // The mega tile will need a combined top edge that matches something in the odd_mega_tiles_that_need_friends but backwards
    // Filter the edges that need friends down to ones whose second element matches the top edge of the top left tile
    let mega_tile_top_edges = odd_mega_tile_edges_that_need_friends.iter()
        .filter(|(a, b)| *b == top_left_tile.top())
        .collect::<Vec<_>>();

    // oriented tiles that could be the top right tile
    let mut top_right_candidates = Vec::new();

    for top_edge_to_match in mega_tile_top_edges {
        // top_edge_to_match.1 should be the same as top_left_tile.top()
        // top_edge_to_match.0 is the one we want on the top of the next tile
        let bigram_needed_for_top_right_tile: (i8, i8) = (top_left_tile.right(), top_edge_to_match.0);
        for top_right_tile_index in index_by_bigram.get(&bigram_needed_for_top_right_tile).unwrap() {
            if !available_mask[*top_right_tile_index] {
                continue;
            }
            let mask: Tile = Tile { top: top_edge_to_match.0, right: top_left_tile.right(), bottom: -1, left: -1 };
            let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*top_right_tile_index, &mask);
            for top_right_tile in tiles_that_match_mask {
                top_right_candidates.push(top_right_tile);
            }
        }
    }

    println!("Found {} Tier 1 top right candidates", top_right_candidates.len());

    // Fall back and don't worry about the top edge if we haven't found something
    if top_right_candidates.len() == 0 {
        for top_right_tile_index in index_by_edge.get(&top_left_tile.right()).unwrap() {
            if !available_mask[*top_right_tile_index] {
                continue;
            }
            let mask: Tile = Tile { top: -1, right: -1, bottom: -1, left: top_left_tile.right() };
            let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*top_right_tile_index, &mask);
            for top_right_tile in tiles_that_match_mask {
                top_right_candidates.push(top_right_tile);
            }
        }
        println!("Found {} Tier 2 top right candidates", top_right_candidates.len());
    }

    // The mega tile will need a combined left edge that matches something in the odd_mega_tiles_that_need_friends but backwards
    // Filter the edges that need friends down to ones whose first element matches the left edge of the top left tile
    let mega_tile_left_edges = odd_mega_tile_edges_that_need_friends.iter()
        .filter(|(a, b)| *a == top_left_tile.left())
        .collect::<Vec<_>>();

    // oriented tiles that could be the bottom left tile
    let mut bottom_left_candidates = Vec::new();

    for left_edge_to_match in mega_tile_left_edges {
        // left_edge_to_match.0 should be the same as top_left_tile.left()
        // left_edge_to_match.1 is the one we want on the left of the next tile
        let bigram_needed_for_bottom_left_tile: (i8, i8) = (left_edge_to_match.1, top_left_tile.bottom());
        for bottom_left_tile_index in index_by_bigram.get(&bigram_needed_for_bottom_left_tile).unwrap() {
            if !available_mask[*bottom_left_tile_index] {
                continue;
            }
            let mask: Tile = Tile { top: top_left_tile.bottom(), right: -1, bottom: -1, left: left_edge_to_match.1 };
            let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*bottom_left_tile_index, &mask);
            for bottom_left_tile in tiles_that_match_mask {
                bottom_left_candidates.push(bottom_left_tile);
            }
        }
    }

    println!("Found {} Tier 1 bottom left candidates", bottom_left_candidates.len());

    // Fall back and don't worry about the left edge if we haven't found something

    if bottom_left_candidates.len() == 0 {
        println!("No bottom left candidates found");
        for bottom_left_tile_index in index_by_edge.get(&top_left_tile.bottom()).unwrap() {
            println!("Checking {}", bottom_left_tile_index);
            if !available_mask[*bottom_left_tile_index] {
                continue;
            }
            let mask: Tile = Tile { top: top_left_tile.bottom(), right: -1, bottom: -1, left: -1 };
            let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*bottom_left_tile_index, &mask);
            for bottom_left_tile in tiles_that_match_mask {
                bottom_left_candidates.push(bottom_left_tile);
            }
        }
        println!("Found {} Tier 2 bottom left candidates", bottom_left_candidates.len());
    }

    // Now we have a list of top right and bottom left tiles that could work
    // We need to find a pair that works
    // Tier 1 - We find a fourth tile that matches with both a top right and bottom left tile
    for top_right_tile in top_right_candidates {
        for bottom_left_tile in &bottom_left_candidates {
            if top_right_tile.left() != bottom_left_tile.right() {
                continue;
            }
            // Find a tile to go in the bottom right
            let bi_gram_needed_for_bottom_right_tile: (i8, i8) = (bottom_left_tile.right(), top_right_tile.bottom());
            for bottom_right_tile_index in index_by_bigram.get(&bi_gram_needed_for_bottom_right_tile).unwrap() {
                if !available_mask[*bottom_right_tile_index] {
                    continue;
                }
                let mask: Tile = Tile { top: top_right_tile.bottom(), right: -1, bottom: -1, left: bottom_left_tile.right() };
                let tiles_that_match_mask = find_orientations_for_tile_id_from_mask(*bottom_right_tile_index, &mask);
                for bottom_right_tile in tiles_that_match_mask {
                    return Some(MegaTile { tile1: *top_left_tile, tile2: top_right_tile, tile3: *bottom_left_tile, tile4: bottom_right_tile });
                }
            }
        }
    }
    Some(MegaTile { tile1: OrientedTile::new(0, 0), tile2: OrientedTile::new(0, 0), tile3: OrientedTile::new(0, 0), tile4: OrientedTile::new(0, 0) })
}


fn find_top_left_oriented_tiles_that_matches_two_mega_tile_edges(odd_mega_tile_edges_that_need_friends: &Vec<(i8, i8)>,
               index_by_bigram: &HashMap<(i8, i8), Vec<usize>>,
               available_mask: &[bool; 256]) -> Vec<OrientedTile>
{
    // The goal here is to find a tile that can go in the top left of a mega tile
    // Since this tile is in the top left, the top side will have to match the first part of an edge
    // and the left side will have to match the second part of an edge
    // We want to find a tile that matches some combination of the edges that are currently odd

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

fn find_top_left_oriented_tiles_that_matches_one_mega_tile_edge(odd_mega_tile_edges_that_need_friends: &Vec<(i8, i8)>,
                                                         index_by_edge: &HashMap<i8, Vec<usize>>,
                                                         available_mask: &[bool; 256]) -> Vec<OrientedTile>
{
    // The goal here is to find a tile that can go in the top left of a mega tile
    // Since this tile is in the top left, it's left side will match the first part of an edge
    // and it's top side will match the second part of a bigram

    // Check firest for matches to the first part of the edge
    let mut tiles_that_could_match_one_mega_tile_edge = Vec::new();
    for edge in odd_mega_tile_edges_that_need_friends {
        if let Some(matching_tile_ids) = index_by_edge.get(&edge.0) {
            for tile_id in matching_tile_ids {
                if available_mask[*tile_id] {
                    // Try out all of the four possible orientations
                    for orientation in 0..4 {
                        let oriented_tile = OrientedTile::new(*tile_id, orientation);
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