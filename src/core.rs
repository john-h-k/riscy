use std::ptr::read_unaligned;

use crate::{
    instruction::Instruction,
    load::{LoadedElf, Segment},
};

fn read_unaligned<T: Copy>(data: &[u8]) -> T {
    unsafe { (data.as_ptr() as *const T).read_unaligned() }
}

fn write_unaligned<T: Copy>(data: &[u8], value: T) {
    unsafe { (data.as_ptr() as *mut T).write_unaligned(value) }
}

struct Regfile {
    registers: [i32; 31],
}

impl Regfile {
    pub fn new() -> Self {
        Self {
            registers: [0xBEBE; 31],
        }
    }

    pub fn read(&self, idx: u8) -> i32 {
        if idx == 0 {
            0
        } else {
            self.registers[idx as usize]
        }
    }

    pub fn write(&mut self, idx: u8, value: i32) {
        if idx != 0 {
            self.registers[idx as usize] = value
        };
    }
}

#[derive(Clone, Copy)]
union FpReg {
    single: f32,
    double: f64,
    u64: u64,
}

struct FpRegfile {
    registers: [FpReg; 32],
}

impl FpRegfile {
    pub fn new() -> Self {
        Self {
            registers: [FpReg { u64: 0xBEBEBEBE }; 32],
        }
    }

    pub fn read_single(&self, idx: u8) -> f32 {
        unsafe { self.registers[idx as usize].single }
    }

    pub fn read_double(&self, idx: u8) -> f64 {
        unsafe { self.registers[idx as usize].double }
    }

