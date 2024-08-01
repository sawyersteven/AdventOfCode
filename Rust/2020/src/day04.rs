use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day04 {
    template: HashMap<String, String>,
    passports: Vec<HashMap<String, String>>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 {
            passports: Vec::new(),
            template: HashMap::new(),
        };
    }

    fn passport_from_string(&self, data: &str) -> HashMap<String, String> {
        let mut pp = HashMap::new();
        for line in data.split(|x| x == '\n' || x == ' ') {
            let mut kv = line.split(':');
            pp.insert(kv.next().unwrap().to_string(), kv.next().unwrap().to_string());
        }
        return pp;
    }

    fn is_valid_hex(&self, hex: &str) -> bool {
        for b in hex.bytes() {
            if !(b'0'..=b'9').contains(&b) && !(b'a'..=b'f').contains(&b) {
                return false;
            }
        }
        return true;
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        let mut groups = raw_input.split("\n\n");

        self.template = self.passport_from_string(groups.next().unwrap());
        while let Some(g) = groups.next() {
            self.passports.push(self.passport_from_string(g));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut req_fields: HashSet<&String> = self.template.keys().collect();
        req_fields.remove(&String::from("cid"));

        let mut good_passports = 1;
        for passport in &self.passports {
            let set = passport.keys().collect();
            if req_fields.difference(&set).count() == 0 {
                good_passports += 1;
            }
        }

        return Box::new(good_passports);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const EYES: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

        let mut req_fields: HashSet<&String> = self.template.keys().collect();
        req_fields.remove(&String::from("cid"));

        let mut good_passports = 0;
        for passport in &self.passports {
            let set = passport.keys().collect();
            if req_fields.difference(&set).count() != 0 {
                continue;
            }
            if !(1920..=2002).contains(&passport[&String::from("byr")].parse::<usize>().unwrap()) {
                continue;
            }

            if !(2010..=2020).contains(&passport[&String::from("iyr")].parse::<usize>().unwrap()) {
                continue;
            }

            if !(2020..=2030).contains(&passport[&String::from("eyr")].parse::<usize>().unwrap()) {
                continue;
            }

            if passport[&String::from("hgt")].len() < 4 {
                continue;
            }
            let hgt: usize = passport[&String::from("hgt")][0..passport[&String::from("hgt")].len() - 2]
                .parse()
                .unwrap();
            if passport[&String::from("hgt")][(passport[&String::from("hgt")].len() - 2)..] == *"in" {
                if hgt < 59 || hgt > 76 {
                    continue;
                };
            } else if hgt < 150 || hgt > 193 {
                continue;
            };

            if passport[&String::from("hcl")].as_bytes()[0] != b'#' {
                continue;
            }

            if !self.is_valid_hex(&passport[&String::from("hcl")][1..]) {
                continue;
            }

            if !EYES.contains(&&*passport[&String::from("ecl")]) {
                continue;
            };

            let pid = &passport[&String::from("pid")];
            if pid.len() != 9 || pid.parse::<usize>().is_err() {
                continue;
            }

            good_passports += 1;
        }
        return Box::new(good_passports);
    }
}
