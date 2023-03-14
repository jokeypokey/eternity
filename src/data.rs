
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub struct Tile {
    pub top: i8,
    pub right: i8,
    pub bottom: i8,
    pub left: i8,
    pub id: usize,
}


#[derive(Copy, Clone)]
pub struct OrientedTile {
    pub tile: Tile,
    pub orientation: u8, // clockwise: 0 = normal, 1 = 90 degrees , 2 = 180 degrees, 3 = 270 degrees
}


impl OrientedTile {
    // Easily get the top, right, bottom, left sides of a tile given its rotation
    pub fn tile_as_rotated(&self) -> Tile {
        match self.orientation {
            0 => self.tile,
            1 => Tile {
                top: self.tile.left,
                right: self.tile.top,
                bottom: self.tile.right,
                left: self.tile.bottom,
                id: self.tile.id,
            },
            2 => Tile {
                top: self.tile.bottom,
                right: self.tile.left,
                bottom: self.tile.top,
                left: self.tile.right,
                id: self.tile.id,
            },
            3 => Tile {
                top: self.tile.right,
                right: self.tile.bottom,
                bottom: self.tile.left,
                left: self.tile.top,
                id: self.tile.id,
            },
            _ => panic!("Invalid orientation"),
        }
    }

    pub fn top(&self) -> i8 {
        self.tile_as_rotated().top
    }
    pub fn right(&self) -> i8 {
        self.tile_as_rotated().right
    }
    pub fn bottom(&self) -> i8 {
        self.tile_as_rotated().bottom
    }
    pub fn left(&self) -> i8 {
        self.tile_as_rotated().left
    }
    pub fn id(&self) -> usize {
        self.tile.id
    }

    pub fn new(tileId: usize, tileset: &[Tile], orientation: u8) -> OrientedTile {
        OrientedTile {
            tile: tileset[tileId],
            orientation,
        }
    }
}

#[derive(Copy, Clone)]
pub struct MegaTile {
    // A mega tile is a 2x2 grid of tiles
    // ┌1 2┐
    // └3 4┘
    pub tile1: OrientedTile,
    pub tile2: OrientedTile,
    pub tile3: OrientedTile,
    pub tile4: OrientedTile,
}

// Old non-generic version
// pub fn build_indices() -> (HashMap<i8, Vec<usize>>, HashMap<(i8, i8), Vec<usize>>) {
//     let mut index_unigram = HashMap::new();
//     let mut index_bigram = HashMap::new();
//
//     for (i, tile) in TILE_SET.iter().enumerate() {
//         // skip over the first 60
//         if i < 60 {
//             continue;
//         }
//
//         index_unigram.entry(tile.top).or_insert(Vec::new()).push(i);
//         index_unigram.entry(tile.right).or_insert(Vec::new()).push(i);
//         index_unigram.entry(tile.bottom).or_insert(Vec::new()).push(i);
//         index_unigram.entry(tile.left).or_insert(Vec::new()).push(i);
//
//         index_bigram.entry((tile.top, tile.right)).or_insert(Vec::new()).push(i);
//         index_bigram.entry((tile.right, tile.bottom)).or_insert(Vec::new()).push(i);
//         index_bigram.entry((tile.bottom, tile.left)).or_insert(Vec::new()).push(i);
//         index_bigram.entry((tile.left, tile.top)).or_insert(Vec::new()).push(i);
//     }
//
//     // Remove duplicates
//     for (_, v) in index_unigram.iter_mut() {
//         v.dedup();
//     }
//     for (_, v) in index_bigram.iter_mut() {
//         v.dedup();
//     }
//     (index_unigram, index_bigram)
// }

