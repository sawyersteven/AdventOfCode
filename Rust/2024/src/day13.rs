use crate::Base;
use std::fmt::Display;

pub struct Day13 {
    input: String,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 { input: String::new() };
    }

    /// Solve for lowest number of A presses possible
    /// return (a, b)
    fn solve_for_a(&self, c: &Claw, limit: i128) -> (i128, i128) {
        /* Simple math but hard to read when written efficiently

        Button A: X+94, Y+34
        Button B: X+22, Y+67
        Prize: X=8400, Y=5400

        Two formulas must be true:
        94A + 22B = 8400
        34A + 67B = 5400

        Multiply each by the other's B multiplier so that both Bs are the same:
        (94A + 22B = 8400) * 67 = (6258A + 1474B = 562800)
        (34A + 67B = 5400) * 22 =  (748A + 1474B = 118800)

        Isolate B:
        1474B = 562800 - 6258A
        1474B = 118800 - 748A

        Combine and simplify:
        118800 - 748A = 562800 - 6258A
        118800 - 562800 = -6258A + 748A
        444000 = 5510A

        div floor for A, use to solve for B
        if B is not round number this cannot be solved

        */
        let a = ((c.prize.x * c.btn_b.y) - (c.prize.y * c.btn_b.x))
            / ((c.btn_a.x * c.btn_b.y) - (c.btn_a.y * c.btn_b.x));

        if a > limit {
            return (0, 0);
        }

        let b = (c.prize.x - (c.btn_a.x * a)) / c.btn_b.x;
        if b > limit {
            return (0, 0);
        }

        // check if B presses required are whole num
        if (c.prize.x - (c.btn_a.x * a)) % c.btn_b.x != 0 || (c.prize.y - (c.btn_a.y * a)) % c.btn_b.y != 0 {
            return (0, 0);
        }
        return (a, b);
    }

    /// Same as solve_for_a, but solves for lowest B presses possible
    /// return (a, b) which is (0,0) if unsolvable
    fn solve_for_b(&self, c: &Claw, limit: i128) -> (i128, i128) {
        let b = ((c.prize.x * c.btn_a.y) - (c.prize.y * c.btn_a.x))
            / ((c.btn_b.x * c.btn_a.y) - (c.btn_b.y * c.btn_a.x));

        if b > limit {
            return (0, 0);
        }

        let a = (c.prize.x - (c.btn_b.x * b)) / c.btn_a.x;
        if a > limit {
            return (0, 0);
        }

        if (c.prize.x - (c.btn_b.x * b)) % c.btn_a.x != 0 || (c.prize.y - (c.btn_b.y * b)) % c.btn_a.y != 0 {
            return (0, 0);
        }

        return (a, b);
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut cost = 0;
        for p in self.input.split("\n\n") {
            let c = Claw::from(p);

            let bdist = c.btn_b.x + c.btn_b.y;
            let adist = c.btn_a.x + c.btn_a.y;

            let (a, b) = if (bdist * 3) <= adist {
                self.solve_for_b(&c, 100)
            } else {
                self.solve_for_a(&c, 100)
            };

            cost += (a * 3) + b;
        }
        return Box::new(cost);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut cost = 0;
        for p in self.input.split("\n\n") {
            let mut c = Claw::from(p);
            c.prize.x += 10000000000000;
            c.prize.y += 10000000000000;

            let bdist = c.btn_b.x + c.btn_b.y;
            let adist = c.btn_a.x + c.btn_a.y;

            let (a, b) = if (bdist * 3) <= adist {
                self.solve_for_b(&c, i128::MAX)
            } else {
                self.solve_for_a(&c, i128::MAX)
            };

            cost += (a * 3) + b;
        }
        return Box::new(cost);
    }
}

struct Claw {
    btn_a: Coord128,
    btn_b: Coord128,
    prize: Coord128,
}

impl From<&str> for Claw {
    fn from(value: &str) -> Self {
        let lines: Vec<&str> = value.lines().collect();
        let axy = lines[0][12..].split_once(", Y+").unwrap();
        let bxy = lines[1][12..].split_once(", Y+").unwrap();
        let pxy = lines[2][9..].split_once(", Y=").unwrap();

        return Self {
            btn_a: Coord128 {
                x: axy.0.parse().unwrap(),
                y: axy.1.parse().unwrap(),
            },
            btn_b: Coord128 {
                x: bxy.0.parse().unwrap(),
                y: bxy.1.parse().unwrap(),
            },
            prize: Coord128 {
                x: pxy.0.parse().unwrap(),
                y: pxy.1.parse().unwrap(),
            },
        };
    }
}

struct Coord128 {
    x: i128,
    y: i128,
}
