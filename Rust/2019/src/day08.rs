use crate::Base;
use std::fmt::Display;

const W: usize = 25;
const H: usize = 6;
const LAYER_SZ: usize = W * H;
const BLACK: u8 = b'0';
const WHITE: u8 = b'1';
const TRANS: u8 = b'2';

pub struct Day08 {
    input: Vec<u8>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { input: Vec::new() };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.as_bytes().to_vec();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut layer_match = (isize::MAX, 0, 0);
        for i in (0..self.input.len()).step_by(LAYER_SZ) {
            let mut counts = (0, 0, 0);
            for b in self.input[i..(i + LAYER_SZ)].iter() {
                match *b {
                    BLACK => counts.0 += 1,
                    WHITE => counts.1 += 1,
                    TRANS => counts.2 += 1,
                    _ => panic!(),
                }
            }
            if counts.0 < layer_match.0 {
                layer_match = counts;
            }
        }
        return Box::new(layer_match.1 * layer_match.2);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut layers = Vec::new();

        for i in (0..self.input.len()).step_by(LAYER_SZ) {
            let mut layer = Vec::new();
            let l = &self.input[i..(i + LAYER_SZ)];
            let mut j = 0;
            while j * W < l.len() {
                layer.push(&l[(j * W)..((j * W) + W)]);
                j += 1;
            }
            layers.push(layer);
        }

        let mut image = Vec::with_capacity(H);
        for h in 0..H {
            image.push(vec![0; W]);
            for w in 0..W {
                for layer in &layers {
                    if layer[h][w] == TRANS {
                        continue;
                    }
                    if layer[h][w] == BLACK {
                        image[h][w] = b' ';
                    } else {
                        image[h][w] = b'#';
                    }
                    break;
                }
            }
        }

        let mut sb = String::new();
        for row in image {
            sb.push('\n');
            sb.push_str(&*(row.iter().map(|x| *x as char).collect::<String>()));
        }

        return Box::new(sb);
    }
}