pub fn build_indices(tile_set: &[Tile], skip_n: usize) -> HashMap<(i8, i8, i8, i8), Vec<OrientedTile>> {
    let mut index = HashMap::new();

    for (i, tile) in tile_set.iter().enumerate() {
        // skip over the first 60
        if i < skip_n {
            continue;
        }

        for orientation in 0..4 {
            let oriented_tile = OrientedTile::new(i, tile_set, orientation);
            // The cache treats -1 as a wildcard
            for top in vec![oriented_tile.top(), -1] {
                for right in vec![oriented_tile.right(), -1] {
                    for bottom in vec![oriented_tile.bottom(), -1] {
                        for left in vec![oriented_tile.left(), -1] {
                            index.entry((top, right, bottom, left)).or_insert(Vec::new()).push(oriented_tile);
                        }
                    }
                }
            }
        }
    }
    index
}

// make a global array of tiles
pub const MINI_TILE_SET: [Tile; 49] = [
    Tile {top: 12, right: 4, bottom: 5, left: 19, id: 0},
    Tile {top: 10, right: 13, bottom: 6, left: 4, id: 1},
    Tile {top: 12, right: 7, bottom: 8, left: 13, id: 2},
    Tile {top: 7, right: 1, bottom: 8, left: 7, id: 3},
    Tile {top: 5, right: 4, bottom: 13, left: 1, id: 4},
    Tile {top: 18, right: 12, bottom: 13, left: 4, id: 5},
    Tile {top: 19, right: 10, bottom: 1, left: 12, id: 6},
    Tile {top: 5, right: 18, bottom: 17, left: 1, id: 7},
    Tile {top: 6, right: 12, bottom: 19, left: 18, id: 8},
    Tile {top: 8, right: 19, bottom: 18, left: 12, id: 9},
    Tile {top: 8, right: 4, bottom: 5, left: 19, id: 10},
    Tile {top: 13, right: 8, bottom: 2, left: 4, id: 11},
    Tile {top: 13, right: 16, bottom: 0, left: 8, id: 12},
    Tile {top: 1, right: 8, bottom: 5, left: 16, id: 13},
    Tile {top: 17, right: 1, bottom: 10, left: 17, id: 14},
    Tile {top: 19, right: 9, bottom: 1, left: 1, id: 15},
    Tile {top: 18, right: 4, bottom: 0, left: 9, id: 16},
    Tile {top: 5, right: 6, bottom: 13, left: 4, id: 17},
    Tile {top: 2, right: 0, bottom: 10, left: 6, id: 18},
    Tile {top: 0, right: 10, bottom: 16, left: 0, id: 19},
    Tile {top: 5, right: 0, bottom: 11, left: 10, id: 20},
    Tile {top: 10, right: 7, bottom: 18, left: 12, id: 21},
    Tile {top: 1, right: 15, bottom: 12, left: 7, id: 22},
    Tile {top: 0, right: 17, bottom: 12, left: 15, id: 23},
    Tile {top: 13, right: 16, bottom: 6, left: 17, id: 24},
    Tile {top: 10, right: 11, bottom: 16, left: 16, id: 25},
    Tile {top: 16, right: 4, bottom: 13, left: 11, id: 26},
    Tile {top: 11, right: 4, bottom: 11, left: 4, id: 27},
    Tile {top: 18, right: 0, bottom: 15, left: 2, id: 28},
    Tile {top: 12, right: 16, bottom: 9, left: 0, id: 29},
    Tile {top: 12, right: 13, bottom: 15, left: 16, id: 30},
    Tile {top: 6, right: 10, bottom: 17, left: 13, id: 31},
    Tile {top: 16, right: 11, bottom: 4, left: 10, id: 32},
    Tile {top: 13, right: 0, bottom: 10, left: 11, id: 33},
    Tile {top: 11, right: 13, bottom: 14, left: 0, id: 34},
    Tile {top: 15, right: 13, bottom: 15, left: 18, id: 35},
    Tile {top: 9, right: 4, bottom: 14, left: 13, id: 36},
    Tile {top: 15, right: 14, bottom: 14, left: 4, id: 37},
    Tile {top: 17, right: 19, bottom: 17, left: 14, id: 38},
    Tile {top: 4, right: 17, bottom: 17, left: 19, id: 39},
    Tile {top: 10, right: 16, bottom: 2, left: 17, id: 40},
    Tile {top: 14, right: 7, bottom: 1, left: 16, id: 41},
    Tile {top: 15, right: 0, bottom: 16, left: 12, id: 42},
    Tile {top: 14, right: 1, bottom: 7, left: 0, id: 43},
    Tile {top: 14, right: 16, bottom: 3, left: 1, id: 44},
    Tile {top: 17, right: 14, bottom: 16, left: 16, id: 45},
    Tile {top: 17, right: 16, bottom: 3, left: 14, id: 46},
    Tile {top: 2, right: 4, bottom: 11, left: 16, id: 47},
    Tile {top: 1, right: 13, bottom: 16, left: 4, id: 48},
];


