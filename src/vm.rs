use crate::instructions::{Instruction, NUM_INSTRUCTIONS};

struct Memory {
    program: [u8; 256],
    ram: [u8; 256],
    vram: [u8; 256],
}

#[derive(Debug, Clone, Copy)]
enum SectionType {
    Program,
    Ram,
    VRam,
}

impl Memory {
    fn new() -> Self {
        Self {
            program: [0; 256],
            ram: [0; 256],
            vram: [0; 256],
        }
    }

    fn get_section(&self, mem: SectionType) -> &[u8; 256] {
        match mem {
            SectionType::Program => &self.program,
            SectionType::Ram => &self.ram,
            SectionType::VRam => &self.vram,
        }
    }

    fn get_section_mut(&mut self, mem: SectionType) -> &mut [u8; 256] {
        match mem {
            SectionType::Program => &mut self.program,
            SectionType::Ram => &mut self.ram,
            SectionType::VRam => &mut self.vram,
        }
    }

    fn read(&self, p: u8, mem: SectionType) -> u8 {
        self.get_section(mem)[p as usize]
    }

    fn write(&mut self, p: u8, val: u8, mem: SectionType) {
        self.get_section_mut(mem)[p as usize] = val
    }

    fn copy(&mut self, from_p: u8, to_p: u8, num: u8, mem: SectionType) -> Result<(), StepResult> {
        let Some(from_end_p) = from_p.checked_add(num - 1) else {
            return Err(StepResult::ReadOutOfBounds);
        };
        let Some(to_end_p) = to_p.checked_add(num - 1) else {
            return Err(StepResult::WriteOutOfBounds);
        };
        for (pf, pt) in (from_p..=from_end_p).zip(to_p..=to_end_p) {
            let val = self.read(pf, mem);
            self.write(pt, val, mem);
        }
        Ok(())
    }

    fn fill(&mut self, val: u8, mem: SectionType) {
        *self.get_section_mut(mem) = [val; 256];
    }
}

#[allow(non_camel_case_types)]
struct f8 {
    bits: u8,
}

const NUM_REGISTERS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub enum Register {
    // Arithmetic register
    A = 0,

    // Arithmetic register, handles reads and writes
    B = 1,

    // Instruction pointer
    IP = 2,

    // Input register, bits as pressed flags
    // Left arrow = first bit
    // Right arrow
    // Down arrow
    // Up arrow
    // Space
    // X
    // C
    // Esc = last bit
    INP = 3,
}

pub struct Machine {
    memory: Memory,
    registers: [u8; NUM_REGISTERS],
}

#[derive(Debug, Clone, Copy)]
pub enum StepResult {
    ReadOutOfBounds,
    WriteOutOfBounds,
    EndOfProgram,
    ArgOutOfBounds,
    UnknownOp(u8),
    Flush,
}

impl Machine {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: [0; NUM_REGISTERS],
        }
    }

    pub fn load_program(&mut self, program: [u8; 256]) {
        self.memory.program = program;
    }

    fn read_reg(&self, reg: Register) -> u8 {
        self.registers[reg as usize]
    }

    pub fn write_reg(&mut self, reg: Register, val: u8) {
        self.registers[reg as usize] = val;
    }

    fn step(&mut self) -> Option<StepResult> {
        let ip = self.read_reg(Register::IP);
        let ins = self.memory.read(ip, SectionType::Program);
        if ins >= NUM_INSTRUCTIONS {
            return Some(StepResult::UnknownOp(ins));
        }
        let op: Instruction = unsafe { std::mem::transmute(ins) };

        let num_args = op.num_args();
        if 255 - ip < num_args {
            return Some(StepResult::ArgOutOfBounds);
        }
        let args: Vec<u8> = if num_args == 0 {
            vec![]
        } else {
            (ip + 1..=ip + num_args)
                .map(|i| self.memory.read(i, SectionType::Program))
                .collect()
        };
        //dbg!(op);
        //dbg!(self.memory.read(0, SectionType::Ram));
        use Instruction::*;
        let a = self.read_reg(Register::A);
        let b = self.read_reg(Register::B);
        //dbg!(a, b);
        let mut flushed = false;
        match op {
            NoOp => (),
            Jump => {
                self.write_reg(Register::IP, args[0]);
                return None;
            }
            JumpINZ => {
                if b != 0 {
                    self.write_reg(Register::IP, args[0]);
                    return None;
                }
            }
            SetA => self.write_reg(Register::A, args[0]),
            SetB => self.write_reg(Register::B, args[0]),
            Swap => {
                self.write_reg(Register::A, b);
                self.write_reg(Register::B, a);
            }
            WriteRam => self.memory.write(args[0], b, SectionType::Ram),
            ReadRam => self.write_reg(Register::B, self.memory.read(args[0], SectionType::Ram)),
            WriteRamA => self.memory.write(a, b, SectionType::Ram),
            ReadRamA => self.write_reg(Register::B, self.memory.read(a, SectionType::Ram)),

            WriteVRam => self.memory.write(a, args[0], SectionType::VRam),
            ReadVRam => self.write_reg(Register::B, self.memory.read(a, SectionType::VRam)),
            ReadInp => self.write_reg(Register::B, self.read_reg(Register::INP) & args[0]),
            Add => self.write_reg(Register::B, b.overflowing_add(a).0),
            Mul => self.write_reg(Register::B, b.overflowing_mul(a).0),
            Mod => self.write_reg(Register::B, b % a),
            Or => self.write_reg(Register::B, b | a),
            And => self.write_reg(Register::B, b & a),
            Xor => self.write_reg(Register::B, b ^ a),
            Not => self.write_reg(Register::B, !b),
            Shl => self.write_reg(Register::B, b << args[0]),
            Shr => self.write_reg(Register::B, b >> args[0]),
            Increment => self.write_reg(Register::A, a.overflowing_add(1).0),
            Decrement => self.write_reg(Register::A, a.overflowing_sub(1).0),
            FillVRam => self.memory.fill(args[0], SectionType::VRam),
            Flush => flushed = true,
        }
        if let Some(new_ip) = ip.checked_add(1).and_then(|i| i.checked_add(op.num_args())) {
            self.write_reg(Register::IP, new_ip);
            if flushed {
                Some(StepResult::Flush)
            } else {
                None
            }
        } else {
            Some(StepResult::EndOfProgram)
        }
    }

    pub fn loop_till_flush(&mut self) -> Result<[u8; 256], ()> {
        loop {
            if let Some(out) = self.step() {
                return match out {
                    StepResult::Flush => Ok(self.memory.vram),
                    StepResult::EndOfProgram => Err(()),
                    _ => panic!("Runtime error: {out:?}"),
                };
            }
        }
    }
}
