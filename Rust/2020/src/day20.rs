/*
Debugging this is a pain. I have a working C# solution and I'm tried
of working on this
*/

use shared::grid2d::Grid2D;

use crate::Base;
use std::fmt::Display;

const SQ_SZ: usize = 10;
const NORTH: usize = 0;
const EAST: usize = 1;
const SOUTH: usize = 2;
const WEST: usize = 3;

/*
All edges read clockwise around center:

..->..
......
^....|
|....v
......
..<-..

*/
#[derive(Clone)]
struct Tile {
    id: usize,
    image: Grid2D<bool>,
    edges: [[bool; SQ_SZ]; 4],
    edge_matches: usize,
}

impl Tile {
    fn rotate_cw(&mut self) {
        self.image.rotate_cw();

        let tmp = self.edges[NORTH];
        self.edges[NORTH] = self.edges[WEST];
        self.edges[WEST] = self.edges[SOUTH];
        self.edges[SOUTH] = self.edges[EAST];
        self.edges[EAST] = tmp;
    }

    fn rotate_ccw(&mut self) {
        self.image.rotate_ccw();
        let tmp = self.edges[NORTH];
        self.edges[NORTH] = self.edges[EAST];
        self.edges[EAST] = self.edges[SOUTH];
        self.edges[SOUTH] = self.edges[WEST];
        self.edges[WEST] = tmp;
    }

    fn flip_horizontal(&mut self) {
        self.image.flip_horizontal();

        let mut tmp = self.edges[EAST].clone();
        tmp.reverse();
        self.edges[EAST] = self.edges[WEST].clone();
        self.edges[EAST].reverse();
        self.edges[WEST] = tmp;

        self.edges[NORTH].reverse();
        self.edges[SOUTH].reverse();
    }

    fn flip_vertical(&mut self) {
        self.image.flip_vertical();

        let mut tmp = self.edges[SOUTH].clone();
        tmp.reverse();
        self.edges[SOUTH] = self.edges[NORTH].clone();
        self.edges[SOUTH].reverse();
        self.edges[NORTH] = tmp;
        self.edges[EAST].reverse();
        self.edges[WEST].reverse();
    }
}

