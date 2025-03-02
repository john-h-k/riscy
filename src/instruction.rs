#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Unknown(u32),

    Lui {
        rd: u8,
        imm: i32,
    },
    Auipc {
        rd: u8,
        imm: i32,
    },
    Jal {
        rd: u8,
        imm: i32,
    },
    Jalr {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Beq {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Bne {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Blt {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Bge {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Bltu {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Bgeu {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Lb {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Lh {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Lw {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Lbu {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Lhu {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Sb {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Sh {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Sw {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Addi {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Slti {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Sltiu {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Xori {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Ori {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Andi {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Slli {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    Srli {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    Srai {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    Add {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sub {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sll {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Slt {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sltu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Xor {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Srl {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Sra {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Or {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    And {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fence {
        pred: u8,
        succ: u8,
    },

    FenceI,

    Ecall,

    Ebreak,

    Frrm {
        rd: u8,
    },
    Fsrm {
        rd: u8,
        rs1: u8,
    },

    // m-extension
    Mul {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulh {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulhsu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Mulhu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Div {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Divu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Rem {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Remu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // f/d arithmetic (fp add/sub/mul/div, etc)
    FaddS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FsubS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FmulS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FmaddS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FmsubS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FnmaddS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FnmsubS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FdivS {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FsgnjS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjnS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjxS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsqrtS {
        rd: u8,
        rs1: u8,
        rm: u8,
    },
    FsqrtD {
        rd: u8,
        rs1: u8,
        rm: u8,
    },
    FminS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FmaxS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FaddD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FsubD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FmulD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FmaddD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FmsubD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FnmaddD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FnmsubD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    FdivD {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    FsgnjD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjnD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FsgnjxD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FminD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FmaxD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // fmv instructions (bit-level moves between int and fp)
    FmvSW {
        rd: u8,
        rs1: u8,
    }, // move from single fp to int reg
    FmvWS {
        rd: u8,
        rs1: u8,
    }, // move from int reg to single fp
    FclassS {
        rd: u8,
        rs1: u8,
    },
    FclassD {
        rd: u8,
        rs1: u8,
    },
    FmvXD {
        rd: u8,
        rs1: u8,
    }, // move from double fp to int reg (rv32: lower 32 bits)
    FmvDX {
        rd: u8,
        rs1: u8,
    }, // move from int reg to double fp

    // fcvt instructions (conversions between fp and int, and between precisions)
    FcvtSW {
        rd: u8,
        rs1: u8,
    }, // signed int -> single
    FcvtSWu {
        rd: u8,
        rs1: u8,
    }, // unsigned int -> single
    FcvtWS {
        rd: u8,
        rs1: u8,
    }, // single -> signed int
    FcvtWuS {
        rd: u8,
        rs1: u8,
    }, // single -> unsigned int

    FcvtDW {
        rd: u8,
        rs1: u8,
    }, // signed int -> double
    FcvtDWu {
        rd: u8,
        rs1: u8,
    }, // unsigned int -> double
    FcvtWD {
        rd: u8,
        rs1: u8,
    }, // double -> signed int
    FcvtWuD {
        rd: u8,
        rs1: u8,
    }, // double -> unsigned int

    FcvtSD {
        rd: u8,
        rs1: u8,
    }, // double -> single
    FcvtDS {
        rd: u8,
        rs1: u8,
    }, // single -> double

    // floating point compares (set int reg to 1 if true, else 0)
    FeqS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FltS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FleS {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FeqD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FltD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    FleD {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // loads: imm is 12-bit signed immediate (I-type)
    Flw {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    Fld {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    // stores: S-type with 12-bit immediate
    Fsw {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    Fsd {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
}

impl Instruction {
    pub fn decode(inst: u32) -> Instruction {
        // helper for sign extension
        fn sign_extend(val: u32, bits: u8) -> i32 {
            let shift = 32 - bits;
            ((val << shift) as i32) >> shift
        }

        let opcode = inst & 0x7f;
        let rd = ((inst >> 7) & 0x1f) as u8;
        let funct3 = (inst >> 12) & 0x7;
        let rs1 = ((inst >> 15) & 0x1f) as u8;
        let rs2 = ((inst >> 20) & 0x1f) as u8;
        let sz = ((inst >> 25) & 0x3) as u8;
        let rs3 = ((inst >> 27) & 0x1f) as u8;
        let funct7 = (inst >> 25) & 0x7f;
        let rm = ((inst >> 12) & 0x7) as u8;
        match opcode {
            0x37 => {
                // lui
                let rd = ((inst >> 7) & 0x1f) as u8;
                let imm = (inst & 0xfffff000) as i32;
                Instruction::Lui { rd, imm }
            }
            0x17 => {
                // auipc
                let rd = ((inst >> 7) & 0x1f) as u8;
                let imm = (inst & 0xfffff000) as i32;
                Instruction::Auipc { rd, imm }
            }
            0x6f => {
                // jal
                let rd = ((inst >> 7) & 0x1f) as u8;
                let imm_raw = (((inst >> 31) & 0x1) << 20)
                    | (((inst >> 12) & 0xff) << 12)
                    | (((inst >> 20) & 0x1) << 11)
                    | (((inst >> 21) & 0x3ff) << 1);
                let imm = sign_extend(imm_raw, 21);
                Instruction::Jal { rd, imm }
            }
            0x67 => {
                // jalr (i-type)
                if ((inst >> 12) & 0x7) == 0 {
                    let rd = ((inst >> 7) & 0x1f) as u8;
                    let rs1 = ((inst >> 15) & 0x1f) as u8;
                    let imm = sign_extend((inst >> 20) & 0xfff, 12);
                    Instruction::Jalr { rd, rs1, imm }
                } else {
                    Instruction::Unknown(inst)
                }
            }
            0x63 => {
                // branch (b-type)
                let funct3 = (inst >> 12) & 0x7;
                let rs1 = ((inst >> 15) & 0x1f) as u8;
                let rs2 = ((inst >> 20) & 0x1f) as u8;
                let imm_raw = (((inst >> 31) & 0x1) << 12)
                    | (((inst >> 7) & 0x1) << 11)
                    | (((inst >> 25) & 0x3f) << 5)
                    | (((inst >> 8) & 0xf) << 1);
                let imm = sign_extend(imm_raw, 13);
                match funct3 {
                    0 => Instruction::Beq { rs1, rs2, imm },
                    1 => Instruction::Bne { rs1, rs2, imm },
                    4 => Instruction::Blt { rs1, rs2, imm },
                    5 => Instruction::Bge { rs1, rs2, imm },
                    6 => Instruction::Bltu { rs1, rs2, imm },
                    7 => Instruction::Bgeu { rs1, rs2, imm },
                    _ => Instruction::Unknown(inst),
                }
            }
            0x03 => {
                // load (i-type)
                let funct3 = (inst >> 12) & 0x7;
                let rd = ((inst >> 7) & 0x1f) as u8;
                let rs1 = ((inst >> 15) & 0x1f) as u8;
                let imm = sign_extend((inst >> 20) & 0xfff, 12);
                match funct3 {
                    0 => Instruction::Lb { rd, rs1, imm },
                    1 => Instruction::Lh { rd, rs1, imm },
                    2 => Instruction::Lw { rd, rs1, imm },
                    4 => Instruction::Lbu { rd, rs1, imm },
                    5 => Instruction::Lhu { rd, rs1, imm },
                    _ => Instruction::Unknown(inst),
                }
            }
            0x23 => {
                // store (s-type)
                let funct3 = (inst >> 12) & 0x7;
                let rs1 = ((inst >> 15) & 0x1f) as u8;
                let rs2 = ((inst >> 20) & 0x1f) as u8;
                let imm_raw = (((inst >> 25) & 0x7f) << 5) | ((inst >> 7) & 0x1f);
                let imm = sign_extend(imm_raw, 12);
                match funct3 {
                    0 => Instruction::Sb { rs1, rs2, imm },
                    1 => Instruction::Sh { rs1, rs2, imm },
                    2 => Instruction::Sw { rs1, rs2, imm },
                    _ => Instruction::Unknown(inst),
                }
            }
            0x13 => {
                // op-imm (i-type)
                let funct3 = (inst >> 12) & 0x7;
                let rd = ((inst >> 7) & 0x1f) as u8;
                let rs1 = ((inst >> 15) & 0x1f) as u8;
                match funct3 {
                    0 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::Addi { rd, rs1, imm }
                    }
                    2 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::Slti { rd, rs1, imm }
                    }
                    3 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::Sltiu { rd, rs1, imm }
                    }
                    4 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::Xori { rd, rs1, imm }
                    }
                    6 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::Ori { rd, rs1, imm }
                    }
                    7 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::Andi { rd, rs1, imm }
                    }
                    1 => {
                        // slli; ensure upper bits are 0
                        let shamt = ((inst >> 20) & 0x1f) as u8;
                        Instruction::Slli { rd, rs1, shamt }
                    }
                    5 => {
                        // srli / srai
                        let shamt = ((inst >> 20) & 0x1f) as u8;
                        let funct7 = inst >> 25;
                        if funct7 == 0 {
                            Instruction::Srli { rd, rs1, shamt }
                        } else if funct7 == 0x20 {
                            Instruction::Srai { rd, rs1, shamt }
                        } else {
                            Instruction::Unknown(inst)
                        }
                    }
                    _ => Instruction::Unknown(inst),
                }
            }
            0x33 => {
                // m-extension: only when funct7==0x1
                if funct7 == 0x1 {
                    match funct3 {
                        0x0 => Instruction::Mul { rd, rs1, rs2 },
                        0x1 => Instruction::Mulh { rd, rs1, rs2 },
                        0x2 => Instruction::Mulhsu { rd, rs1, rs2 },
                        0x3 => Instruction::Mulhu { rd, rs1, rs2 },
                        0x4 => Instruction::Div { rd, rs1, rs2 },
                        0x5 => Instruction::Divu { rd, rs1, rs2 },
                        0x6 => Instruction::Rem { rd, rs1, rs2 },
                        0x7 => Instruction::Remu { rd, rs1, rs2 },
                        _ => Instruction::Unknown(inst),
                    }
                } else {
                    match funct3 {
                        0 => {
                            if funct7 == 0 {
                                Instruction::Add { rd, rs1, rs2 }
                            } else if funct7 == 0x20 {
                                Instruction::Sub { rd, rs1, rs2 }
                            } else {
                                Instruction::Unknown(inst)
                            }
                        }
                        1 => Instruction::Sll { rd, rs1, rs2 },
                        2 => Instruction::Slt { rd, rs1, rs2 },
                        3 => Instruction::Sltu { rd, rs1, rs2 },
                        4 => Instruction::Xor { rd, rs1, rs2 },
                        5 => {
                            if funct7 == 0 {
                                Instruction::Srl { rd, rs1, rs2 }
                            } else if funct7 == 0x20 {
                                Instruction::Sra { rd, rs1, rs2 }
                            } else {
                                Instruction::Unknown(inst)
                            }
                        }
                        6 => Instruction::Or { rd, rs1, rs2 },
                        7 => Instruction::And { rd, rs1, rs2 },
                        _ => Instruction::Unknown(inst),
                    }
                }
            }
            0x0f => {
                // fence / fence.i
                let funct3 = (inst >> 12) & 0x7;
                if funct3 == 0 {
                    let pred = ((inst >> 24) & 0xf) as u8;
                    let succ = ((inst >> 20) & 0xf) as u8;
                    Instruction::Fence { pred, succ }
                } else if funct3 == 1 {
                    Instruction::FenceI
                } else {
                    Instruction::Unknown(inst)
                }
            }
            0x73 => {
                let funct3 = (inst >> 12) & 0x7;
                let imm = (inst >> 20) & 0xfff;
                match (funct3, imm) {
                    (0b000000000000, 0b000) => Instruction::Ecall,
                    (0b000000000000, 0b001) => Instruction::Ebreak,
                    (0b000000000010, 0b010) => Instruction::Frrm { rd },
                    (0b000000000010, 0b001) => Instruction::Fsrm { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                }
            }
            0x53 => match funct7 {
                // Single-precision arithmetic
                0x00 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FaddS { rd, rs1, rs2, rm }
                }

                0x2C if rs2 == 0 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FsqrtS { rd, rs1, rm }
                }

                0x2D if rs2 == 0 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FsqrtS { rd, rs1, rm }
                }

                0x04 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FsubS { rd, rs1, rs2, rm }
                }

                0x08 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FmulS { rd, rs1, rs2, rm }
                }

                0x0c => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FdivS { rd, rs1, rs2, rm }
                }

                // Single-precision sign-injection ops
                0x10 => match funct3 {
                    0x0 => Instruction::FsgnjS { rd, rs1, rs2 },
                    0x1 => Instruction::FsgnjnS { rd, rs1, rs2 },
                    0x2 => Instruction::FsgnjxS { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                0x11 => match funct3 {
                    0x0 => Instruction::FsgnjD { rd, rs1, rs2 },
                    0x1 => Instruction::FsgnjnD { rd, rs1, rs2 },
                    0x2 => Instruction::FsgnjxD { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Single-precision min/max
                0x14 => match funct3 {
                    0x0 => Instruction::FminS { rd, rs1, rs2 },
                    0x1 => Instruction::FmaxS { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                0x15 => match funct3 {
                    0x0 => Instruction::FminD { rd, rs1, rs2 },
                    0x1 => Instruction::FmaxD { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Single-precision comparisons
                0x50 => match funct3 {
                    0x0 => Instruction::FleS { rd, rs1, rs2 },
                    0x1 => Instruction::FltS { rd, rs1, rs2 },
                    0x2 => Instruction::FeqS { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Double-precision arithmetic
                0x01 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FaddD { rd, rs1, rs2, rm }
                }
                0x05 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FsubD { rd, rs1, rs2, rm }
                }
                0x09 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FmulD { rd, rs1, rs2, rm }
                }
                0x0d => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::FdivD { rd, rs1, rs2, rm }
                }

                // Double-precision comparisons
                0x51 => match funct3 {
                    0x0 => Instruction::FleD { rd, rs1, rs2 },
                    0x1 => Instruction::FltD { rd, rs1, rs2 },
                    0x2 => Instruction::FeqD { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Conversions & moves (same for both)
                0x60 => match rs2 {
                    0x0 => Instruction::FcvtWS { rd, rs1 },
                    0x1 => Instruction::FcvtWuS { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x68 => match rs2 {
                    0x0 => Instruction::FcvtSW { rd, rs1 },
                    0x1 => Instruction::FcvtSWu { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x61 => match rs2 {
                    0x0 => Instruction::FcvtWD { rd, rs1 },
                    0x1 => Instruction::FcvtWuD { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x69 => match rs2 {
                    0x0 => Instruction::FcvtDW { rd, rs1 },
                    0x1 => Instruction::FcvtDWu { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x78 => Instruction::FmvWS { rd, rs1 },
                0x79 => Instruction::FmvXD { rd, rs1 },
                0x70 => match funct3 {
                    0x0 => Instruction::FmvSW { rd, rs1 },
                    0x1 => Instruction::FclassS { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x71 => match funct3 {
                    0x0 => Instruction::FmvDX { rd, rs1 },
                    0x1 => Instruction::FclassD { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },

                0x7d => Instruction::FcvtWuD { rd, rs1 },
                0x20 => Instruction::FcvtSD { rd, rs1 },
                0x21 => Instruction::FcvtDS { rd, rs1 },
                _ => Instruction::Unknown(inst),
            },
            0x43 => match sz {
                0x0 => Instruction::FmaddS {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::FmaddD {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },
            0x47 => match sz {
                0x0 => Instruction::FmsubS {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::FmsubD {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },

            0x4F => match sz {
                0x0 => Instruction::FnmaddS {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::FnmaddD {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },
            0x4B => match sz {
                0x0 => Instruction::FnmsubS {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::FnmsubD {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },
            0x07 => {
                // fp loads (I-type): imm in bits[31:20]
                let imm = sign_extend(inst >> 20, 12);
                match funct3 {
                    0x2 => Instruction::Flw { rd, rs1, imm },
                    0x3 => Instruction::Fld { rd, rs1, imm },
                    _ => Instruction::Unknown(inst),
                }
            }
            0x27 => {
                // fp stores (S-type): imm = {inst[31:25], inst[11:7]}
                let imm_val = ((inst >> 25) << 5) | ((inst >> 7) & 0x1f);
                let imm = sign_extend(imm_val, 12);
                match funct3 {
                    0x2 => Instruction::Fsw { rs1, rs2, imm },
                    0x3 => Instruction::Fsd { rs1, rs2, imm },
                    _ => Instruction::Unknown(inst),
                }
            }
            _ => Instruction::Unknown(inst),
        }
    }
}
