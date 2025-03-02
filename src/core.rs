use core::{f32, slice};
use std::{
    fmt,
    fs::File,
    io::{self, Read, Write},
    marker::PhantomData,
    mem,
    ops::{Add, Range},
    os::fd::FromRawFd,
    ptr,
};

use crate::{
    instruction::Instruction,
    load::{LoadedElf, Segment},
};

pub trait IdxType: fmt::Debug + Copy + Add + Eq + Ord {
    fn as_usize(self) -> usize;
}

impl IdxType for u64 {
    #[inline(always)]
    fn as_usize(self) -> usize {
        self as _
    }
}

impl IdxType for u32 {
    #[inline(always)]
    fn as_usize(self) -> usize {
        self as _
    }
}

pub trait MemReader {
    type Idx: IdxType;

    // returning 'static is unimaginably unsafe
    unsafe fn get_buf(data: *mut u8, offset: Self::Idx, len: Self::Idx) -> &'static mut [u8] {
        let start = data.byte_add(offset.as_usize());
        slice::from_raw_parts_mut(start, len.as_usize())
    }

    unsafe fn read<T: Copy>(data: *const u8, offset: Self::Idx) -> T;
    unsafe fn write<T: Copy>(data: *mut u8, offset: Self::Idx, val: T);
}

pub struct AlignedMemReader<Idx: IdxType> {
    _phantom_data: PhantomData<Idx>,
}

impl<Idx: IdxType> MemReader for AlignedMemReader<Idx> {
    type Idx = Idx;

    #[inline(always)]
    unsafe fn read<T: Copy>(data: *const u8, offset: Self::Idx) -> T {
        unsafe { *data.byte_add(offset.as_usize()).cast::<T>() }
    }

    #[inline(always)]
    unsafe fn write<T: Copy>(data: *mut u8, offset: Idx, val: T) {
        unsafe { *data.byte_add(offset.as_usize()).cast::<T>() = val }
    }
}

pub struct UnalignedMemReader<Idx: IdxType> {
    _phantom_data: PhantomData<Idx>,
}

impl<Idx: IdxType> MemReader for UnalignedMemReader<Idx> {
    type Idx = Idx;

    #[inline(always)]
    unsafe fn read<T: Copy>(data: *const u8, offset: Self::Idx) -> T {
        unsafe {
            data.byte_add(offset.as_usize())
                .cast::<T>()
                .read_unaligned()
        }
    }

    #[inline(always)]
    unsafe fn write<T: Copy>(data: *mut u8, offset: Idx, val: T) {
        unsafe {
            data.byte_add(offset.as_usize())
                .cast::<T>()
                .write_unaligned(val)
        }
    }
}

struct Regfile {
    registers: [i32; 32],
}

impl Regfile {
    pub fn new() -> Self {
        Self {
            registers: [0xBEBE; 32],
        }
    }

    #[inline(always)]
    pub fn read(&self, idx: u8) -> i32 {
        if idx == 0 {
            0
        } else {
            unsafe { *self.registers.get_unchecked(idx as usize) }
        }
    }