pub struct Day20 {
    tiles: Vec<Tile>,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 { tiles: Vec::new() };
    }

    fn find_match(&self, dir: usize, edge: &[bool; SQ_SZ], candidates: &mut Vec<Tile>) -> Tile {
        for i in 0..candidates.len() {
            for side in 0..4 {
                if edge == &candidates[i].edges[side] {
                    let rot = (6 + dir - side) % 4;
                    if rot == 1 {
                        candidates[i].rotate_cw();
                    } else if rot == 2 {
                        candidates[i].rotate_cw();
                        candidates[i].rotate_cw();
                    } else if rot == 3 {
                        candidates[i].rotate_ccw();
                    }
                    if dir % 2 != 0 {
                        candidates[i].flip_vertical();
                    }
                    return candidates.remove(i);
                }
            }
            candidates[i].flip_horizontal();
            for side in 0..4 {
                if edge == &candidates[i].edges[side] {
                    let rot = (6 + dir - side) % 4;
                    if rot == 1 {
                        candidates[i].rotate_cw();
                    } else if rot == 2 {
                        candidates[i].rotate_cw();
                        candidates[i].rotate_cw();
                    } else if rot == 3 {
                        candidates[i].rotate_ccw();
                    }
                    if dir % 2 != 0 {
                        candidates[i].flip_vertical();
                    }
                    return candidates.remove(i);
                }
            }
        }
        unreachable!();
    }

    fn assemble_map(&mut self) -> Grid2D<bool> {
        let map_edge_len: usize = f64::sqrt(self.tiles.len() as f64) as usize;

        let mut map = Vec::with_capacity(map_edge_len * map_edge_len);

        let mut corners = Vec::with_capacity(4);
        let mut fills = Vec::with_capacity(self.tiles.len() - 4);

        for t in self.tiles.drain(0..) {
            if t.edge_matches == 2 {
                corners.push(t);
            } else {
                fills.push(t);
            }
        }

        let mut top_left_corner = corners.remove(0);

        // orient corner
        let mut h = false;
        let mut v = false;
        for j in 1..fills.len() {
            if !h {
                for edge2 in fills[j].edges {
                    if top_left_corner.edges[WEST] == edge2 || eq_rev(&top_left_corner.edges[WEST], &edge2) {
                        top_left_corner.flip_horizontal();
                        h = true;
                        break;
                    }
                }
            }
            if !v {
                for edge2 in fills[j].edges {
                    if top_left_corner.edges[NORTH] == edge2 || eq_rev(&top_left_corner.edges[NORTH], &edge2) {
                        top_left_corner.flip_vertical();
                        v = true;
                        break;
                    }
                }
            }
        }

        let mut edge_to_match = top_left_corner.edges[EAST].clone();

        map.push(top_left_corner);

        let mut next_match_direction = EAST;

        // fill map
        for map_ind in 1..map.capacity() {
            let next: Tile;
            if map_ind == map_edge_len - 1
                || map_ind == map.capacity() - 1
                || map_ind == map.capacity() - map_edge_len
            {
                next = self.find_match(next_match_direction, &edge_to_match, &mut corners);
            } else {
                next = self.find_match(next_match_direction, &edge_to_match, &mut fills);
            }

            if (map_ind + 1) % map_edge_len == 0
            // at end of row
            {
                next_match_direction = SOUTH;
                edge_to_match = map[map_ind + 1 - map_edge_len].edges[SOUTH].clone();
            } else {
                next_match_direction = EAST;
                edge_to_match = next.edges[EAST].clone();
            }
            map.push(next);
        }
        // trim tiles, edges no longer matter
        for i in 0..map.len() {
            map[i].image.remove_row(0);
            let last_row = map[i].image.size().y - 1;
            map[i].image.remove_row(last_row);
            map[i].image.remove_col(1);
            let last_col = map[i].image.size().x - 1;
            map[i].image.remove_col(last_col);
        }

        // render tiles into single map
        //List<string> rendered = new List<string>();

        let full_edge_len = f64::sqrt(map.len() as f64) as usize * (SQ_SZ - 2);

        let mut linear: Vec<bool> = Vec::with_capacity(full_edge_len * full_edge_len);
        for map_row in (0..map.len()).step_by(map_edge_len) {
            for tile_line in 0..(SQ_SZ - 2) {
                for map_col in 0..map_edge_len {
                    linear.extend(map[map_row + map_col].image.iter_row(tile_line));
                }
            }
        }

        return Grid2D::from_iter(full_edge_len, full_edge_len, linear).unwrap();
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        for t in raw_input.split("\n\n") {
            let id = t[5..9].parse().unwrap();

            let image = Grid2D::from_iter(
                SQ_SZ,
                SQ_SZ,
                t[11..]
                    .as_bytes()
                    .iter()
                    .filter(|&x| *x != b'\n')
                    .map(|x| if *x == b'#' { true } else { false }),
            )
            .unwrap();

            let mut n = [false; SQ_SZ];
            n.copy_from_slice(&image.iter_row(0).cloned().collect::<Vec<bool>>());

            let edges = [
                image.iter_row(0).cloned().collect::<Vec<bool>>().try_into().unwrap(),
                image
                    .iter_col(SQ_SZ - 1)
                    .cloned()
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap(),
                image
                    .iter_row(SQ_SZ - 1)
                    .cloned()
                    .rev()
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap(),
                image
                    .iter_col(0)
                    .cloned()
                    .rev()
                    .collect::<Vec<bool>>()
                    .try_into()
                    .unwrap(),
            ];

            self.tiles.push(Tile {
                id,
                image,
                edges,
                edge_matches: 0,
            });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let tile_count = self.tiles.len();

        let mut edge_counts = vec![0; tile_count];

        for i in 0..tile_count {
            for edge in &self.tiles[i].edges {
                for j in (i + 1)..tile_count {
                    for edge2 in &self.tiles[j].edges {
                        if edge == edge2 || eq_rev(edge, edge2) {
                            edge_counts[i] += 1;
                            edge_counts[j] += 1;
                        }
                    }
                }
            }
        }

        for (i, c) in edge_counts.iter().enumerate() {
            self.tiles[i].edge_matches = *c;
        }

        let mut ans = 1;
        for (i, _) in edge_counts.iter().enumerate().filter(|&(_, v)| *v == 2) {
            ans *= self.tiles[i].id;
        }
        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        self.assemble_map();
        return Box::new("-");
    }
}

// if a is equal the b reversed
fn eq_rev(a: &[bool], b: &[bool]) -> bool {
    return (0..SQ_SZ).all(|i| a[i] == b[SQ_SZ - i - 1]);
}
