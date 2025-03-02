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

    Fence_i,

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
    Fadd_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fsub_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fmul_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fmadd_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fmsub_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fnmadd_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fnmsub_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fdiv_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fsgnj_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fsgnjn_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fsgnjx_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fsqrt_s {
        rd: u8,
        rs1: u8,
        rm: u8,
    },
    Fsqrt_d {
        rd: u8,
        rs1: u8,
        rm: u8,
    },
    Fmin_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fmax_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fadd_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fsub_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fmul_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fmadd_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fmsub_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fnmadd_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fnmsub_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    Fdiv_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    Fsgnj_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fsgnjn_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fsgnjx_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fmin_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fmax_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // fmv instructions (bit-level moves between int and fp)
    Fmv_s_w {
        rd: u8,
        rs1: u8,
    }, // move from single fp to int reg
    Fmv_w_s {
        rd: u8,
        rs1: u8,
    }, // move from int reg to single fp
    Fmv_x_d {
        rd: u8,
        rs1: u8,
    }, // move from double fp to int reg (rv32: lower 32 bits)
    Fmv_d_x {
        rd: u8,
        rs1: u8,
    }, // move from int reg to double fp

    // fcvt instructions (conversions between fp and int, and between precisions)
    Fcvt_s_w {
        rd: u8,
        rs1: u8,
    }, // signed int -> single
    Fcvt_s_wu {
        rd: u8,
        rs1: u8,
    }, // unsigned int -> single
    Fcvt_w_s {
        rd: u8,
        rs1: u8,
    }, // single -> signed int
    Fcvt_wu_s {
        rd: u8,
        rs1: u8,
    }, // single -> unsigned int

    Fcvt_d_w {
        rd: u8,
        rs1: u8,
    }, // signed int -> double
    Fcvt_d_wu {
        rd: u8,
        rs1: u8,
    }, // unsigned int -> double
    Fcvt_w_d {
        rd: u8,
        rs1: u8,
    }, // double -> signed int
    Fcvt_wu_d {
        rd: u8,
        rs1: u8,
    }, // double -> unsigned int

    Fcvt_s_d {
        rd: u8,
        rs1: u8,
    }, // double -> single
    Fcvt_d_s {
        rd: u8,
        rs1: u8,
    }, // single -> double

    // floating point compares (set int reg to 1 if true, else 0)
    Feq_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Flt_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fle_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Feq_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Flt_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    Fle_d {
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
                    Instruction::Fence_i
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
                    Instruction::Fadd_s { rd, rs1, rs2, rm }
                }

                0x2C if rs2 == 0 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fsqrt_s { rd, rs1, rm }
                }

                0x2D if rs2 == 0 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fsqrt_s { rd, rs1, rm }
                }

                0x04 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fsub_s { rd, rs1, rs2, rm }
                }

                0x08 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fmul_s { rd, rs1, rs2, rm }
                }

                0x0c => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fdiv_s { rd, rs1, rs2, rm }
                }

                // Single-precision sign-injection ops
                0x10 => match funct3 {
                    0x0 => Instruction::Fsgnj_s { rd, rs1, rs2 },
                    0x1 => Instruction::Fsgnjn_s { rd, rs1, rs2 },
                    0x2 => Instruction::Fsgnjx_s { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                0x11 => match funct3 {
                    0x0 => Instruction::Fsgnj_d { rd, rs1, rs2 },
                    0x1 => Instruction::Fsgnjn_d { rd, rs1, rs2 },
                    0x2 => Instruction::Fsgnjx_d { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Single-precision min/max
                0x14 => match funct3 {
                    0x0 => Instruction::Fmin_s { rd, rs1, rs2 },
                    0x1 => Instruction::Fmax_s { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                0x15 => match funct3 {
                    0x0 => Instruction::Fmin_d { rd, rs1, rs2 },
                    0x1 => Instruction::Fmax_d { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Single-precision comparisons
                0x50 => match funct3 {
                    0x0 => Instruction::Fle_s { rd, rs1, rs2 },
                    0x1 => Instruction::Flt_s { rd, rs1, rs2 },
                    0x2 => Instruction::Feq_s { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Double-precision arithmetic
                0x01 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fadd_d { rd, rs1, rs2, rm }
                }
                0x05 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fsub_d { rd, rs1, rs2, rm }
                }
                0x09 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fmul_d { rd, rs1, rs2, rm }
                }
                0x0d => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::Fdiv_d { rd, rs1, rs2, rm }
                }

                // Double-precision comparisons
                0x51 => match funct3 {
                    0x0 => Instruction::Fle_d { rd, rs1, rs2 },
                    0x1 => Instruction::Flt_d { rd, rs1, rs2 },
                    0x2 => Instruction::Feq_d { rd, rs1, rs2 },
                    _ => Instruction::Unknown(inst),
                },

                // Conversions & moves (same for both)
                0x60 => match rs2 {
                    0x0 => Instruction::Fcvt_w_s { rd, rs1 },
                    0x1 => Instruction::Fcvt_wu_s { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x68 => match rs2 {
                    0x0 => Instruction::Fcvt_s_w { rd, rs1 },
                    0x1 => Instruction::Fcvt_s_wu { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x61 => match rs2 {
                    0x0 => Instruction::Fcvt_w_d { rd, rs1 },
                    0x1 => Instruction::Fcvt_wu_d { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x69 => match rs2 {
                    0x0 => Instruction::Fcvt_d_w { rd, rs1 },
                    0x1 => Instruction::Fcvt_d_wu { rd, rs1 },
                    _ => Instruction::Unknown(inst),
                },
                0x78 => Instruction::Fmv_w_s { rd, rs1 },
                0x70 => Instruction::Fmv_s_w { rd, rs1 },

                0x7d => Instruction::Fcvt_wu_d { rd, rs1 },
                0x20 => Instruction::Fcvt_s_d { rd, rs1 },
                0x21 => Instruction::Fcvt_d_s { rd, rs1 },
                _ => Instruction::Unknown(inst),
            },
            0x43 => match sz {
                0x0 => Instruction::Fmadd_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::Fmadd_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },
            0x47 => match sz {
                0x0 => Instruction::Fmsub_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::Fmsub_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },

            0x4F => match sz {
                0x0 => Instruction::Fnmadd_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::Fnmadd_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::Unknown(inst),
            },
            0x4B => match sz {
                0x0 => Instruction::Fnmsub_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::Fnmsub_d {
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