    pub fn write(&mut self, idx: u8, value: i32) {
        if idx != 0 {
            self.registers[idx as usize] = value
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Register {
    Zero,     // x0
    Ra,       // x1
    Sp,       // x2
    Gp,       // x3
    Tp,       // x4
    T(usize), // t registers: t0-t2 map to x5-x7, t3-t6 map to x28-x31
    S(usize), // s registers: s0 maps to x8, s1 to x9, s2-s11 to x18-x27
    A(usize), // a registers: a0-a7 map to x10-x17
}

impl Register {
    pub fn to_idx(self) -> u8 {
        match self {
            Register::Zero => 0,
            Register::Ra => 1,
            Register::Sp => 2,
            Register::Gp => 3,
            Register::Tp => 4,
            Register::T(i) => match i {
                0..=2 => i as u8 + 5,        // t0 => 5, t1 => 6, t2 => 7
                3..=6 => (i as u8 - 3) + 28, // t3 => 28, ... t6 => 31
                _ => unreachable!("invalid t register index"),
            },
            Register::S(i) => match i {
                0 => 8,                 // s0/fp => 8
                1 => 9,                 // s1 => 9
                2..=11 => i as u8 + 16, // s2 => 18, ... s11 => 27
                _ => unreachable!("invalid s register index"),
            },
            Register::A(i) => {
                if i < 8 {
                    i as u8 + 10 // a0 => 10, ... a7 => 17
                } else {
                    unreachable!("invalid a register index")
                }
            }
        }
    }
}

pub struct Memory {
    data: [u8; 0xFFFF],
    elf: LoadedElf,
}

impl Memory {
    fn new(elf: LoadedElf) -> Self {
        Self {
            elf,
            data: [0xBE; 0xFFFF],
        }
    }

    fn get_data(&self, idx: u32) -> (&[u8], u32) {
        match self.elf.find_segment(idx as u64) {
            Some((segment, _, offset)) => (&segment.data, offset as u32),
            None => (&self.data, idx),
        }
    }

    fn load<T: Copy>(&self, idx: u32) -> T {
        let (data, offset) = self.get_data(idx);
        read_unaligned(&data[offset as usize..])
    }

    fn store<T: Copy>(&self, idx: u32, value: T) {
        let (data, offset) = self.get_data(idx);
        write_unaligned(&data[offset as usize..], value);
    }
}

pub struct Core {
    pc: u32,
    text: Segment,
    memory: Memory,
    gp_regfile: Regfile,
    debug: bool,
}

pub struct RunInfo {
    pub return_code: i32,
}

#[repr(u32)]
enum SysCall {
    Exit = 93,
}

enum ExecResult {
    Continue,
    Jump(u32),
    Exit,
}

impl Core {
    pub fn new(elf: LoadedElf, entrypoint: Option<u64>, debug: bool) -> Self {
        let (text, _start, pc_offset) = elf
            .find_segment(entrypoint.unwrap_or(elf.entrypoint))
            .expect("entrypoint not found!");

        Self {
            debug,
            pc: (text.vaddr + pc_offset as u64) as u32,
            text: text.clone(),
            memory: Memory::new(elf),
            gp_regfile: Regfile::new(),
        }
    }

    pub fn read(&self, reg: Register) -> i32 {
        self.gp_regfile.read(reg.to_idx())
    }

    pub fn run(&mut self) -> RunInfo {
        loop {
            let pc = self.pc as usize;

            if pc == 0xBEBE {
                return RunInfo {
                    return_code: self.read(Register::A(0)),
                };
            }

            let rel_pc = pc - self.text.vaddr as usize;
            let instr = self.text.data[rel_pc..rel_pc + 4].try_into().unwrap();
            let instr = Instruction::decode(u32::from_le_bytes(instr));

            if self.debug {
                eprintln!("pc: {:#x}: {:?}", self.text.vaddr as usize + pc, instr);
            }

            match self.exec(instr) {
                ExecResult::Jump(pc) => {
                    if self.pc == pc {
                        eprintln!("Entered infinite loop");
                        return RunInfo { return_code: 0 };
                    }

                    self.pc = pc;
                }
                ExecResult::Continue => self.pc += 4,
                ExecResult::Exit => {
                    return RunInfo {
                        return_code: self.read(Register::A(0)),
                    }
                }
            }
        }
    }

    fn exec(&mut self, instr: Instruction) -> ExecResult {
        let reg = &mut self.gp_regfile;

        match instr {
            Instruction::lui { rd, imm } => {
                reg.write(rd, imm);
            }
            Instruction::auipc { rd, imm } => {
                reg.write(rd, (self.pc as i32).wrapping_add(imm));
            }
            Instruction::jal { rd, imm } => {
                let ret = self.pc.wrapping_add(4);
                reg.write(rd, ret as i32);
                return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
            }
            Instruction::jalr { rd, rs1, imm } => {
                let ret = self.pc.wrapping_add(4);
                let target = (reg.read(rs1) as u32).wrapping_add(imm as u32) & !1;
                reg.write(rd, ret as i32);
                return ExecResult::Jump(target);
            }
            Instruction::beq { rs1, rs2, imm } => {
                if reg.read(rs1) == reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::bne { rs1, rs2, imm } => {
                if reg.read(rs1) != reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::blt { rs1, rs2, imm } => {
                if reg.read(rs1) < reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::bge { rs1, rs2, imm } => {
                if reg.read(rs1) >= reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::bltu { rs1, rs2, imm } => {
                if (reg.read(rs1) as u32) < (reg.read(rs2) as u32) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::bgeu { rs1, rs2, imm } => {
                if (reg.read(rs1) as u32) >= (reg.read(rs2) as u32) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::lb { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<i8>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::lh { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<i16>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::lw { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<u32>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::lbu { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<u8>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::lhu { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<u16>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::sb { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = reg.read(rs2) as u8;
                self.memory.store::<u8>(addr, val);
            }
            Instruction::sh { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = reg.read(rs2) as u16;
                self.memory.store::<u16>(addr, val);
            }
            Instruction::sw { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = reg.read(rs2) as u32;
                self.memory.store::<u32>(addr, val);
            }
            Instruction::addi { rd, rs1, imm } => {
                let res = reg.read(rs1).wrapping_add(imm);
                reg.write(rd, res);
            }
            Instruction::slti { rd, rs1, imm } => {
                let res = if reg.read(rs1) < imm { 1 } else { 0 };
                reg.write(rd, res);
            }
            Instruction::sltiu { rd, rs1, imm } => {
                let res = if (reg.read(rs1) as u32) < (imm as u32) {
                    1
                } else {
                    0
                };
                reg.write(rd, res);
            }
            Instruction::xori { rd, rs1, imm } => {
                let res = reg.read(rs1) ^ imm;
                reg.write(rd, res);
            }
            Instruction::ori { rd, rs1, imm } => {
                let res = reg.read(rs1) | imm;
                reg.write(rd, res);
            }
            Instruction::andi { rd, rs1, imm } => {
                let res = reg.read(rs1) & imm;
                reg.write(rd, res);
            }
            Instruction::slli { rd, rs1, shamt } => {
                let res = (reg.read(rs1) as u32) << shamt;
                reg.write(rd, res as i32);
            }
            Instruction::srli { rd, rs1, shamt } => {
                let res = (reg.read(rs1) as u32) >> shamt;
                reg.write(rd, res as i32);
            }
            Instruction::srai { rd, rs1, shamt } => {
                let res = reg.read(rs1) >> shamt;
                reg.write(rd, res);
            }
            Instruction::add { rd, rs1, rs2 } => {
                let res = reg.read(rs1).wrapping_add(reg.read(rs2));
                reg.write(rd, res);
            }
            Instruction::sub { rd, rs1, rs2 } => {
                let res = reg.read(rs1).wrapping_sub(reg.read(rs2));
                reg.write(rd, res);
            }
            Instruction::sll { rd, rs1, rs2 } => {
                let sh = reg.read(rs2) & 0x1f;
                let res = (reg.read(rs1) as u32) << sh;
                reg.write(rd, res as i32);
            }
            Instruction::slt { rd, rs1, rs2 } => {
                let res = if reg.read(rs1) < reg.read(rs2) { 1 } else { 0 };
                reg.write(rd, res);
            }
            Instruction::sltu { rd, rs1, rs2 } => {
                let res = if (reg.read(rs1) as u32) < (reg.read(rs2) as u32) {
                    1
                } else {
                    0
                };
                reg.write(rd, res);
            }
            Instruction::xor { rd, rs1, rs2 } => {
                let res = reg.read(rs1) ^ reg.read(rs2);
                reg.write(rd, res);
            }
            Instruction::srl { rd, rs1, rs2 } => {
                let sh = reg.read(rs2) & 0x1f;
                let res = (reg.read(rs1) as u32) >> sh;
                reg.write(rd, res as i32);
            }
            Instruction::sra { rd, rs1, rs2 } => {
                let sh = reg.read(rs2) & 0x1f;
                let res = reg.read(rs1) >> sh;
                reg.write(rd, res);
            }
            Instruction::or { rd, rs1, rs2 } => {
                let res = reg.read(rs1) | reg.read(rs2);
                reg.write(rd, res);
            }
            Instruction::and { rd, rs1, rs2 } => {
                let res = reg.read(rs1) & reg.read(rs2);
                reg.write(rd, res);
            }
            Instruction::fence { .. } => { /* no-op */ }
            Instruction::fence_i => { /* no-op */ }
            Instruction::ecall => {
                if self.read(Register::A(7)) == SysCall::Exit as i32 {
                    return ExecResult::Exit;
                }
            }
            Instruction::ebreak => {
                todo!("ebreak encountered");
            }
            Instruction::unknown(val) => {
                panic!("unknown instruction!");
            }
        }
        ExecResult::Continue
    }
}