    #[inline(always)]
    pub fn write(&mut self, idx: u8, value: i32) {
        if idx != 0 {
            unsafe { *self.registers.get_unchecked_mut(idx as usize) = value }
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

#[repr(u8)]
#[derive(Debug, Clone, Copy, Default)]
enum RoundingMode {
    #[default]
    RNE = 0b000,
    RTZ = 0b001,
    RDN = 0b010,
    RUP = 0b011,
    RMM = 0b100,
    // Reserved = 0b101,
    // Reserved = 0b110,
    DYN = 0b111,
}

impl TryFrom<i32> for RoundingMode {
    type Error = &'static str;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0b000 => Ok(Self::RNE),
            0b001 => Ok(Self::RTZ),
            0b010 => Ok(Self::RDN),
            0b011 => Ok(Self::RUP),
            0b100 => Ok(Self::RMM),
            0b111 => Ok(Self::DYN),
            _ => Err("bad rounding mode"),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct Fcsr {
    pub rm: RoundingMode,

    pub nv: bool,
    pub dz: bool,
    pub of: bool,
    pub uf: bool,
    pub nx: bool,
}

struct FpRegfile {
    registers: [FpReg; 32],
    fcsr: Fcsr,
}

impl FpRegfile {
    pub fn new() -> Self {
        Self {
            registers: [FpReg { u64: 0xBEBEBEBE }; 32],
            fcsr: Fcsr::default(),
        }
    }

    #[inline(always)]
    pub fn read_u32(&self, idx: u8) -> u32 {
        unsafe { self.registers.get_unchecked(idx as usize).u32 }
    }

    #[inline(always)]
    pub fn read_single(&self, idx: u8) -> f32 {
        unsafe { self.registers.get_unchecked(idx as usize).single }
    }

    #[inline(always)]
    pub fn read_double(&self, idx: u8) -> f64 {
        unsafe { self.registers.get_unchecked(idx as usize).double }
    }

    #[inline(always)]
    pub fn write_single(&mut self, idx: u8, value: f32) {
        unsafe {
            self.registers.get_unchecked_mut(idx as usize).single = value;
        }
    }

    #[inline(always)]
    pub fn write_double(&mut self, idx: u8, value: f64) {
        unsafe {
            self.registers.get_unchecked_mut(idx as usize).double = value;
        }
    }

    #[inline(always)]
    pub fn write_u32(&mut self, idx: u8, value: u32) {
        unsafe {
            self.registers.get_unchecked_mut(idx as usize).u32 = value;
        }
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

    Fa(usize),
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

            Register::Fa(i) => i as u8 + 10,
        }
    }
}

pub struct Memory<Reader: MemReader> {
    data_owner: Box<[u8]>,
    data: *mut u8,
    size: usize,

    elf: LoadedElf,

    _phantom_data: PhantomData<Reader>,
}

#[repr(C, align(16))]
struct Align16(u8);

impl<Reader: MemReader> Memory<Reader> {
    fn new(elf: LoadedElf, size: usize) -> Self {
        let mut data_owner = vec![0xBEu8; size].into_boxed_slice();

        let data;
        let size;
        unsafe {
            let (_pref, aligned, _suf) = data_owner.align_to_mut::<Align16>();

            data = aligned.as_mut_ptr() as *mut u8;
            size = std::mem::size_of_val(aligned);

            for seg in elf.segments.iter() {
                let dest = data.byte_add(seg.vaddr as usize);
                assert!(seg.vaddr as usize + seg.data.len() < size);
                dest.copy_from(seg.data.as_ptr(), seg.data.len());
            }
        }

        Self {
            elf,
            data_owner,
            data,
            size,
            _phantom_data: PhantomData,
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    // fn get_data(&self, idx: u32) -> (&[AlignedU8], u32) {
    //     match self.elf.find_segment(idx as u64) {
    //         Some(_) => panic!(""),
    //         // Some((segment, _, offset)) => (&segment.data, offset as u32),
    //         None => (&self.data, idx),
    //     }
    // }

    fn get_buf(&mut self, addr: Reader::Idx, len: Reader::Idx) -> &mut [u8] {
        assert!(
            addr.as_usize() + len.as_usize() <= self.size,
            "{addr:?} {len:?}"
        );

        // let (data, offset) = self.get_data(idx);
        let data = self.data;
        unsafe { Reader::get_buf(data, addr, len) }
    }

    fn load<T: Copy>(&self, addr: Reader::Idx) -> T {
        assert!(
            addr.as_usize() + mem::size_of::<T>() <= self.size,
            "addr={addr:?}, size={}, len={}",
            mem::size_of::<T>(),
            self.size
        );

        // let (data, offset) = self.get_data(idx);
        let data = self.data;
        unsafe { Reader::read(data, addr) }
    }

    fn store<T: Copy>(&self, addr: Reader::Idx, val: T) {
        assert!(
            addr.as_usize() + mem::size_of::<T>() <= self.size,
            "addr={addr:?}, size={}, len={}",
            mem::size_of::<T>(),
            self.size
        );

        // let (data, offset) = self.get_data(idx);
        let data = self.data;
        unsafe { Reader::write(data, addr, val) }
    }

    fn memset(&mut self, idx: i32, value: i32, length: i32) {
        unsafe {
            ptr::write_bytes(
                self.data.byte_add(idx as usize),
                value as u8,
                length as usize,
            );
        }
    }

    fn memcpy(&mut self, dest: i32, src: i32, length: i32) {
        unsafe {
            ptr::copy_nonoverlapping(
                self.data.byte_add(src as usize),
                self.data.byte_add(dest as usize),
                length as usize,
            );
        }
    }

    fn memmove(&mut self, dest: i32, src: i32, length: i32) {
        unsafe {
            ptr::copy(
                self.data.byte_add(src as usize),
                self.data.byte_add(dest as usize),
                length as usize,
            );
        }
    }
}

pub struct Core32<Reader: MemReader> {
    pc: u32,
    text: Segment,
    memory: Memory<Reader>,
    fp_regfile: FpRegfile,
    gp_regfile: Regfile,
    debug: bool,

    pub wk_memmove: u32,
    pub wk_memcpy: u32,
    pub wk_memset: u32,
    pub wk_cos: u32,
    pub wk_sin: u32,
}

pub struct RunInfo {
    pub return_code: i32,
}

const SYSCALL_EXIT: i32 = 93;
// const SYSCALL_NEWFSTAT: i32 = 80;
const SYSCALL_WRITE: i32 = 64;
const SYSCALL_READ: i32 = 63;
const SYSCALL_BRK: i32 = 214;

enum ExecResult {
    Continue,
    Jump(u32),
    Call(u32),
    Exit,
}

impl<Reader: MemReader<Idx = u32>> Core32<Reader> {
    pub fn new(elf: LoadedElf, entrypoint: Option<u64>, size: usize, debug: bool) -> Self {
        let (text, _start, pc_offset) = elf
            .find_segment(entrypoint.unwrap_or(elf.entrypoint))
            .expect("entrypoint not found!");

        Self {
            debug,
            pc: (text.vaddr + pc_offset as u64) as u32,
            text: text.clone(),
            fp_regfile: FpRegfile::new(),
            gp_regfile: Regfile::new(),

            wk_memmove: elf.wk_memmove,
            wk_memcpy: elf.wk_memcpy,
            wk_memset: elf.wk_memset,
            wk_cos: elf.wk_cos,
            wk_sin: elf.wk_sin,

            memory: Memory::new(elf, size),
        }
    }

    pub fn read(&self, reg: Register) -> i32 {
        self.gp_regfile.read(reg.to_idx())
    }

    pub fn write(&mut self, reg: Register, value: i32) {
        self.gp_regfile.write(reg.to_idx(), value);
    }

    #[cold]
    fn debug_print(&self, instr: &Instruction) {
        eprintln!("pc: {:#x}: {:?}", self.pc, instr);
    }

    #[cold]
    fn get_exit_info(&self) -> RunInfo {
        RunInfo {
            return_code: self.read(Register::A(0)),
        }
    }

    pub fn run(&mut self) -> RunInfo {
        let sp = (self.memory.size() as i32 - 128) & !0xF;
        self.write(Register::Sp, sp);

        let vaddr = self.text.vaddr as usize;
        let data = self.text.data.clone();

        let mut ins_cache = Vec::with_capacity((data.len() + 3) / 4);
        unsafe {
            let Range { mut start, end } = data.as_ptr_range();

            while start < end {
                let instr = *(start as *const u32);
                let instr = Instruction::decode(instr);

                ins_cache.push(instr);

                start = start.wrapping_add(4);
            }

            ins_cache.set_len(data.len() / 4);
        }

        let wk_memmove = self.wk_memmove;
        let wk_memcpy = self.wk_memcpy;
        let wk_memset = self.wk_memset;
        let wk_cos = self.wk_cos;
        let wk_sin = self.wk_sin;

        loop {
            let pc = self.pc;

            let pc = pc as usize;
            let rel_pc = pc - vaddr;
            // let instr = read_unaligned(&data, rel_pc);
            // let instr = Instruction::decode(u32::from_le_bytes(instr));
            let instr = unsafe { *ins_cache.get_unchecked(rel_pc / 4) };

            if self.debug {
                self.debug_print(&instr);
            }

            match self.exec(instr) {
                ExecResult::Jump(pc) => {
                    self.pc = pc;
                }
                ExecResult::Call(pc) => {
                    if self.pc == pc {
                        // loop
                        return RunInfo { return_code: 0 };
                    }

                    if pc == wk_memset {
                        let dst = self.read(Register::A(0));
                        let value = self.read(Register::A(1));
                        let count = self.read(Register::A(2));

                        self.memory.memset(dst, value, count);

                        self.pc = self.read(Register::Ra) as u32;
                    } else if pc == wk_memcpy {
                        let dst = self.read(Register::A(0));
                        let src = self.read(Register::A(1));
                        let count = self.read(Register::A(2));

                        self.memory.memcpy(dst, src, count);

                        self.pc = self.read(Register::Ra) as u32;
                    } else if pc == wk_memmove {
                        let dst = self.read(Register::A(0));
                        let src = self.read(Register::A(1));
                        let count = self.read(Register::A(2));

                        self.memory.memmove(dst, src, count);

                        self.pc = self.read(Register::Ra) as u32;
                    } else if pc == wk_cos {
                        let arg = self.fp_regfile.read_double(10);
                        self.fp_regfile.write_double(10, arg.cos());

                        self.pc = self.read(Register::Ra) as u32;
                    } else if pc == wk_sin {
                        let arg = self.fp_regfile.read_double(10);
                        self.fp_regfile.write_double(10, arg.sin());

                        self.pc = self.read(Register::Ra) as u32;
                    } else {
                        self.pc = pc;
                    }
                }
                ExecResult::Continue => self.pc += 4,
                ExecResult::Exit => return self.get_exit_info(),
            }
        }
    }

    fn exec(&mut self, instr: Instruction) -> ExecResult {
        let fp_reg = &mut self.fp_regfile;
        let reg = &mut self.gp_regfile;

        match instr {
            Instruction::Lui { rd, imm } => {
                reg.write(rd, imm);
            }
            Instruction::Auipc { rd, imm } => {
                reg.write(rd, (self.pc as i32).wrapping_add(imm));
            }
            Instruction::Jal { rd, imm } => {
                let ret = self.pc.wrapping_add(4);
                reg.write(rd, ret as i32);

                if rd == 1 {
                    return ExecResult::Call(self.pc.wrapping_add(imm as u32));
                } else {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Jalr { rd, rs1, imm } => {
                let ret = self.pc.wrapping_add(4);
                let target = (reg.read(rs1) as u32).wrapping_add(imm as u32) & !1;
                reg.write(rd, ret as i32);

                if rd == 1 {
                    return ExecResult::Call(target);
                } else {
                    return ExecResult::Jump(target);
                }
            }
            Instruction::Beq { rs1, rs2, imm } => {
                if reg.read(rs1) == reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Bne { rs1, rs2, imm } => {
                if reg.read(rs1) != reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Blt { rs1, rs2, imm } => {
                if reg.read(rs1) < reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Bge { rs1, rs2, imm } => {
                if reg.read(rs1) >= reg.read(rs2) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Bltu { rs1, rs2, imm } => {
                if (reg.read(rs1) as u32) < (reg.read(rs2) as u32) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Bgeu { rs1, rs2, imm } => {
                if (reg.read(rs1) as u32) >= (reg.read(rs2) as u32) {
                    return ExecResult::Jump(self.pc.wrapping_add(imm as u32));
                }
            }
            Instruction::Lb { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<i8>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::Lh { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<i16>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<u32>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::Lbu { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<u8>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::Lhu { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<u16>(addr) as i32;
                reg.write(rd, val);
            }
            Instruction::Flw { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<f32>(addr);
                fp_reg.write_single(rd, val);
            }
            Instruction::Fld { rd, rs1, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = self.memory.load::<f64>(addr);
                fp_reg.write_double(rd, val);
            }
            Instruction::Sb { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = reg.read(rs2) as u8;
                self.memory.store::<u8>(addr, val);
            }
            Instruction::Sh { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = reg.read(rs2) as u16;
                self.memory.store::<u16>(addr, val);
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = reg.read(rs2) as u32;
                self.memory.store::<u32>(addr, val);
            }
            Instruction::Fsw { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = fp_reg.read_single(rs2);
                self.memory.store::<f32>(addr, val);
            }
            Instruction::Fsd { rs1, rs2, imm } => {
                let addr = (reg.read(rs1) as u32).wrapping_add(imm as u32);
                let val = fp_reg.read_double(rs2);
                self.memory.store::<f64>(addr, val);
            }
            Instruction::Addi { rd, rs1, imm } => {
                let res = reg.read(rs1).wrapping_add(imm);
                reg.write(rd, res);
            }
            Instruction::Slti { rd, rs1, imm } => {
                let res = if reg.read(rs1) < imm { 1 } else { 0 };
                reg.write(rd, res);
            }
            Instruction::Sltiu { rd, rs1, imm } => {
                let res = if (reg.read(rs1) as u32) < (imm as u32) {
                    1
                } else {
                    0
                };
                reg.write(rd, res);
            }
            Instruction::Xori { rd, rs1, imm } => {
                let res = reg.read(rs1) ^ imm;
                reg.write(rd, res);
            }
            Instruction::Ori { rd, rs1, imm } => {
                let res = reg.read(rs1) | imm;
                reg.write(rd, res);
            }
            Instruction::Andi { rd, rs1, imm } => {
                let res = reg.read(rs1) & imm;
                reg.write(rd, res);
            }
            Instruction::Slli { rd, rs1, shamt } => {
                let res = (reg.read(rs1) as u32) << shamt;
                reg.write(rd, res as i32);
            }
            Instruction::Srli { rd, rs1, shamt } => {
                let res = (reg.read(rs1) as u32) >> shamt;
                reg.write(rd, res as i32);
            }
            Instruction::Srai { rd, rs1, shamt } => {
                let res = reg.read(rs1) >> shamt;
                reg.write(rd, res);
            }
            Instruction::Add { rd, rs1, rs2 } => {
                let res = reg.read(rs1).wrapping_add(reg.read(rs2));
                reg.write(rd, res);
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let res = reg.read(rs1).wrapping_sub(reg.read(rs2));
                reg.write(rd, res);
            }
            Instruction::Sll { rd, rs1, rs2 } => {
                let sh = reg.read(rs2) & 0x1f;
                let res = (reg.read(rs1) as u32) << sh;
                reg.write(rd, res as i32);
            }
            Instruction::Slt { rd, rs1, rs2 } => {
                let res = if reg.read(rs1) < reg.read(rs2) { 1 } else { 0 };
                reg.write(rd, res);
            }
            Instruction::Sltu { rd, rs1, rs2 } => {
                let res = if (reg.read(rs1) as u32) < (reg.read(rs2) as u32) {
                    1
                } else {
                    0
                };
                reg.write(rd, res);
            }
            Instruction::Xor { rd, rs1, rs2 } => {
                let res = reg.read(rs1) ^ reg.read(rs2);
                reg.write(rd, res);
            }
            Instruction::Srl { rd, rs1, rs2 } => {
                let sh = reg.read(rs2) & 0x1f;
                let res = (reg.read(rs1) as u32) >> sh;
                reg.write(rd, res as i32);
            }
            Instruction::Sra { rd, rs1, rs2 } => {
                let sh = reg.read(rs2) & 0x1f;
                let res = reg.read(rs1) >> sh;
                reg.write(rd, res);
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let res = reg.read(rs1) | reg.read(rs2);
                reg.write(rd, res);
            }
            Instruction::And { rd, rs1, rs2 } => {
                let res = reg.read(rs1) & reg.read(rs2);
                reg.write(rd, res);
            }

            // m-extension
            Instruction::Mul { rd, rs1, rs2 } => {
                let a = reg.read(rs1);
                let b = reg.read(rs2);
                reg.write(rd, a.wrapping_mul(b));
            }
            Instruction::Mulh { rd, rs1, rs2 } => {
                let a = reg.read(rs1) as i64;
                let b = reg.read(rs2) as i64;
                reg.write(rd, (a.wrapping_mul(b) >> 32) as i32);
            }
            Instruction::Mulhsu { rd, rs1, rs2 } => {
                let a = reg.read(rs1) as i64;
                let b = reg.read(rs2) as u32 as u64;
                let prod = (a as i128).wrapping_mul(b as i128);
                reg.write(rd, (prod >> 32) as i32);
            }
            Instruction::Mulhu { rd, rs1, rs2 } => {
                let a = reg.read(rs1) as u32 as u64;
                let b = reg.read(rs2) as u32 as u64;
                reg.write(rd, (a.wrapping_mul(b) >> 32) as i32);
            }
            Instruction::Div { rd, rs1, rs2 } => {
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
            Instruction::Divu { rd, rs1, rs2 } => {
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
            Instruction::Rem { rd, rs1, rs2 } => {
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
            Instruction::Remu { rd, rs1, rs2 } => {
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
            Instruction::Fadd_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a + b);
            }

            Instruction::Fclass_s { rd, rs1 } => {
                let a = fp_reg.read_single(rs1);

                let bits = a.to_bits();
                let sign = bits >> 31;
                let exp = (bits >> 23) & 0xff;
                let frac = bits & 0x7fffff;
                let mut mask = 0;
                if exp == 0xff {
                    if frac == 0 {
                        if sign != 0 {
                            mask |= 1 << 0
                        } else {
                            mask |= 1 << 7
                        }
                    } else if (frac & (1 << 22)) == 0 {
                        mask |= 1 << 8
                    } else {
                        mask |= 1 << 9
                    }
                } else if exp == 0 {
                    if frac == 0 {
                        if sign != 0 {
                            mask |= 1 << 3
                        } else {
                            mask |= 1 << 4
                        }
                    } else if sign != 0 {
                        mask |= 1 << 2
                    } else {
                        mask |= 1 << 5
                    }
                } else if sign != 0 {
                    mask |= 1 << 1
                } else {
                    mask |= 1 << 6
                }

                reg.write(rd, mask);
            }
            Instruction::Fclass_d { rd, rs1 } => {
                let a = fp_reg.read_double(rs1);

                let bits = a.to_bits();
                let sign = bits >> 63;
                let exp = (bits >> 52) & 0x7ff;
                let frac = bits & 0xfffffffffffff;
                let mut mask = 0;
                if exp == 0x7ff {
                    if frac == 0 {
                        if sign != 0 {
                            mask |= 1 << 0
                        } else {
                            mask |= 1 << 7
                        }
                    } else if (frac & (1 << 51)) == 0 {
                        mask |= 1 << 8
                    } else {
                        mask |= 1 << 9
                    }
                } else if exp == 0 {
                    if frac == 0 {
                        if sign != 0 {
                            mask |= 1 << 3
                        } else {
                            mask |= 1 << 4
                        }
                    } else if sign != 0 {
                        mask |= 1 << 2
                    } else {
                        mask |= 1 << 5
                    }
                } else if sign != 0 {
                    mask |= 1 << 1
                } else {
                    mask |= 1 << 6
                }

                reg.write(rd, mask);
            }
            Instruction::Fsqrt_s { rd, rs1, rm: _ } => {
                let a = fp_reg.read_single(rs1);
                fp_reg.write_single(rd, a.sqrt());
            }
            Instruction::Fsqrt_d { rd, rs1, rm: _ } => {
                let a = fp_reg.read_double(rs1);
                fp_reg.write_double(rd, a.sqrt());
            }
            Instruction::Fsub_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a - b);
            }
            Instruction::Fmul_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a * b);
            }
            Instruction::Fmadd_s {
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
            Instruction::Fmsub_s {
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
            Instruction::Fmadd_d {
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
            Instruction::Fmsub_d {
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
            Instruction::Fnmadd_s {
                rd,
                rs1,
                rs2,
                rs3,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                let c = fp_reg.read_single(rs3);
                fp_reg.write_single(rd, -(a * b) + c);
            }
            Instruction::Fnmsub_s {
                rd,
                rs1,
                rs2,
                rs3,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                let c = fp_reg.read_single(rs3);
                fp_reg.write_single(rd, -(a * b) - c);
            }
            Instruction::Fnmadd_d {
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
            Instruction::Fnmsub_d {
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

            Instruction::Fdiv_s {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a / b);
            }
            Instruction::Fsgnj_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.copysign(b));
            }
            Instruction::Fsgnjn_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.copysign(-b));
            }
            Instruction::Fsgnjx_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.copysign(a * b));
            }
            Instruction::Fmin_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.min(b));
            }
            Instruction::Fmax_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                fp_reg.write_single(rd, a.max(b));
            }
            Instruction::Fadd_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a + b);
            }
            Instruction::Fsub_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a - b);
            }
            Instruction::Fmul_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a * b);
            }
            Instruction::Fdiv_d {
                rd,
                rs1,
                rs2,
                rm: _,
            } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a / b);
            }
            Instruction::Fsgnj_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.copysign(b));
            }
            Instruction::Fsgnjn_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.copysign(-b));
            }
            Instruction::Fsgnjx_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.copysign(a * b));
            }
            Instruction::Fmin_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.min(b));
            }
            Instruction::Fmax_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                fp_reg.write_double(rd, a.max(b));
            }

            // fmv Instructions
            Instruction::Fmv_s_w { rd, rs1 } => {
                let bits = fp_reg.read_u32(rs1);
                reg.write(rd, bits as i32);
            }
            Instruction::Fmv_w_s { rd, rs1 } => {
                let bits = reg.read(rs1);
                fp_reg.write_u32(rd, bits as u32);
            }
            Instruction::Fmv_x_d { rd: _rd, rs1: _rs1 } => {
                panic!("not supported on rv32i");
                // let bits = fp_reg.read_u32(rs1).to_bits();
                // reg.write(rd, bits as u32; // rv32: lower 32 bits onl);
            }
            Instruction::Fmv_d_x { rd: _rd, rs1: _rs1 } => {
                panic!("not supported on rv32i");
                // let bits = reg.read(rs1) as u64;
                // fp_reg.write_double(rd, f64::from_bits(bits));
            }

            // fcvt Instructions
            Instruction::Fcvt_s_w { rd, rs1 } => {
                let a = reg.read(rs1);
                fp_reg.write_single(rd, a as f32);
            }
            Instruction::Fcvt_s_wu { rd, rs1 } => {
                let a = reg.read(rs1) as u32;
                fp_reg.write_single(rd, a as f32);
            }
            Instruction::Fcvt_w_s { rd, rs1 } => {
                let f = fp_reg.read_single(rs1);
                reg.write(rd, f as i32);
            }
            Instruction::Fcvt_wu_s { rd, rs1 } => {
                let f = fp_reg.read_single(rs1);
                reg.write(rd, f as u32 as i32);
            }
            Instruction::Fcvt_d_w { rd, rs1 } => {
                let a = reg.read(rs1);
                fp_reg.write_double(rd, a as f64);
            }
            Instruction::Fcvt_d_wu { rd, rs1 } => {
                let a = reg.read(rs1) as u32;
                fp_reg.write_double(rd, a as f64);
            }
            Instruction::Fcvt_w_d { rd, rs1 } => {
                let d = fp_reg.read_double(rs1);
                reg.write(rd, d as i32);
            }
            Instruction::Fcvt_wu_d { rd, rs1 } => {
                let d = fp_reg.read_double(rs1);
                reg.write(rd, d as u32 as i32);
            }
            Instruction::Fcvt_s_d { rd, rs1 } => {
                let d = fp_reg.read_double(rs1);
                fp_reg.write_single(rd, d as f32);
            }
            Instruction::Fcvt_d_s { rd, rs1 } => {
                let f = fp_reg.read_single(rs1);
                fp_reg.write_double(rd, f as f64);
            }

            // fp compare Instructions
            Instruction::Feq_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                reg.write(rd, if a == b { 1 } else { 0 });
            }
            Instruction::Flt_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                reg.write(rd, if a < b { 1 } else { 0 });
            }
            Instruction::Fle_s { rd, rs1, rs2 } => {
                let a = fp_reg.read_single(rs1);
                let b = fp_reg.read_single(rs2);
                reg.write(rd, if a <= b { 1 } else { 0 });
            }
            Instruction::Feq_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                reg.write(rd, if a == b { 1 } else { 0 });
            }
            Instruction::Flt_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                reg.write(rd, if a < b { 1 } else { 0 });
            }
            Instruction::Fle_d { rd, rs1, rs2 } => {
                let a = fp_reg.read_double(rs1);
                let b = fp_reg.read_double(rs2);
                reg.write(rd, if a <= b { 1 } else { 0 });
            }
            Instruction::Fence { .. } => { /* no-op */ }
            Instruction::Fence_i => { /* no-op */ }
            Instruction::Ecall => {
                let syscall = self.read(Register::A(7));
                match syscall {
                    SYSCALL_EXIT => return ExecResult::Exit,
                    SYSCALL_WRITE => {
                        let fd = self.read(Register::A(0));
                        let buf = self.read(Register::A(1));
                        let count = self.read(Register::A(2));

                        let buf = self.memory.get_buf(buf as u32, count as u32);

                        let mut f = unsafe { File::from_raw_fd(fd) };
                        let count = f.write(buf).expect("write failed");

                        self.write(Register::A(0), count as i32);

                        // IMPORTANT: don't close the file
                        mem::forget(f);
                    }
                    SYSCALL_READ => {
                        let fd = self.read(Register::A(0));
                        let buf = self.read(Register::A(1));
                        let count = self.read(Register::A(2));

                        let buf = self.memory.get_buf(buf as u32, count as u32);

                        let mut f = unsafe { File::from_raw_fd(fd) };
                        let count = f.read(buf).expect("write failed");

                        self.write(Register::A(0), count as i32);

                        // IMPORTANT: don't close the file
                        mem::forget(f);
                    }
                    SYSCALL_BRK => {
                        let p = self.read(Register::A(0));
                        eprintln!("brk to {:#x}", p);
                    }
                    _ => eprintln!("unknown syscall '{syscall}'"),
                    // _ => panic!("unknown syscall '{syscall}'"),
                }
            }
            Instruction::Frrm { rd } => {
                let rm = fp_reg.fcsr.rm;
                reg.write(rd, rm as i32);
            }
            Instruction::Fsrm { rd, rs1 } => {
                let rm = fp_reg.fcsr.rm;
                reg.write(rd, rm as i32);

                let new_rm = reg.read(rs1);
                fp_reg.fcsr.rm = new_rm.try_into().expect("bad rounding mode");
            }
            Instruction::Ebreak => {
                todo!("ebreak encountered");
            }

            Instruction::Unknown(val) => {
                panic!("unknown instruction {val:#x} at pc {:#x}!", self.pc);
            }
        }
        ExecResult::Continue
    }
}
