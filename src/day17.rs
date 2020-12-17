use std::fs;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("input/day17.txt").unwrap();
//     let input = r".#.
// ..#
// ###";

    let initial_region = parse(&input);
    (
        part1::Space::new(&initial_region).active_after_n_cycles(6),
        part2::Space::new(&initial_region).active_after_n_cycles(6),
    )
}

fn parse(input: &str) -> Vec<Vec<char>> {

    fn parse_line(line: &str) -> Vec<char> {
        line
            .trim()
            .chars()
            .collect()
    }

    input
        .lines()
        .map(parse_line)
        .collect()
}

mod part1 {
    #[derive(Debug)]   
    struct Point {
        x: usize,
        y: usize,
        z: usize,
    }

    impl Point {
        fn new(x: usize, y: usize, z: usize) -> Point {
            Point {x,y,z}
        }

        fn neighbours(&self) -> Vec<Point> {
            let p = self;
            let mut n: Vec<Point> = Vec::new();
            for z in 0..3 {
                for y in 0..3 {
                    for x in 0..3 {
                        if x != 1 || y != 1 || z != 1 {
                            n.push(Point::new(x + p.x - 1, y + p.y -1, z + p.z - 1));       
                        }
                    }
                }
            }
            n
        }
    }

    #[derive(Debug)]   
    pub struct Space {
        origin: Point,
        array: Vec<Vec<Vec<char>>>
    }

    impl Space {

        pub fn new(initial_region: &Vec<Vec<char>>) -> Space {

            let w = initial_region.len();

            // Calculate the dimensions of the final region
            let d = 2 + w + 6 * 2;

            // Initialize the array with '.'s
            let mut array: Vec<Vec<Vec<char>>> = vec![vec![vec!['.'; d]; d]; d];

            // Set the "origin" to be the centre of the region
            let origin = Point::new(d/2, d/2, d/2);

            // Set the initial region
            let z = origin.z;
            for x in 0..w {
                for y in 0..w {
                    array[origin.x + x - w/2][origin.y + y - w/2][z] = initial_region[y][x];
                }
            }

            Space {
                origin,
                array
            }
        }

        pub fn active_after_n_cycles(&mut self, n: usize) -> usize {
            for i in 0..n {
                self.do_cycle(i);
            }
            self
                .array
                .iter()
                .flatten()
                .flatten()
                .filter(|c| **c == '#')
                .count()
        }

        fn do_cycle(&mut self, cycle: usize) {
            let a = &mut self.array;
            let b = a.clone();
            let w = a.len();
            let d = w - ((6 - cycle) * 2);
            let o = (w - d) / 2;
            let e = o + d;

            for z in o..e {
                for y in o..e {
                    for x in o..e {
                        let c = Space::new_state(&b, Point::new(x, y, z));
                        a[x][y][z] = c;
                        //print!("{}", c);
                    }
                    //println!("");
                }
                //println!("");
            }
        }

        fn new_state(a: &Vec<Vec<Vec<char>>>, p: Point) -> char {
            
            let active = p
                .neighbours()
                .iter()
                .map(|q| a[q.x][q.y][q.z])
                .filter(|c| *c == '#')
                .count();

            match a[p.x][p.y][p.z] {
                '#' => {
                    if active == 2 || active == 3 {'#'} else {'.'}
                },
                '.' => {
                    if active == 3 {'#'} else {'.'}
                },
                _ => panic!("Broken")
            }
        }
    }
}

mod part2 {
    #[derive(Debug)]   
    struct Point {
        x: usize,
        y: usize,
        z: usize,
        w: usize,
    }

    impl Point {
        fn new(x: usize, y: usize, z: usize, w: usize) -> Point {
            Point {x,y,z,w}
        }

        fn neighbours(&self) -> Vec<Point> {
            let p = self;
            let mut n: Vec<Point> = Vec::new();
            for w in 0..3 {
                for z in 0..3 {
                    for y in 0..3 {
                        for x in 0..3 {
                            if x != 1 || y != 1 || z != 1 || w != 1 {
                                n.push(Point::new(x + p.x - 1, y + p.y -1, z + p.z - 1, w + p.w - 1));       
                            }
                        }
                    }
                }
            }
            n
        }
    }

    #[derive(Debug)]   
    pub struct Space {
        origin: Point,
        array: Vec<Vec<Vec<Vec<char>>>>
    }

    impl Space {

        pub fn new(initial_region: &Vec<Vec<char>>) -> Space {

            let s = initial_region.len();

            // Calculate the dimensions of the final region
            let d = 2 + s + 6 * 2;

            // Initialize the array with '.'s
            let mut array: Vec<Vec<Vec<Vec<char>>>> = vec![vec![vec![vec!['.'; d]; d]; d]; d];

            // Set the "origin" to be the centre of the region
            let origin = Point::new(d/2, d/2, d/2, d/2);

            // Set the initial region
            let z = origin.z;
            let w = origin.w;
            for x in 0..s {
                for y in 0..s {
                    array[origin.x + x - s/2][origin.y + y - s/2][z][w] = initial_region[y][x];
                }
            }

            Space {
                origin,
                array
            }
        }

        pub fn active_after_n_cycles(&mut self, n: usize) -> usize {
            for i in 0..n {
                self.do_cycle(i);
            }
            self
                .array
                .iter()
                .flatten()
                .flatten()
                .flatten()
                .filter(|c| **c == '#')
                .count()
        }

        fn do_cycle(&mut self, cycle: usize) {
            let a = &mut self.array;
            let b = a.clone();
            let s = a.len();
            let d = s - ((6 - cycle) * 2);
            let o = (s - d) / 2;
            let e = o + d;

            for w in o..e {
                for z in o..e {
                    for y in o..e {
                        for x in o..e {
                            let c = Space::new_state(&b, Point::new(x,y,z,w));
                            a[x][y][z][w] = c;
                            //print!("{}", c);
                        }
                        //println!("");
                    }
                    //println!("");
                }
                //println!("");
            }
        }

        fn new_state(a: &Vec<Vec<Vec<Vec<char>>>>, p: Point) -> char {
            
            let active = p
                .neighbours()
                .iter()
                .map(|q| a[q.x][q.y][q.z][q.w])
                .filter(|c| *c == '#')
                .count();

            match a[p.x][p.y][p.z][p.w] {
                '#' => {
                    if active == 2 || active == 3 {'#'} else {'.'}
                },
                '.' => {
                    if active == 3 {'#'} else {'.'}
                },
                _ => panic!("Broken")
            }
        }
    }
}