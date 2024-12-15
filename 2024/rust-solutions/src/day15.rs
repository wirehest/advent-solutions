// Standard
use std::mem;

// Local
use crate::read_input;
use crate::Part;

// External

type Map = Vec<Vec<char>>;
type Moves = Vec<char>;
type Move = char;

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    x: usize,
    y: usize,
}

struct Warehouse {
    map: Map,
    robot: Coordinates,
    height: usize,
    width: usize,
}

impl Warehouse {
    fn from(map: Map) -> Self {
        let robot = Warehouse::get_robot_position(&map).expect("robot should be on map");
        let height = map.len();
        let width = map.first().expect("map should be non-empty").len();

        Warehouse {
            map,
            robot,
            height,
            width,
        }
    }

    fn get_robot_position(map: &[Vec<char>]) -> Option<Coordinates> {
        for (y, row) in map.iter().enumerate() {
            for (x, node) in row.iter().enumerate() {
                if *node == '@' {
                    return Some(Coordinates { y, x });
                }
            }
        }

        None
    }

    fn move_robot(&mut self, movement: &Move) {
        let look_ahead = self.make_look_ahead(movement);

        if !look_ahead
            .iter()
            .map(|ahead| ahead.0)
            .collect::<Vec<Move>>()
            .contains(&'.')
        {
            return;
        }

        // let look_ahead_string: String = look_ahead.iter().map(|ahead| ahead.0).collect();
        // println!("movement: {movement}\nlook_ahead: {look_ahead_string:?}");
        // println!("full look ahead: {look_ahead:?}");
        // println!("robot pos before: {:?}", self.robot);

        let mut previous = '.';
        for ahead in look_ahead.iter() {
            let node = ahead.0;
            let Coordinates { y, x } = ahead.1;

            if node == '@' || node == 'O' {
                mem::swap(&mut self.map[y][x], &mut previous);
            }
            if node == '.' {
                mem::swap(&mut self.map[y][x], &mut previous);
                break;
            }
        }
        self.robot = Warehouse::get_robot_position(&self.map).unwrap();
        // println!("robot pos after: {:?}", self.robot);
        // self.draw();
    }

    fn make_look_ahead(&self, movement: &Move) -> Vec<(Move, Coordinates)> {
        let Coordinates { y, x } = self.robot;
        match movement {
            '^' => (0..=y)
                .rev()
                .map(|ahead_y| {
                    let node = self.map.get(ahead_y).unwrap().get(x).unwrap();
                    (*node, Coordinates { y: ahead_y, x })
                })
                .take_while(|(node, _)| node != &'#')
                .collect(),
            '>' => (x..=self.width)
                .map(|ahead_x| {
                    let node = self.map.get(y).unwrap().get(ahead_x).unwrap();
                    (*node, Coordinates { y, x: ahead_x })
                })
                .take_while(|(node, _)| node != &'#')
                .collect(),
            'v' => (y..=self.height)
                .map(|ahead_y| {
                    let node = self.map.get(ahead_y).unwrap().get(x).unwrap();
                    (*node, Coordinates { y: ahead_y, x })
                })
                .take_while(|(node, _)| node != &'#')
                .collect(),
            '<' => (0..=x)
                .rev()
                .map(|ahead_x| {
                    let node = self.map.get(y).unwrap().get(ahead_x).unwrap();
                    (*node, Coordinates { y, x: ahead_x })
                })
                .take_while(|(node, _)| node != &'#')
                .collect(),
            _ => unreachable!(),
        }
    }

    fn sum_gps(&self) -> usize {
        self.map
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, node)| {
                        if node == &'O' {
                            Some(100 * y + x)
                        } else {
                            None
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn draw(&self) {
        self.map.iter().for_each(|l| {
            let line: String = l.iter().collect();
            println!("{line}");
        });
        println!("\n");
    }
}

const PUZZLE_INPUT: &str = "data/day15.txt";

fn solve(raw_input: &str, part: Part) -> usize {
    let (map, moves) = parse(raw_input);
    let mut warehouse = Warehouse::from(map);

    // println!("START");
    // warehouse.draw();
    for movement in moves {
        warehouse.move_robot(&movement);
    }

    // warehouse.draw();

    warehouse.sum_gps()
}

fn parse(raw_input: &str) -> (Map, Moves) {
    let (raw_map, raw_moves) = raw_input
        .split_once("\n\n")
        .expect("input should be splittable");

    let parsed_map: Vec<Vec<char>> = raw_map
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();
    let parsed_moves: Vec<char> = raw_moves.replace("\n", "").chars().collect();

    (parsed_map, parsed_moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        const SAMPLE_INPUT: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let sum_gps = solve(SAMPLE_INPUT, Part::One);
        println!("sum_gps: {sum_gps}");
        assert_eq!(2_028, sum_gps);
    }

    #[test]
    fn part1_sample2() {
        const SAMPLE_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let sum_gps = solve(SAMPLE_INPUT, Part::One);
        println!("sum_gps: {sum_gps}");
        assert_eq!(10_092, sum_gps);
    }

    // #[test]
    // fn part2_sample() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     solve(SAMPLE_INPUT, Part::Two);
    // }

    #[test]
    fn part1_actual() {
        let raw_input = read_input(PUZZLE_INPUT).unwrap();
        let sum_gps = solve(&raw_input, Part::One);
        println!("sum_gps: {sum_gps}");
        // assert_eq!(10_092, sum_gps);
    }

    // #[test]
    // fn part2_actual() {
    //     let raw_input = read_input(PUZZLE_INPUT).unwrap();
    //     solve(&raw_input, Part::Two);
    // }
}
