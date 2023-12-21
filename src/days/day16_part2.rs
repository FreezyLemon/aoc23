use std::cmp::min;

use crate::LINE_SEPARATOR;

pub struct Day16Part2;

impl crate::days::Day for Day16Part2 {
    fn solve(&self, input: &str) -> String {
        let cols = input.find(LINE_SEPARATOR).expect("has line separator");
        let str_cols = LINE_SEPARATOR.len() + cols;
        let rows = 1 + (input.len() / str_cols) as Coordinate;
        let cols = cols as Coordinate;

        let nw_mirrors = get_locations(input, str_cols, '/');
        let ne_mirrors = get_locations(input, str_cols, '\\');
        let h_splitters = get_locations(input, str_cols, '-');
        let v_splitters = get_locations(input, str_cols, '|');

        let total_len = nw_mirrors.len() + ne_mirrors.len() + v_splitters.len() + h_splitters.len();
        let mut important_tiles = Vec::with_capacity(total_len);

        important_tiles.extend(nw_mirrors.into_iter().map(|l| NodeData::new(l, Tile::MirrorNorthWest)));
        important_tiles.extend(ne_mirrors.into_iter().map(|l| NodeData::new(l, Tile::MirrorNorthEast)));
        important_tiles.extend(h_splitters.into_iter().map(|l| NodeData::new(l, Tile::SplitterHorizontal)));
        important_tiles.extend(v_splitters.into_iter().map(|l| NodeData::new(l, Tile::SplitterVertical)));

        let mut energized_max = 0;
        for row in 0..rows {
            // from west
            let first_node = important_tiles.iter()
                .filter(|n| n.location.row == row)
                .min_by_key(|n| n.location.col)
                .unwrap();

            let start_location = Location {
                row,
                col: 0,
            };
            let energized = construct_graph(first_node.clone(), Direction::West, start_location, &important_tiles, cols, rows);
            if energized > energized_max {
                energized_max = energized;
            }

            // from east
            let first_node = important_tiles.iter()
                .filter(|n| n.location.row == row)
                .max_by_key(|n| n.location.col)
                .unwrap();

            let start_location = Location {
                row,
                col: cols as Coordinate - 1,
            };
            let energized = construct_graph(first_node.clone(), Direction::West, start_location, &important_tiles, cols, rows);
            if energized > energized_max {
                energized_max = energized;
            }
        }

        for col in 0..cols as Coordinate {
            // from north
            let first_node = important_tiles.iter()
                .filter(|n| n.location.col == col)
                .min_by_key(|n| n.location.row)
                .unwrap();

            let start_location = Location {
                row: 0,
                col,
            };
            let energized = construct_graph(first_node.clone(), Direction::North, start_location, &important_tiles, cols, rows);
            if energized > energized_max {
                energized_max = energized;
            }

            // from south
            let first_node = important_tiles.iter()
                .filter(|n| n.location.col == col)
                .max_by_key(|n| n.location.row)
                .unwrap();

            let start_location = Location {
                row: rows - 1,
                col,
            };
            let energized = construct_graph(first_node.clone(), Direction::South, start_location, &important_tiles, cols, rows);
            if energized > energized_max {
                energized_max = energized;
            }
        }

        energized_max.to_string()
    }
}

fn construct_graph<'a>(first_node: NodeData, from_direction: Direction, start_location: Location, important_tiles: &[NodeData], cols: Coordinate, rows: Coordinate) -> i32 {
    let mut energized_tiles = vec![false; cols as usize * rows as usize];
    // let mut energized_tiles = HashSet::new();
    let mut search_vector = vec![(first_node, from_direction)];
    let mut already_visited = Vec::new();
    loop {
        let mut next_search = Vec::new();
        for (search_node, direction_from) in search_vector {
            already_visited.push((search_node.clone(), direction_from));

            let next_nodes = search_node.next_nodes(direction_from, important_tiles, cols, rows);
            for (next_node, next_direction) in next_nodes {
                add_energized(&mut energized_tiles, cols, search_node.location, next_node.location);
                next_search.push((next_node, next_direction));
            }
        }

        next_search.retain(|elem| !already_visited.contains(elem));

        if next_search.is_empty() {
            break;
        }
        search_vector = next_search;
    }

    // add tiles from start to first mirror/splitter
    add_energized(&mut energized_tiles, cols, start_location, already_visited[0].0.location);

    energized_tiles.into_iter().filter(|b| *b).count() as i32
}

