use std::fmt::Display;

use crate::Base;

pub struct Day09 {
    input: String,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { input: String::new() };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        // parse input
        let mut files = Vec::with_capacity(2 + self.input.len() / 2);
        let input = self.input.as_bytes();
        let mut id = 0;
        for (i, b) in input.iter().enumerate() {
            let n = (b - b'0') as usize;
            if i % 2 == 0 {
                files.push(File { len: n, id: id, gap: 0 });
                id += 1;
            } else {
                let lasti = files.len() - 1;
                files[lasti].gap = n;
            }
        }

        let mut checksum = 0;
        let mut pos = 0;

        let mut i = 0;
        let mut last_valid = files.len() - 1;
        loop {
            let f = &files[i];
            // consume file
            for _ in 0..f.len {
                checksum += pos * (f.id);
                pos += 1;
            }

            // fill gap
            let mut gap = f.gap;
            while gap > 0 {
                if files[last_valid].len > gap {
                    for _ in 0..gap {
                        checksum += pos * (files[last_valid].id);
                        pos += 1;
                    }
                    files[last_valid].len -= gap;
                    break;
                } else {
                    for _ in 0..files[last_valid].len {
                        checksum += pos * (files[last_valid].id);
                        pos += 1;
                    }
                    gap -= files[last_valid].len;
                    files[last_valid].len = 0;
                    last_valid -= 1;
                    if last_valid == i {
                        break;
                    }
                }
            }
            i += 1;
            if i >= (last_valid) {
                break;
            }
        }
        return Box::new(checksum);
    }

    // this is not super efficient because of the removes/inserts but rust
    // doesn't play nice with linked lists so this is it. I'm sure there's a
    // better way.
    fn part2(&mut self) -> Box<dyn Display> {
        // parse input
        let mut files = Vec::with_capacity(2 + self.input.len() / 2);

        let input = self.input.as_bytes();
        let mut id = 0;
        for i in (0..(input.len() - 1)).step_by(2) {
            files.push(File {
                len: (input[i] - b'0') as usize,
                id: id,
                gap: (input[i + 1] - b'0') as usize,
            });
            id += 1;
        }
        files.push(File {
            len: (input[input.len() - 1] - b'0') as usize,
            id: id,
            gap: 0,
        });

        /* This one is hard to read, so here's some pictures

        00....111..22233333
         ^         ^
         anchor    move

        00222.111.....33333
                ^ file to left of move's gap += move's gap + move's len
            ^ move's new gap is anchor's gap - move's len (one in this instance)
         ^ anchor's new gap is zero

         */
        let mut i = files.len() - 1;
        while i > 0 {
            for j in 0..i {
                if files[j].gap >= files[i].len {
                    // set new gaps
                    files[i - 1].gap += files[i].gap + files[i].len;
                    files[i].gap = files[j].gap - files[i].len;
                    files[j].gap = 0;

                    // move file
                    let move_file = files.remove(i);
                    files.insert(j + 1, move_file);
                    // since a file was moved to earlier in the list, bump index
                    i += 1;
                    break;
                }
            }
            i -= 1;
        }
        let mut checksum = 0;
        let mut pos = 0;
        for f in &files {
            for _ in 0..f.len {
                checksum += f.id * pos;
                pos += 1;
            }
            pos += f.gap;
        }

        return Box::new(checksum);
    }
}

#[derive(PartialEq, Eq, Hash)]
struct File {
    len: usize,
    id: usize,
    gap: usize,
}
