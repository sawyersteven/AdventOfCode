import os
import shutil

abspath = os.path.abspath(__file__)
os.chdir(os.path.dirname(abspath))

day_template = """use crate::Base;
use std::fmt::Display;

pub struct Day__ {
    pub input: String,
}

impl Day__ {
    pub fn new() -> Day__ {
        return Day__ { input: String::from("") };
    }
}

impl Base for Day__ {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}
"""


year = input("Year [eg 2019]:")

if not os.path.exists(f"./{year}"):
    print(f"Copying project template to ./{year}")
    shutil.copytree("./year_template", f"./{year}")

with open(f"./{year}/Cargo.toml", "r+")as f:
    content = f.read()
    content = content.replace("####", year)
    f.truncate(0)
    f.seek(0)
    f.write(content)

with open(f"./{year}/Cargo.lock", "r+")as f:
    content = f.read()
    content = content.replace("####", year)
    f.truncate(0)
    f.seek(0)
    f.write(content)

with open(f"./{year}/src/runner.rs", "r+")as f:
    content = f.read()
    content = content.replace("####", year)
    f.truncate(0)
    f.seek(0)
    f.write(content)


if not os.path.exists(f"../Input/{year}"):
    print(f"Making directory ../Input/{year}")
    os.makedirs(f"../Input/{year}")

for day in range(1, 26):
    d = f"{day:02d}"
    cs = f"./{year}/src/day{d}.rs"
    if not os.path.isfile(cs):
        with open(cs, "w") as f:
            f.write(day_template.replace("__", d))