fn add_energized<'a>(set: &mut Vec<bool>, cols: Coordinate, from: Location, to: Location) {
    let Location { col: from_col, row: from_row } = from;
    let Location { col: to_col, row: to_row } = to;

    if from_col == to_col {
        let start = min(from_row, to_row);
        let end = start + (from_row - to_row).abs();
        for row in start..=end {
            let idx = row as usize * cols as usize + from_col as usize;
            set[idx] = true;
        }
    } else if from_row == to_row {
        let start = min(from_col, to_col);
        let end = start + (from_col - to_col).abs();
        for col in start..=end {
            let idx = from_row as usize * cols as usize + col as usize;
            set[idx] = true;
        }
    } else {
        panic!("from/to should always be on the same x- or y-axis");
    }
}

fn get_locations(s: &str, cols: usize, char: char) -> Vec<Location> {
    s.match_indices(char)
        .map(|(idx, _)| (idx % cols, idx / cols))
        .map(|(col, row)| (col as Coordinate, row as Coordinate))
        .map(|(col, row)| Location { col, row })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct NodeData {
    location: Location,
    tile: Tile,
}

impl NodeData {
    fn new(location: Location, tile: Tile) -> Self {
        Self {
            location,
            tile,
        }
    }

    fn next_nodes(&self, from_direction: Direction, tiles: &[NodeData], cols: Coordinate, rows: Coordinate) -> Vec<(NodeData, Direction)> {
        let Self { location: Location { col, row }, tile } = *self;

        tile.next_directions_from(from_direction)
            .into_iter()
            .map(|next_direction| {
                let next_data = match next_direction {
                    Direction::North => tiles.iter().filter(|nd| nd.location.col == col && nd.location.row < row).max_by_key(|nd| nd.location.row),
                    Direction::West => tiles.iter().filter(|nd| nd.location.row == row && nd.location.col < col).max_by_key(|nd| nd.location.col),
                    Direction::South => tiles.iter().filter(|nd| nd.location.col == col && nd.location.row > row).min_by_key(|nd| nd.location.row),
                    Direction::East => tiles.iter().filter(|nd| nd.location.row == row && nd.location.col > col).min_by_key(|nd| nd.location.col),
                };
    
                (
                    next_data.map_or_else(|| {
                        let location = match next_direction {
                            Direction::North => Location { col, row: 0 },
                            Direction::West => Location { col: 0, row },
                            Direction::South => Location { col, row: rows - 1 },
                            Direction::East => Location { col: cols - 1, row },
                        };
        
                        NodeData { location, tile: Tile::EndOfBounds }
                    }, Clone::clone),
                    next_direction.invert()
                )
            })
            .collect()
    }
}

type Coordinate = i16;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Location {
    col: Coordinate,
    row: Coordinate,
}

impl Location {

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    MirrorNorthWest, // /
    MirrorNorthEast, // \
    SplitterVertical, // |
    SplitterHorizontal, // -
    EndOfBounds, // whenever the bounds are hit
}

impl Tile {
    // fn collides_in(self, dir: Direction) -> bool {
    //     match (self, dir) {
    //         (Tile::MirrorNorthWest, _) => true,
    //         (Tile::MirrorNorthEast, _) => true,
    //         (Tile::SplitterVertical, Direction::North | Direction::South) => false,
    //         (Tile::SplitterVertical, Direction::West | Direction::East) => true,
    //         (Tile::SplitterHorizontal, Direction::North | Direction::South) => true,
    //         (Tile::SplitterHorizontal, Direction::West | Direction::East) => false,
    //         (Tile::EndOfBounds, _) => true,
    //     }
    // }

    fn next_directions_from(self, direction: Direction) -> Vec<Direction> {
        match (self, direction) {
            (Tile::MirrorNorthWest, Direction::North) => vec![Direction::West],
            (Tile::MirrorNorthWest, Direction::West) => vec![Direction::North],
            (Tile::MirrorNorthWest, Direction::South) => vec![Direction::East],
            (Tile::MirrorNorthWest, Direction::East) => vec![Direction::South],
            (Tile::MirrorNorthEast, Direction::North) => vec![Direction::East],
            (Tile::MirrorNorthEast, Direction::West) => vec![Direction::South],
            (Tile::MirrorNorthEast, Direction::South) => vec![Direction::West],
            (Tile::MirrorNorthEast, Direction::East) => vec![Direction::North],
            (Tile::SplitterVertical, Direction::North) => vec![Direction::South],
            (Tile::SplitterVertical, Direction::West | Direction::East) => vec![Direction::North, Direction::South],
            (Tile::SplitterVertical, Direction::South) => vec![Direction::North],
            (Tile::SplitterHorizontal, Direction::West) => vec![Direction::East],
            (Tile::SplitterHorizontal, Direction::North | Direction::South) => vec![Direction::West, Direction::East],
            (Tile::SplitterHorizontal, Direction::East) => vec![Direction::West],
            (Tile::EndOfBounds, _) => vec![],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Direction {
    fn invert(self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
}