// make a global array of tiles
pub const TILE_SET: [Tile; 256] = [
    Tile {top: 0, right: 1, bottom: -1, left: -1, id: 0},
    Tile {top: 0, right: 2, bottom: -1, left: -1, id: 1},
    Tile {top: 3, right: 1, bottom: -1, left: -1, id: 2},
    Tile {top: 1, right: 3, bottom: -1, left: -1, id: 3},
    Tile {top: 0, right: 5, bottom: 0, left: -1, id: 4},
    Tile {top: 0, right: 6, bottom: 3, left: -1, id: 5},
    Tile {top: 0, right: 7, bottom: 0, left: -1, id: 6},
    Tile {top: 0, right: 7, bottom: 4, left: -1, id: 7},
    Tile {top: 0, right: 8, bottom: 1, left: -1, id: 8},
    Tile {top: 0, right: 9, bottom: 2, left: -1, id: 9},
    Tile {top: 0, right: 10, bottom: 3, left: -1, id: 10},
    Tile {top: 0, right: 11, bottom: 2, left: -1, id: 11},
    Tile {top: 0, right: 11, bottom: 4, left: -1, id: 12},
    Tile {top: 0, right: 12, bottom: 2, left: -1, id: 13},
    Tile {top: 3, right: 6, bottom: 0, left: -1, id: 14},
    Tile {top: 3, right: 13, bottom: 1, left: -1, id: 15},
    Tile {top: 3, right: 14, bottom: 4, left: -1, id: 16},
    Tile {top: 3, right: 15, bottom: 4, left: -1, id: 17},
    Tile {top: 3, right: 9, bottom: 3, left: -1, id: 18},
    Tile {top: 3, right: 10, bottom: 3, left: -1, id: 19},
    Tile {top: 3, right: 16, bottom: 2, left: -1, id: 20},
    Tile {top: 3, right: 17, bottom: 0, left: -1, id: 21},
    Tile {top: 3, right: 17, bottom: 4, left: -1, id: 22},
    Tile {top: 3, right: 18, bottom: 0, left: -1, id: 23},
    Tile {top: 3, right: 12, bottom: 0, left: -1, id: 24},
    Tile {top: 1, right: 5, bottom: 3, left: -1, id: 25},
    Tile {top: 1, right: 5, bottom: 1, left: -1, id: 26},
    Tile {top: 1, right: 6, bottom: 1, left: -1, id: 27},
    Tile {top: 1, right: 13, bottom: 1, left: -1, id: 28},
    Tile {top: 1, right: 9, bottom: 4, left: -1, id: 29},
    Tile {top: 1, right: 10, bottom: 3, left: -1, id: 30},
    Tile {top: 1, right: 18, bottom: 1, left: -1, id: 31},
    Tile {top: 1, right: 11, bottom: 3, left: -1, id: 32},
    Tile {top: 1, right: 11, bottom: 2, left: -1, id: 33},
    Tile {top: 1, right: 19, bottom: 4, left: -1, id: 34},
    Tile {top: 1, right: 20, bottom: 2, left: -1, id: 35},
    Tile {top: 2, right: 13, bottom: 0, left: -1, id: 36},
    Tile {top: 2, right: 21, bottom: 4, left: -1, id: 37},
    Tile {top: 2, right: 8, bottom: 4, left: -1, id: 38},
    Tile {top: 2, right: 15, bottom: 3, left: -1, id: 39},
    Tile {top: 2, right: 15, bottom: 1, left: -1, id: 40},
    Tile {top: 2, right: 10, bottom: 0, left: -1, id: 41},
    Tile {top: 2, right: 10, bottom: 3, left: -1, id: 42},
    Tile {top: 2, right: 10, bottom: 1, left: -1, id: 43},
    Tile {top: 2, right: 16, bottom: 0, left: -1, id: 44},
    Tile {top: 2, right: 18, bottom: 2, left: -1, id: 45},
    Tile {top: 2, right: 11, bottom: 2, left: -1, id: 46},
    Tile {top: 2, right: 19, bottom: 2, left: -1, id: 47},
    Tile {top: 4, right: 5, bottom: 4, left: -1, id: 48},
    Tile {top: 4, right: 6, bottom: 0, left: -1, id: 49},
    Tile {top: 4, right: 6, bottom: 3, left: -1, id: 50},
    Tile {top: 4, right: 7, bottom: 0, left: -1, id: 51},
    Tile {top: 4, right: 9, bottom: 2, left: -1, id: 52},
    Tile {top: 4, right: 16, bottom: 2, left: -1, id: 53},
    Tile {top: 4, right: 16, bottom: 4, left: -1, id: 54},
    Tile {top: 4, right: 11, bottom: 1, left: -1, id: 55},
    Tile {top: 4, right: 19, bottom: 0, left: -1, id: 56},
    Tile {top: 4, right: 19, bottom: 4, left: -1, id: 57},
    Tile {top: 4, right: 12, bottom: 3, left: -1, id: 58},
    Tile {top: 4, right: 20, bottom: 1, left: -1, id: 59},
    Tile {top: 5, right: 5, bottom: 7, left: 13, id: 60},
    Tile {top: 5, right: 5, bottom: 14, left: 9, id: 61},
    Tile {top: 5, right: 6, bottom: 6, left: 21, id: 62},
    Tile {top: 5, right: 13, bottom: 5, left: 11, id: 63},
    Tile {top: 5, right: 13, bottom: 13, left: 20, id: 64},
    Tile {top: 5, right: 13, bottom: 14, left: 14, id: 65},
    Tile {top: 5, right: 13, bottom: 8, left: 6, id: 66},
    Tile {top: 5, right: 13, bottom: 18, left: 7, id: 67},
    Tile {top: 5, right: 13, bottom: 20, left: 11, id: 68},
    Tile {top: 5, right: 21, bottom: 21, left: 9, id: 69},
    Tile {top: 5, right: 21, bottom: 9, left: 17, id: 70},
    Tile {top: 5, right: 8, bottom: 14, left: 13, id: 71},
    Tile {top: 5, right: 8, bottom: 10, left: 16, id: 72},
    Tile {top: 5, right: 8, bottom: 18, left: 10, id: 73},
    Tile {top: 5, right: 8, bottom: 11, left: 21, id: 74},
    Tile {top: 5, right: 15, bottom: 14, left: 10, id: 75},
    Tile {top: 5, right: 15, bottom: 15, left: 10, id: 76},
    Tile {top: 5, right: 9, bottom: 21, left: 19, id: 77},
    Tile {top: 5, right: 9, bottom: 18, left: 21, id: 78},
    Tile {top: 5, right: 9, bottom: 19, left: 12, id: 79},
    Tile {top: 5, right: 10, bottom: 15, left: 13, id: 80},
    Tile {top: 5, right: 16, bottom: 13, left: 13, id: 81},
    Tile {top: 5, right: 16, bottom: 8, left: 16, id: 82},
    Tile {top: 5, right: 17, bottom: 13, left: 15, id: 83},
    Tile {top: 5, right: 17, bottom: 7, left: 14, id: 84},
    Tile {top: 5, right: 17, bottom: 11, left: 17, id: 85},
    Tile {top: 5, right: 17, bottom: 19, left: 18, id: 86},
    Tile {top: 5, right: 18, bottom: 5, left: 12, id: 87},
    Tile {top: 5, right: 18, bottom: 7, left: 20, id: 88},
    Tile {top: 5, right: 18, bottom: 16, left: 19, id: 89},
    Tile {top: 5, right: 11, bottom: 8, left: 17, id: 90},
    Tile {top: 5, right: 11, bottom: 15, left: 10, id: 91},
    Tile {top: 5, right: 11, bottom: 15, left: 16, id: 92},
    Tile {top: 5, right: 11, bottom: 16, left: 12, id: 93},
    Tile {top: 5, right: 11, bottom: 17, left: 14, id: 94},
    Tile {top: 5, right: 12, bottom: 12, left: 21, id: 95},
    Tile {top: 5, right: 20, bottom: 16, left: 15, id: 96},
    Tile {top: 5, right: 20, bottom: 18, left: 11, id: 97},
    Tile {top: 5, right: 20, bottom: 12, left: 7, id: 98},
    Tile {top: 5, right: 20, bottom: 20, left: 12, id: 99},
    Tile {top: 6, right: 6, bottom: 17, left: 10, id: 100},
    Tile {top: 6, right: 6, bottom: 17, left: 19, id: 101},
    Tile {top: 6, right: 6, bottom: 19, left: 15, id: 102},
    Tile {top: 6, right: 6, bottom: 20, left: 7, id: 103},
    Tile {top: 6, right: 13, bottom: 16, left: 10, id: 104},
    Tile {top: 6, right: 7, bottom: 21, left: 11, id: 105},
    Tile {top: 6, right: 7, bottom: 15, left: 11, id: 106},
    Tile {top: 6, right: 7, bottom: 16, left: 10, id: 107},
    Tile {top: 6, right: 7, bottom: 19, left: 8, id: 108},
    Tile {top: 6, right: 14, bottom: 10, left: 17, id: 109},
    Tile {top: 6, right: 14, bottom: 17, left: 10, id: 110},
    Tile {top: 6, right: 21, bottom: 18, left: 15, id: 111},
    Tile {top: 6, right: 21, bottom: 18, left: 19, id: 112},
    Tile {top: 6, right: 8, bottom: 14, left: 16, id: 113},
    Tile {top: 6, right: 8, bottom: 9, left: 17, id: 114},
    Tile {top: 6, right: 8, bottom: 17, left: 8, id: 115},
    Tile {top: 6, right: 8, bottom: 20, left: 19, id: 116},
    Tile {top: 6, right: 15, bottom: 21, left: 12, id: 117},
    Tile {top: 6, right: 9, bottom: 19, left: 17, id: 118},
    Tile {top: 6, right: 10, bottom: 11, left: 20, id: 119},
    Tile {top: 6, right: 16, bottom: 14, left: 20, id: 120},
    Tile {top: 6, right: 18, bottom: 7, left: 19, id: 121},
    Tile {top: 6, right: 18, bottom: 14, left: 15, id: 122},
    Tile {top: 6, right: 18, bottom: 18, left: 10, id: 123},
    Tile {top: 6, right: 11, bottom: 17, left: 20, id: 124},
    Tile {top: 6, right: 11, bottom: 12, left: 10, id: 125},
    Tile {top: 6, right: 19, bottom: 14, left: 7, id: 126},
    Tile {top: 6, right: 19, bottom: 15, left: 12, id: 127},
    Tile {top: 6, right: 19, bottom: 16, left: 21, id: 128},
    Tile {top: 6, right: 19, bottom: 18, left: 11, id: 129},
    Tile {top: 6, right: 12, bottom: 7, left: 18, id: 130},
    Tile {top: 6, right: 12, bottom: 17, left: 14, id: 131},
    Tile {top: 6, right: 20, bottom: 14, left: 19, id: 132},
    Tile {top: 6, right: 20, bottom: 8, left: 16, id: 133},
    Tile {top: 6, right: 20, bottom: 16, left: 21, id: 134},
    Tile {top: 6, right: 20, bottom: 19, left: 18, id: 135},
    Tile {top: 13, right: 13, bottom: 18, left: 9, id: 136},
    Tile {top: 13, right: 7, bottom: 7, left: 21, id: 137},
    Tile {top: 13, right: 7, bottom: 7, left: 8, id: 138},
    Tile {top: 13, right: 7, bottom: 7, left: 17, id: 139},
    Tile {top: 13, right: 7, bottom: 15, left: 12, id: 140},
    Tile {top: 13, right: 7, bottom: 10, left: 7, id: 141},
    Tile {top: 13, right: 7, bottom: 19, left: 17, id: 142},
    Tile {top: 13, right: 7, bottom: 12, left: 12, id: 143},
    Tile {top: 13, right: 14, bottom: 21, left: 16, id: 144},
    Tile {top: 13, right: 21, bottom: 13, left: 17, id: 145},
    Tile {top: 13, right: 21, bottom: 13, left: 20, id: 146},
    Tile {top: 13, right: 21, bottom: 21, left: 14, id: 147},
    Tile {top: 13, right: 21, bottom: 10, left: 17, id: 148},
    Tile {top: 13, right: 15, bottom: 7, left: 8, id: 149},
    Tile {top: 13, right: 15, bottom: 16, left: 20, id: 150},
    Tile {top: 13, right: 9, bottom: 8, left: 8, id: 151},
    Tile {top: 13, right: 9, bottom: 8, left: 15, id: 152},
    Tile {top: 13, right: 9, bottom: 20, left: 19, id: 153},
    Tile {top: 13, right: 16, bottom: 9, left: 9, id: 154},
    Tile {top: 13, right: 16, bottom: 9, left: 17, id: 155},
    Tile {top: 13, right: 16, bottom: 20, left: 9, id: 156},
    Tile {top: 13, right: 18, bottom: 9, left: 19, id: 157},
    Tile {top: 13, right: 18, bottom: 11, left: 7, id: 158},
    Tile {top: 13, right: 11, bottom: 12, left: 12, id: 159},
    Tile {top: 13, right: 19, bottom: 7, left: 18, id: 160},
    Tile {top: 13, right: 19, bottom: 14, left: 18, id: 161},
    Tile {top: 13, right: 20, bottom: 10, left: 8, id: 162},
    Tile {top: 13, right: 20, bottom: 16, left: 19, id: 163},
    Tile {top: 7, right: 14, bottom: 21, left: 16, id: 164},
    Tile {top: 7, right: 14, bottom: 16, left: 11, id: 165},
    Tile {top: 7, right: 8, bottom: 21, left: 21, id: 166},
    Tile {top: 7, right: 15, bottom: 8, left: 10, id: 167},
    Tile {top: 7, right: 15, bottom: 15, left: 12, id: 168},
    Tile {top: 7, right: 9, bottom: 16, left: 11, id: 169},
    Tile {top: 7, right: 9, bottom: 18, left: 19, id: 170},
    Tile {top: 7, right: 9, bottom: 12, left: 8, id: 171},
    Tile {top: 7, right: 10, bottom: 10, left: 10, id: 172},
    Tile {top: 7, right: 10, bottom: 17, left: 18, id: 173},
    Tile {top: 7, right: 16, bottom: 9, left: 12, id: 174},
    Tile {top: 7, right: 17, bottom: 9, left: 15, id: 175},
    Tile {top: 7, right: 18, bottom: 14, left: 16, id: 176},
    Tile {top: 7, right: 11, bottom: 17, left: 19, id: 177},
    Tile {top: 7, right: 11, bottom: 11, left: 10, id: 178},
    Tile {top: 7, right: 19, bottom: 9, left: 19, id: 179},
    Tile {top: 7, right: 12, bottom: 8, left: 19, id: 180},
    Tile {top: 7, right: 12, bottom: 9, left: 8, id: 181},
    Tile {top: 14, right: 14, bottom: 15, left: 11, id: 182},
    Tile {top: 14, right: 21, bottom: 20, left: 9, id: 183},
    Tile {top: 14, right: 8, bottom: 15, left: 17, id: 184},
    Tile {top: 14, right: 8, bottom: 11, left: 11, id: 185},
    Tile {top: 14, right: 15, bottom: 12, left: 9, id: 186},
    Tile {top: 14, right: 9, bottom: 14, left: 12, id: 187},
    Tile {top: 14, right: 9, bottom: 21, left: 15, id: 188},
    Tile {top: 14, right: 9, bottom: 19, left: 15, id: 189},
    Tile {top: 14, right: 10, bottom: 21, left: 21, id: 190},
    Tile {top: 14, right: 10, bottom: 10, left: 18, id: 191},
    Tile {top: 14, right: 16, bottom: 8, left: 9, id: 192},
    Tile {top: 14, right: 17, bottom: 12, left: 8, id: 193},
    Tile {top: 14, right: 17, bottom: 12, left: 20, id: 194},
    Tile {top: 14, right: 17, bottom: 20, left: 10, id: 195},
    Tile {top: 14, right: 18, bottom: 8, left: 20, id: 196},
    Tile {top: 14, right: 18, bottom: 15, left: 11, id: 197},
    Tile {top: 14, right: 18, bottom: 18, left: 18, id: 198},
    Tile {top: 14, right: 11, bottom: 15, left: 21, id: 199},
    Tile {top: 14, right: 19, bottom: 12, left: 11, id: 200},
    Tile {top: 14, right: 19, bottom: 20, left: 9, id: 201},
    Tile {top: 14, right: 12, bottom: 17, left: 15, id: 202},
    Tile {top: 14, right: 12, bottom: 17, left: 11, id: 203},
    Tile {top: 14, right: 12, bottom: 19, left: 21, id: 204},
    Tile {top: 14, right: 12, bottom: 20, left: 12, id: 205},
    Tile {top: 21, right: 21, bottom: 20, left: 9, id: 206},
    Tile {top: 21, right: 8, bottom: 15, left: 20, id: 207},
    Tile {top: 21, right: 8, bottom: 11, left: 10, id: 208},
    Tile {top: 21, right: 9, bottom: 8, left: 15, id: 209},
    Tile {top: 21, right: 9, bottom: 19, left: 10, id: 210},
    Tile {top: 21, right: 10, bottom: 21, left: 19, id: 211},
    Tile {top: 21, right: 16, bottom: 11, left: 11, id: 212},
    Tile {top: 21, right: 17, bottom: 21, left: 18, id: 213},
    Tile {top: 21, right: 17, bottom: 16, left: 20, id: 214},
    Tile {top: 21, right: 17, bottom: 20, left: 12, id: 215},
    Tile {top: 21, right: 18, bottom: 15, left: 10, id: 216},
    Tile {top: 21, right: 19, bottom: 16, left: 17, id: 217},
    Tile {top: 21, right: 12, bottom: 8, left: 16, id: 218},
    Tile {top: 21, right: 20, bottom: 8, left: 19, id: 219},
    Tile {top: 21, right: 20, bottom: 12, left: 12, id: 220},
    Tile {top: 21, right: 20, bottom: 12, left: 20, id: 221},
    Tile {top: 8, right: 8, bottom: 17, left: 20, id: 222},
    Tile {top: 8, right: 8, bottom: 18, left: 9, id: 223},
    Tile {top: 8, right: 8, bottom: 19, left: 10, id: 224},
    Tile {top: 8, right: 9, bottom: 15, left: 10, id: 225},
    Tile {top: 8, right: 9, bottom: 17, left: 17, id: 226},
    Tile {top: 8, right: 16, bottom: 15, left: 11, id: 227},
    Tile {top: 8, right: 18, bottom: 9, left: 20, id: 228},
    Tile {top: 8, right: 18, bottom: 19, left: 11, id: 229},
    Tile {top: 8, right: 11, bottom: 17, left: 18, id: 230},
    Tile {top: 8, right: 11, bottom: 17, left: 12, id: 231},
    Tile {top: 15, right: 15, bottom: 15, left: 18, id: 232},
    Tile {top: 15, right: 9, bottom: 19, left: 16, id: 233},
    Tile {top: 15, right: 16, bottom: 9, left: 16, id: 234},
    Tile {top: 15, right: 16, bottom: 9, left: 18, id: 235},
    Tile {top: 15, right: 16, bottom: 17, left: 10, id: 236},
    Tile {top: 15, right: 17, bottom: 16, left: 19, id: 237},
    Tile {top: 15, right: 18, bottom: 10, left: 20, id: 238},
    Tile {top: 15, right: 18, bottom: 12, left: 10, id: 239},
    Tile {top: 15, right: 11, bottom: 9, left: 12, id: 240},
    Tile {top: 15, right: 11, bottom: 16, left: 12, id: 241},
    Tile {top: 9, right: 10, bottom: 10, left: 17, id: 242},
    Tile {top: 9, right: 10, bottom: 18, left: 11, id: 243},
    Tile {top: 9, right: 16, bottom: 20, left: 18, id: 244},
    Tile {top: 9, right: 12, bottom: 19, left: 20, id: 245},
    Tile {top: 10, right: 10, bottom: 12, left: 20, id: 246},
    Tile {top: 10, right: 16, bottom: 17, left: 16, id: 247},
    Tile {top: 10, right: 17, bottom: 16, left: 12, id: 248},
    Tile {top: 10, right: 18, bottom: 19, left: 12, id: 249},
    Tile {top: 16, right: 16, bottom: 20, left: 11, id: 250},
    Tile {top: 16, right: 17, bottom: 11, left: 17, id: 251},
    Tile {top: 17, right: 11, bottom: 19, left: 18, id: 252},
    Tile {top: 18, right: 19, bottom: 12, left: 19, id: 253},
    Tile {top: 18, right: 20, bottom: 19, left: 20, id: 254},
    Tile {top: 11, right: 20, bottom: 12, left: 20, id: 255},
];




