
use crate::day20::part1::Puzzle;
use std::fs;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("input/day20_test.txt").unwrap();
    let puzzle = Puzzle::parse(&input);
    puzzle.print();
    (
        0,
        0
    )
}

mod part1 {
    pub struct Puzzle {
        array: Vec<Vec<Tile>>
    }

    impl Puzzle {
        pub fn parse(input: &str) -> Puzzle {

            let d = input
                .split("\n\n")
                .count();

            let d = (d as f64).sqrt() as usize;

            let mut tiles = input.split("\n\n");

            let mut array: Vec<Vec<Tile>> = Vec::new();

            for _y in 0..d {
                let mut row: Vec<Tile> = Vec::new();
                for _x in 0..d {
                    let tile = Tile::parse(tiles.next().unwrap());
                    row.push(tile);
                }
                array.push(row);
            }

            Puzzle {
                array
            }
        }

        pub fn print(&self) {
            let h = self.array.len();
            let w = self.array.first().unwrap().len();
            for y in 0..h {
                for x in 0..w {
                    let tile = &self.array[y][x];
                    tile.print_title();
                    print!(" ");
                }               
                println!();             
                for r in 0..10 {
                    for x in 0..w {
                        let tile = &self.array[y][x];
                        tile.print_row(r);
                        print!(" ");
                    }               
                    println!();             
                }
                println!();             
            }
        }
    }

    struct Tile {
        id: usize,
        image: Vec<Vec<bool>>
    }

    impl Tile {
        fn parse(text: &str) -> Tile {

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

            Tile {
                id,
                image
            }
        }

        fn print_title(&self) {
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
}