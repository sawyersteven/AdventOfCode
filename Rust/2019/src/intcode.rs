use std::collections::VecDeque;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum StatusCode {
    InputRequest = 74,
    EOF = 38,
    OutputDelivery = 4,
    Complete = 99,
    InvalidCommand = 1,
    Null = 0,
}

pub type Response = (StatusCode, isize);

pub fn parse_code(code: &String) -> Vec<isize> {
    return code.split(',').map(|x| x.parse().unwrap()).collect();
}

pub struct Emulator {
    pub memory: Vec<isize>,
    position: usize,
    relative_base: isize,
    orig_program: Vec<isize>,
    input_queue: VecDeque<isize>,
    response: Response,
}

#[allow(unused)]
impl Emulator {
    const ADD: isize = 1;
    const MUL: isize = 2;
    const INP: isize = 3;
    const OUT: isize = 4;
    const TRU: isize = 5;
    const FAL: isize = 6;
    const LTN: isize = 7;
    const EQL: isize = 8;
    const SRB: isize = 9;
    const FIN: isize = 99;

    const MEM_EXPAND_LEN: usize = 200;

    pub fn new(program: Vec<isize>) -> Self {
        let mut e = Emulator {
            orig_program: Vec::new(),
            memory: Vec::new(),
            position: 0,
            relative_base: 0,
            input_queue: VecDeque::new(),
            response: (StatusCode::Null, 0),
        };
        e.set_program(program);
        return e;
    }

    /*
    Install a new program to the emulator
    The code passed in the `new()` function will overwritten
    */
    pub fn set_program(&mut self, mut program: Vec<isize>) {
        program.append(&mut vec![0; Self::MEM_EXPAND_LEN]);
        self.memory = program;
        self.orig_program = self.memory.clone();
    }

    pub fn queue_input(&mut self, input: &[isize]) {
        for i in input {
            self.input_queue.push_back(*i);
        }
    }

    pub fn clear_input_queue(&mut self) {
        self.input_queue.clear();
    }

    // Expands working memory with zeros
    pub fn expand_mem(&mut self, by: usize) {
        self.memory.append(&mut vec![0; by]);
    }

    // Resets to original state
    pub fn reboot(&mut self) -> Response {
        self.position = 0;
        self.input_queue.clear();
        self.relative_base = 0;

        let mut op = self.orig_program.clone();
        op.append(&mut vec![0; Self::MEM_EXPAND_LEN]);
        self.memory = op;
        return (StatusCode::Null, 0);
    }

    pub fn addr(&self, pos: usize) -> isize {
        let mode = (self.memory[self.position] / [0, 100, 1000, 10000][pos]) % 10;
        return match mode {
            0 => self.memory[self.position + pos],
            1 => (self.position + pos) as isize,
            2 => self.relative_base + self.memory[self.position + pos],
            _ => panic!(),
        };
    }

    // Expands working memory to addr + MEM_EXPAND_LEN
    fn expand_memory_to_addr(&mut self, addr: usize) {
        let len = addr + Self::MEM_EXPAND_LEN;
        self.memory.append(&mut vec![0; len - self.memory.len()]);
    }

    // Writes val to address, expanding memory if needed
    pub fn write(&mut self, val: isize, addr: usize) {
        if addr > self.memory.len() - 1 {
            self.expand_memory_to_addr(addr);
        }
        self.memory[addr] = val;
    }

    // Returns val at address, expanding memory if needed, which will return 0
    pub fn read(&mut self, addr: usize) -> isize {
        if addr > self.memory.len() - 1 {
            self.expand_memory_to_addr(addr);
        }
        return self.memory[addr];
    }

    /*
    Runs program until either...
        * Input is required but input queue is empty
        * Output is generated and returned
        * Program executes opcode 99 and is complete
        * An invalid opcode has been reached
        * The end of the program has been reached

    Calling `run` again will resume execution where the emulator stopped
    */
    pub fn run(&mut self) -> Response {
        while self.position < self.memory.len() {
            let opcode = self.memory[self.position] % 100;
            match opcode {
                Self::ADD => {
                    let addr1 = self.addr(1) as usize;
                    let addr2 = self.addr(2) as usize;
                    let dst = self.addr(3) as usize;
                    let val = self.read(addr1) + self.read(addr2);
                    self.write(val, dst);
                    self.position += 4;
                }
                Self::MUL => {
                    let addr1 = self.addr(1) as usize;
                    let addr2 = self.addr(2) as usize;
                    let dst = self.addr(3) as usize;
                    let val = self.read(addr1) * self.read(addr2);
                    self.write(val, dst);
                    self.position += 4;
                }
                Self::INP => {
                    if self.input_queue.is_empty() {
                        return (StatusCode::InputRequest, 0);
                    }
                    let val = self.input_queue.pop_front().unwrap();
                    self.write(val, self.addr(1) as usize);
                    self.position += 2;
                }
                Self::OUT => {
                    let ret = self.read(self.addr(1) as usize);
                    self.position += 2;
                    self.response = (StatusCode::OutputDelivery, ret);
                    return self.response;
                }
                Self::TRU => {
                    let addr1 = self.addr(1) as usize;
                    if self.read(addr1) != 0 {
                        let addr2 = self.addr(2) as usize;
                        self.position = self.read(addr2) as usize;
                    } else {
                        self.position += 3;
                    }
                }
                Self::FAL => {
                    let addr1 = self.addr(1) as usize;
                    if self.read(addr1) == 0 {
                        let addr2 = self.addr(2) as usize;
                        self.position = self.read(addr2) as usize;
                    } else {
                        self.position += 3;
                    }
                }
                Self::LTN => {
                    let addr1 = self.addr(1) as usize;
                    let addr2 = self.addr(2) as usize;
                    let dst = self.addr(3) as usize;
                    let val = if self.read(addr1) < self.read(addr2) { 1 } else { 0 };
                    self.write(val, dst);
                    self.position += 4;
                }
                Self::EQL => {
                    let addr1 = self.addr(1) as usize;
                    let addr2 = self.addr(2) as usize;
                    let dst = self.addr(3) as usize;
                    let val = if self.read(addr1) == self.read(addr2) { 1 } else { 0 };
                    self.write(val, dst);
                    self.position += 4;
                }
                Self::SRB => {
                    let addr1 = self.addr(1) as usize;
                    self.relative_base += self.read(addr1);
                    self.position += 2;
                }
                Self::FIN => {
                    self.response.0 = StatusCode::Complete;
                    return self.response;
                }
                _ => return (StatusCode::InvalidCommand, self.read(self.position) % 100),
            }
        }
        return (StatusCode::EOF, 0);
    }
}