//
// fn load_tiles() -> [Tile; 256] {
//     // Tiles are stored inside data.csv
//     // Each row is a tile
//     // The first  column is the tileID, the next 4 columns are the top, right, bottom, left sides
//
//     // Load the data. data.csv is in the same directory as the the src
//     let mut reader = csv::Reader::from_path("data.csv").unwrap();
//
//     // Create a thing to store the tiles. We know the size to be 256, so can stack allocate
//     let mut tiles = [EMPTY_TILE; 256];
//
//
//     for (i, result) in reader.records().enumerate() {
//
//         // Get the row
//         let record = result.unwrap();
//
//         // The first 4 tiles are corner tiles with no bottom or left side
//         // The next 56 tiles are edge tiles with no left side
//         // The next 196 tiles are normal tiles
//
//         // The edges of tiles are stored as a number from 1 to 22. I want 0 indexing, with -1 as empty
//         // Rust doesn't like going straight to an i8, so we have to do some weird stuff
//         if i < 4 { // Corner tile
//             tiles [i] = Tile {
//                 top: i8::from_str_radix(&record[1], 10).unwrap() - 1,
//                 right: i8::from_str_radix(&record[2], 10).unwrap() - 1,
//                 bottom: -1,
//                 left: -1,
//             };
//         } else if i < 60 { // Edge tile
//             tiles [i] = Tile {
//                 top: i8::from_str_radix(&record[1], 10).unwrap() - 1,
//                 right: i8::from_str_radix(&record[2], 10).unwrap() - 1,
//                 bottom: i8::from_str_radix(&record[3], 10).unwrap() - 1,
//                 left: -1,
//             };
//         } else { // Normal tile
//             tiles [i] = Tile {
//                 top: i8::from_str_radix(&record[1], 10).unwrap() - 1,
//                 right: i8::from_str_radix(&record[2], 10).unwrap() - 1,
//                 bottom: i8::from_str_radix(&record[3], 10).unwrap() - 1,
//                 left: i8::from_str_radix(&record[4], 10).unwrap() - 1,
//             };
//         }
//     }
//     tiles
// }