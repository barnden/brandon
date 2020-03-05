pub struct Instruction(pub u32);

impl Instruction {
    pub fn as_u32(&self) -> u32 {
        *&self.0
    }

    pub fn as_hex(&self) -> String {
        format!("{:#06X}", *&self.0)
    }

    pub fn as_bin(&self) -> String {
        format!("{:#29b}", *&self.0)
    }

    pub fn get_opcode(&self) -> u8 {
        (&self.0 >> 24) as u8
    }

    pub fn get_ophex(&self) -> String {
        format!("{:#02X}", self.get_opcode())
    }

    pub fn get_op(&self) -> Option<Opcode> {
        num::FromPrimitive::from_u8(self.get_opcode())
    }

    pub fn get_bit(&self, pos: u8) -> bool {
        ((&self.0 >> pos) & 1) == 1
    }

    pub fn mask(&self, mask: u32) -> u32 {
        &self.0 & mask
    }

    pub fn smask(&self, shift: i8, mask: u32) -> u32 {
        self.shift(shift) & mask
    }

    pub fn shift(&self, shift: i8) -> u32 {
        if shift < 0 {
            (&self.0 << (shift.abs() as u32))
        } else {
            &self.0 >> shift as u32
        }
    }
}

#[derive(FromPrimitive,PartialEq)]
pub enum Opcode {
    MOV = 0,
    MEX,
    MRX,
    MMX,
    NIL,
    LFX,
    SWX,
    JMP,
    JSR,
    CMP,
    CMZ,
    ARG,
    ADD,
    SUB,
    MUL,
    DIV,
    AND,
    NOT,
    CAL,
    JPA,
    FLX,
    ILX
}