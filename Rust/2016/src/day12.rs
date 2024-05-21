use crate::Base;
use std::fmt::Display;

#[derive(Clone, Debug)]
enum Word {
    CPY,
    INC,
    DEC,
    JNZ,
    OUT,
    TGL,
}

#[derive(Clone)]
pub struct Command {
    word: Word,
    in_val: isize,
    in_is_reg: bool,
    out_val: isize,
    out_is_reg: bool,
}

impl Word {
    pub fn toggle(&self) -> Word {
        match self {
            &Word::CPY => Word::JNZ,
            &Word::INC => Word::DEC,
            &Word::DEC => Word::INC,
            &Word::JNZ => Word::CPY,
            &Word::OUT => todo!(),
            &Word::TGL => Word::INC,
        }
    }
}

pub struct Day12 {
    pub input: Vec<Command>,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 { input: Vec::new() };
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = parse_code(raw_input);
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut rnnr = CodeRunner::new(self.input.clone());
        rnnr.next();
        return Box::new(rnnr.registers[0]);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut rnnr = CodeRunner::new(self.input.clone());
        rnnr.registers[2] = 1;
        rnnr.next();
        return Box::new(rnnr.registers[0]);
    }
}

pub struct CodeRunner {
    code: Vec<Command>,
    pub registers: [isize; 4],
    index: isize,
}

impl CodeRunner {
    pub fn new(code: Vec<Command>) -> CodeRunner {
        return CodeRunner {
            code: code,
            registers: [0, 0, 0, 0],
            index: 0,
        };
    }

    // Runs code until next output command is handled
    pub fn next(&mut self) -> Option<isize> {
        while self.index < self.code.len() as isize {
            let cmd = &self.code[self.index as usize];
            match cmd.word {
                Word::CPY => {
                    self.registers[cmd.out_val as usize] = if cmd.in_is_reg {
                        self.registers[cmd.in_val as usize]
                    } else {
                        cmd.in_val
                    };
                }
                Word::INC => {
                    self.registers[cmd.in_val as usize] += 1;
                }
                Word::DEC => {
                    self.registers[cmd.in_val as usize] -= 1;
                }
                Word::JNZ => {
                    let nz = if cmd.in_is_reg {
                        self.registers[cmd.in_val as usize]
                    } else {
                        cmd.in_val
                    } != 0;

                    if nz {
                        let jmp = if cmd.out_is_reg {
                            self.registers[cmd.out_val as usize]
                        } else {
                            cmd.out_val
                        };
                        self.index += jmp - 1;
                    }
                }
                Word::TGL => {
                    let offset = if cmd.in_is_reg {
                        (self.registers[cmd.in_val as usize] + self.index) as usize
                    } else {
                        (cmd.in_val + self.index) as usize
                    };
                    if offset < self.code.len() {
                        self.code[offset].word = self.code[offset].word.toggle();
                    }
                }
                Word::OUT => {
                    let ret: Option<isize>;
                    if self.code[self.index as usize].in_is_reg {
                        ret = Some(self.registers[self.code[self.index as usize].in_val as usize]);
                    } else {
                        ret = Some(self.code[self.index as usize].in_val);
                    }
                    self.index += 1;
                    return ret;
                }
            }
            self.index += 1;
        }
        return None;
    }
}

pub fn parse_code(raw_input: String) -> Vec<Command> {
    let mut commands = Vec::<Command>::new();
    for line in raw_input.split('\n').collect::<Vec<&str>>() {
        let parts = line.split(' ').collect::<Vec<&str>>();

        let word = match parts[0] {
            "cpy" => Word::CPY,
            "inc" => Word::INC,
            "dec" => Word::DEC,
            "jnz" => Word::JNZ,
            "out" => Word::OUT,
            "tgl" => Word::TGL,
            _ => {
                println!("{}", parts[0]);
                panic!();
            }
        };

        let mut cmd = Command {
            word: word,
            in_val: 0,
            in_is_reg: false,
            out_val: 0,
            out_is_reg: false,
        };

        let a_chunk = &parts[1][0..1].chars().collect::<Vec<char>>();
        if a_chunk[0] >= 'a' {
            cmd.in_val = a_chunk[0] as isize - 'a' as isize;
            cmd.in_is_reg = true;
        } else {
            cmd.in_val = parts[1].parse().unwrap();
            cmd.in_is_reg = false;
        }

        if parts.len() > 2 {
            let b_chunk = &parts[2].chars().nth(0).unwrap();
            if b_chunk >= &'a' {
                cmd.out_val = *b_chunk as isize - 'a' as isize;
                cmd.out_is_reg = true;
            } else {
                cmd.out_val = parts[2].parse().unwrap();
                cmd.out_is_reg = false;
            }
        }
        commands.push(cmd);
    }
    return commands;
}
