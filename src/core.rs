use std::{
    fs::File,
    io::{self, Write},
    os::fd::FromRawFd,
};

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
            self.registers[idx as usize - 1]
        }
    }

    pub fn write(&mut self, idx: u8, value: i32) {
        if idx != 0 {
            self.registers[idx as usize - 1] = value
        };
    }
}

#[derive(Clone, Copy)]
union FpReg {
    single: f32,
    double: f64,
    u32: u32,
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

    pub fn read_u32(&self, idx: u8) -> u32 {
        unsafe { self.registers[idx as usize].u32 }
    }

    pub fn read_single(&self, idx: u8) -> f32 {
        unsafe { self.registers[idx as usize].single }
    }

    pub fn read_double(&self, idx: u8) -> f64 {
        unsafe { self.registers[idx as usize].double }
    }

    pub fn write_single(&mut self, idx: u8, value: f32) {
        self.registers[idx as usize].single = value;
    }

    pub fn write_double(&mut self, idx: u8, value: f64) {
        self.registers[idx as usize].double = value;
    }

    pub fn write_u32(&mut self, idx: u8, value: u32) {
        self.registers[idx as usize].u32 = value;
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

    fn load_buf(&self, idx: u32, len: u32) -> &[u8] {
        let (data, offset) = self.get_data(idx);
        return &data[offset as usize..(offset + len) as usize];
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
    fp_regfile: FpRegfile,
    gp_regfile: Regfile,
    debug: bool,
}

pub struct RunInfo {
    pub return_code: i32,
}

const SYSCALL_EXIT: i32 = 93;
const SYSCALL_NEWFSTAT: i32 = 80;
const SYSCALL_WRITE: i32 = 64;
const SYSCALL_READ: i32 = 63;

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
            fp_regfile: FpRegfile::new(),
            gp_regfile: Regfile::new(),
        }
    }

    pub fn read(&self, reg: Register) -> i32 {
        self.gp_regfile.read(reg.to_idx())
    }

    pub fn write(&mut self, reg: Register, value: i32) {
        self.gp_regfile.write(reg.to_idx(), value);
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
                eprintln!("pc: {:#x}: {:?}", pc, instr);
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
        let fp_reg = &mut self.fp_regfile;
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
            Instruction::flw { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<f32>(addr);
                fp_reg.write_single(rd, val);
            }
            Instruction::fld { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<f64>(addr);
                fp_reg.write_double(rd, val);
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
            Instruction::fsw { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = fp_reg.read_single(rs2);
                self.memory.store::<f32>(addr, val);
            }
            Instruction::fsd { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = fp_reg.read_double(rs2);
                self.memory.store::<f64>(addr, val);
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

            // m-extension
            Instruction::mul { rd, rs1, rs2 } => {
                let a = reg.read(rs1);
                let b = reg.read(rs2);
                reg.write(rd, a.wrapping_mul(b));
            }
            Instruction::mulh { rd, rs1, rs2 } => {
                let a = reg.read(rs1) as i64;
                let b = reg.read(rs2) as i64;
                reg.write(rd, (a.wrapping_mul(b) >> 32) as i32);
            }
            Instruction::mulhsu { rd, rs1, rs2 } => {
                let a = reg.read(rs1) as i64;
                let b = reg.read(rs2) as u32 as u64;
                let prod = (a as i128).wrapping_mul(b as i128);
                reg.write(rd, (prod >> 32) as i32);
            }
            Instruction::mulhu { rd, rs1, rs2 } => {
                let a = reg.read(rs1) as u32 as u64;
                let b = reg.read(rs2) as u32 as u64;
                reg.write(rd, (a.wrapping_mul(b) >> 32) as i32);
            }
            Instruction::div { rd, rs1, rs2 } => {
                let dividend = reg.read(rs1);
                let divisor = reg.read(rs2);
                reg.write(
                    rd,
                    if divisor == 0 {
                        -1
                    } else if dividend == i32::MIN && divisor == -1 {
                        dividend
                    } else {
                        dividend.wrapping_div(divisor)
                    },
                );
            }
            Instruction::divu { rd, rs1, rs2 } => {
                let dividend = reg.read(rs1) as u32;
                let divisor = reg.read(rs2) as u32;
                reg.write(
                    rd,
                    if divisor == 0 {
                        -1
                    } else {
                        (dividend / divisor) as i32
                    },
                );
            }
            Instruction::rem { rd, rs1, rs2 } => {
                let dividend = reg.read(rs1);
                let divisor = reg.read(rs2);
                reg.write(
                    rd,
                    if divisor == 0 {
                        dividend
                    } else if dividend == i32::MIN && divisor == -1 {
                        0
                    } else {
                        dividend.wrapping_rem(divisor)
                    },
                );
            }
            Instruction::remu { rd, rs1, rs2 } => {
                let dividend = reg.read(rs1) as u32;
                let divisor = reg.read(rs2) as u32;
                reg.write(
                    rd,
                    if divisor == 0 {
                        dividend as i32
                    } else {
                        (dividend % divisor) as i32
                    },
                );
            }

            // f/d arithmetic using fp_reg
            Instruction::fadd_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a + b);
            }
            Instruction::fsub_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a - b);
            }
            Instruction::fmul_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a * b);
            }
            Instruction::fmadd_s {
                rd,
                rs1,
                rs2,
                rs3,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                let c = fp_reg.read_single(rs3);
                fp_reg.write_single(rd, a * b + c);
            }
            Instruction::fmsub_s {
                rd,
                rs1,
                rs2,
                rs3,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                let c = fp_reg.read_single(rs3);
                fp_reg.write_single(rd, a * b - c);
            }
            Instruction::fmadd_d {
                rd,
                rs1,
                rs2,
                rs3,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                let c = fp_reg.read_double(rs3);
                fp_reg.write_double(rd, a * b + c);
            }
            Instruction::fmsub_d {
                rd,
                rs1,
                rs2,
                rs3,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                let c = fp_reg.read_double(rs3);
                fp_reg.write_double(rd, a * b - c);
            }
            Instruction::fdiv_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a / b);
            }
            Instruction::fsgnj_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.copysign(b));
            }
            Instruction::fsgnjn_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.copysign(-b));
            }
            Instruction::fsgnjx_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.copysign(a * b));
            }
            Instruction::fmin_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.min(b));
            }
            Instruction::fmax_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.max(b));
            }
            Instruction::fadd_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a + b);
            }
            Instruction::fsub_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a - b);
            }
            Instruction::fmul_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a * b);
            }
            Instruction::fdiv_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a / b);
            }
            Instruction::fsgnj_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.copysign(b));
            }
            Instruction::fsgnjn_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.copysign(-b));
            }
            Instruction::fsgnjx_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.copysign(a * b));
            }
            Instruction::fmin_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.min(b));
            }
            Instruction::fmax_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.max(b));
            }

            // fmv Instructions
            Instruction::fmv_s_w { rd, rs1 } => {
                let bits = fp_reg.read_u32(rs1);
                reg.write(rd, bits as i32);
            }
            Instruction::fmv_w_s { rd, rs1 } => {
                let bits = reg.read(rs1);
                fp_reg.write_u32(rd, bits as u32);
            }
            Instruction::fmv_x_d { rd, rs1 } => {
                panic!("not supported on rv32i");
                // let bits = fp_reg.read_u32(rs1).to_bits();
                // reg.write(rd, bits as u32; // rv32: lower 32 bits onl);
            }
            Instruction::fmv_d_x { rd, rs1 } => {
                panic!("not supported on rv32i");
                // let bits = reg.read(rs1) as u64;
                // fp_reg.write_double(rd, f64::from_bits(bits));
            }

            // fcvt Instructions
            Instruction::fcvt_s_w { rd, rs1 } => {
                let a = reg.read(rs1);
                fp_reg.write_single(rd, a as f32);
            }
            Instruction::fcvt_s_wu { rd, rs1 } => {
                let a = reg.read(rs1) as u32;
                fp_reg.write_single(rd, a as f32);
            }
            Instruction::fcvt_w_s { rd, rs1 } => {
                let f = fp_reg.read_single(rs1);
                reg.write(rd, f as i32);
            }
            Instruction::fcvt_wu_s { rd, rs1 } => {
                let f = fp_reg.read_single(rs1);
                reg.write(rd, f as u32 as i32);
            }
            Instruction::fcvt_d_w { rd, rs1 } => {
                let a = reg.read(rs1);
                fp_reg.write_double(rd, a as f64);
            }
            Instruction::fcvt_d_wu { rd, rs1 } => {
                let a = reg.read(rs1) as u32;
                fp_reg.write_double(rd, a as f64);
            }
            Instruction::fcvt_w_d { rd, rs1 } => {
                let d = fp_reg.read_double(rs1);
                reg.write(rd, d as i32);
            }
            Instruction::fcvt_wu_d { rd, rs1 } => {
                let d = fp_reg.read_double(rs1);
                reg.write(rd, d as u32 as i32);
            }
            Instruction::fcvt_s_d { rd, rs1 } => {
                let d = fp_reg.read_double(rs1);
                fp_reg.write_single(rd, d as f32);
            }
            Instruction::fcvt_d_s { rd, rs1 } => {
                let f = fp_reg.read_single(rs1);
                fp_reg.write_double(rd, f as f64);
            }

            // fp compare Instructions
            Instruction::feq_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                reg.write(rd, if a == b { 1 } else { 0 });
            }
            Instruction::flt_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                reg.write(rd, if a < b { 1 } else { 0 });
            }
            Instruction::fle_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                reg.write(rd, if a <= b { 1 } else { 0 });
            }
            Instruction::feq_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                reg.write(rd, if a == b { 1 } else { 0 });
            }
            Instruction::flt_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                reg.write(rd, if a < b { 1 } else { 0 });
            }
            Instruction::fle_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                reg.write(rd, if a <= b { 1 } else { 0 });
            }
            Instruction::fence { .. } => { /* no-op */ }
            Instruction::fence_i => { /* no-op */ }
            Instruction::ecall => {
                let syscall = self.read(Register::A(7));
                match syscall {
                    SYSCALL_EXIT => return ExecResult::Exit,
                    SYSCALL_WRITE => {
                        let fd = self.read(Register::A(0));
                        let buf = self.read(Register::A(1));
                        let count = self.read(Register::A(2));

                        let buf = self.memory.load_buf(buf as u32, count as u32);

                        // let mut f = unsafe { File::from_raw_fd(fd) };
                        // f.write_all().expect("write failed");

                        let count = io::stdout().write(buf).expect("write failed");
                        self.write(Register::A(0), count as i32);
                    }
                    _ => {} // _ => panic!("unknown syscall '{syscall}'"),
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
