use std::str::FromStr;

pub const NUM_INSTRUCTIONS: u8 = 26;

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    NoOp = 0,
    Jump,      // Set IP
    JumpINZ,   // Jump if B not equal to zero
    SetA,      // Set A to arg
    SetB,      // Set B to arg
    Swap,      // Swap A and B
    WriteRam,  // Write B to RAM at addr arg
    ReadRam,   // Read RAM at addr arg to B
    WriteRamA, // Write B to RAM at addr A
    ReadRamA,  // Read RAM at addr A to B
    WriteVRam, // Write B to VRAM at addr A
    ReadVRam,  // Read VRAM at addr A to B
    ReadInp,   // And INP with arg, store in B
    Add,       // Add A to B
    Mul,       // Mul B by A
    Mod,       // Mod B by A
    Or,        // Bitwise or A and B, store in B
    And,       // Bitwise and A and B, store in B
    Xor,       // Bitwise xor A and B, store in B
    Not,       // Negate B, store in B
    Shl,       // Shift B left by arg
    Shr,       // Shift B right by arg
    Increment, // Increment A by 1
    Decrement, // Decrement A by 1
    FillVRam,  // Fill vram with B
    Flush,     // Flush VRAM to screen, and wait
}

impl Instruction {
    pub fn num_args(self) -> u8 {
        use Instruction::*;
        match self {
            NoOp => 0,
            Jump => 1,
            JumpINZ => 1,
            SetA => 1,
            SetB => 1,
            Swap => 0,
            WriteRam => 1,
            ReadRam => 1,
            WriteRamA => 0,
            ReadRamA => 0,
            WriteVRam => 1,
            ReadVRam => 0,
            ReadInp => 1,
            Add => 0,
            Mul => 0,
            Mod => 0,
            Or => 0,
            And => 0,
            Xor => 0,
            Not => 0,
            Shl => 1,
            Shr => 1,
            Increment => 0,
            Decrement => 0,
            FillVRam => 1,
            Flush => 0,
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Instruction::*;
        Ok(match s {
            "noop" => NoOp,
            "jmp" => Jump,
            "jnz" => JumpINZ,
            "sta" => SetA,
            "stb" => SetB,
            "swp" => Swap,
            "mvr" => WriteRam,
            "rdr" => ReadRam,
            "mva" => WriteRamA,
            "rda" => ReadRamA,
            "mvv" => WriteVRam,
            "rev" => ReadVRam,
            "rdi" => ReadInp,
            "add" => Add,
            "mul" => Mul,
            "mod" => Mod,
            "or" => Or,
            "and" => And,
            "xor" => Xor,
            "not" => Not,
            "shl" => Shl,
            "shr" => Shr,
            "inc" => Increment,
            "dec" => Decrement,
            "flv" => FillVRam,
            "fsh" => Flush,

            _ => return Err(()),
        })
    }
}
