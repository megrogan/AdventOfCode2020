
use crate::day20::part1::*;
use std::fs;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("input/day20.txt").unwrap();

    // Build a hashmap of id -> tile
    // For each tile also enumerate each edge (and in flipped orientation) as a number
    let tiles = parse(&input);

    // for tile in tiles.values() {
    //     tile.print_title();
    //     print!(" ");
    //     tile.print_edges();
    //     println!();
    // }

    // Build a map of id -> neighbours (a neighbour consists of id and the connecting edge number)
    // A major assumption for this puzzle is that each edge only matches once - if not it is a much harder problem!
    // I agonised for quite some time until I realised we could make this assumption!!
    // Given that we expect:
    // internal tiles to have 4 neighbours
    // edge tiles to have 3 neighbours and 
    // corner tiles to have 2 neighbours
    let neighbours = find_all_neighbours(&tiles);
    let corners = find_corners(&neighbours);
    (
        corners.iter().product(),
        0
    )
}

mod part1 {
    use std::collections::HashMap;

    pub fn parse(input: &str) -> HashMap<usize, Tile> {
        input
            .split("\n\n")
            .map(|text_block| {
                let tile = Tile::parse(text_block);
                (
                    tile.id,
                    tile
                )
            })
            .collect()
    }

    pub fn find_all_neighbours(tiles: &HashMap<usize, Tile>) -> HashMap<usize, Vec<Neighbour>> {
        tiles
            .values()
            .map(|tile| {
                (
                    tile.id,
                    tiles
                        .iter()
                        .filter(|(id, _)| **id != tile.id) // exclude self
                        .filter_map(|(_, other)| tile.find_neighbour(other))
                        .collect()
                )
            })
            .collect()
    }

    pub fn find_corners(neighbours: &HashMap<usize, Vec<Neighbour>>) -> Vec<usize> {
        neighbours
            .iter()
            //.inspect(|p| println!("{:?}", p))
            .filter(|(_, neighbours)| neighbours.len() == 2)
            .map(|(id, _)| *id)
            .collect()
    }

    #[derive(Debug)] 
    pub struct Neighbour {
        tile_id: usize,
        edge: u32, // 0-7 4 edges * 2 flips
    }

    impl Neighbour {
        fn new(tile_id: usize, edge: u32) -> Neighbour {
            Neighbour {
                tile_id,
                edge
            }
        }
    }

    #[derive(Debug)] 
    pub struct Tile {
        id: usize,
        image: Vec<Vec<bool>>,
        edges: Vec<u32>, // [top, right, bottom, left, flip top, flip right, flip bottom, flip left]
    }

    impl Tile {
        fn parse(text: &str) -> Tile {

            fn to_int(edge: Vec<bool>) -> u32 {
                edge.iter().fold(0, |v, b| {
                    let mut val = v;
                    if *b { val = val | 1; }
                    val << 1
                })
            }

            let mut lines = text.lines();

            let id: usize = lines
                .next()
                .unwrap()
                .replace("Tile ", "")
                .replace(":", "")
                .trim()
                .parse()
                .unwrap();

            let image: Vec<Vec<bool>> = lines
                .filter_map(|line| {
                    line.chars().map(|c| match c {
                        '#' => Some(true),
                        '.' => Some(false),
                        _ => None
                    }).collect()
                })
                .collect();

            let edges = vec![
                to_int((0..10).map(|x| image[0][x]).collect()), // top
                to_int((0..10).map(|y| image[y][9]).collect()), // right
                to_int((0..10).map(|x| image[9][x]).collect()), // bottom
                to_int((0..10).map(|y| image[y][0]).collect()), // left
                to_int((0..10).rev().map(|x| image[0][x]).collect()), // flip top
                to_int((0..10).rev().map(|y| image[y][9]).collect()), // flip right
                to_int((0..10).rev().map(|x| image[9][x]).collect()), // flip bottom
                to_int((0..10).rev().map(|y| image[y][0]).collect()), // flip left
            ];

            Tile {
                id,
                image,
                edges
            }
        }

        fn find_neighbour(&self, other: &Tile) -> Option<Neighbour> {
            self
                .edges
                .iter()
                .filter_map(|e| other
                    .edges
                    .iter()
                    .filter_map(|o| if o == e {
                            Some(Neighbour::new(other.id, *o))
                        } else {
                            None
                        })
                    .next()
                )
                .next()
        }

        pub fn print_edges(&self) {
            print!("Edges {:?}", self.edges);
        }

        pub fn print_title(&self) {
            print!("Tile {}:", self.id);
        }

        fn print_row(&self, r: usize) {
            for x in 0..self.image.first().unwrap().len() {
                let c = match self.image[r][x] {
                    false => '.',
                    true => '#'
                };
                print!("{}", c);
            }
        }
    }

    // pub struct Puzzle {
    //     array: Vec<Vec<Tile>>
    // }

    // impl Puzzle {
    //     pub fn parse(input: &str) -> Puzzle {

    //         let d = input
    //             .split("\n\n")
    //             .count();

    //         let d = (d as f64).sqrt() as usize;

    //         let mut tiles = input.split("\n\n");

    //         let mut array: Vec<Vec<Tile>> = Vec::new();

    //         for _y in 0..d {
    //             let mut row: Vec<Tile> = Vec::new();
    //             for _x in 0..d {
    //                 let tile = Tile::parse(tiles.next().unwrap());
    //                 row.push(tile);
    //             }
    //             array.push(row);
    //         }

    //         Puzzle {
    //             array
    //         }
    //     }

    //     pub fn print(&self) {
    //         let h = self.array.len();
    //         let w = self.array.first().unwrap().len();
    //         for y in 0..h {
    //             for x in 0..w {
    //                 let tile = &self.array[y][x];
    //                 tile.print_title();
    //                 print!(" ");
    //             }               
    //             println!();             
    //             for r in 0..10 {
    //                 for x in 0..w {
    //                     let tile = &self.array[y][x];
    //                     tile.print_row(r);
    //                     print!(" ");
    //                 }               
    //                 println!();             
    //             }
    //             println!();             
    //         }
    //     }
    // }
}