#[derive(Debug)]
pub enum Instruction {
    unknown(u32),

    lui {
        rd: u8,
        imm: i32,
    },
    auipc {
        rd: u8,
        imm: i32,
    },
    jal {
        rd: u8,
        imm: i32,
    },
    jalr {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    beq {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    bne {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    blt {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    bge {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    bltu {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    bgeu {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    lb {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    lh {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    lw {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    lbu {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    lhu {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    sb {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    sh {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    sw {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    addi {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    slti {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    sltiu {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    xori {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    ori {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    andi {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    slli {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    srli {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    srai {
        rd: u8,
        rs1: u8,
        shamt: u8,
    },
    add {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    sub {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    sll {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    slt {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    sltu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    xor {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    srl {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    sra {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    or {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    and {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fence {
        pred: u8,
        succ: u8,
    },
    fence_i,
    ecall,
    ebreak,

    // m-extension
    mul {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    mulh {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    mulhsu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    mulhu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    div {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    divu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    rem {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    remu {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // f/d arithmetic (fp add/sub/mul/div, etc)
    fadd_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fsub_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fmul_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fmadd_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    fmsub_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    fdiv_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fsgnj_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fsgnjn_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fsgnjx_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fmin_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fmax_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fadd_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fsub_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fmul_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fmadd_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    fmsub_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    fdiv_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rm: u8,
    },
    fsgnj_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fsgnjn_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fsgnjx_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fmin_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fmax_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // fmv instructions (bit-level moves between int and fp)
    fmv_s_w {
        rd: u8,
        rs1: u8,
    }, // move from single fp to int reg
    fmv_w_s {
        rd: u8,
        rs1: u8,
    }, // move from int reg to single fp
    fmv_x_d {
        rd: u8,
        rs1: u8,
    }, // move from double fp to int reg (rv32: lower 32 bits)
    fmv_d_x {
        rd: u8,
        rs1: u8,
    }, // move from int reg to double fp

    // fcvt instructions (conversions between fp and int, and between precisions)
    fcvt_s_w {
        rd: u8,
        rs1: u8,
    }, // signed int -> single
    fcvt_s_wu {
        rd: u8,
        rs1: u8,
    }, // unsigned int -> single
    fcvt_w_s {
        rd: u8,
        rs1: u8,
    }, // single -> signed int
    fcvt_wu_s {
        rd: u8,
        rs1: u8,
    }, // single -> unsigned int

    fcvt_d_w {
        rd: u8,
        rs1: u8,
    }, // signed int -> double
    fcvt_d_wu {
        rd: u8,
        rs1: u8,
    }, // unsigned int -> double
    fcvt_w_d {
        rd: u8,
        rs1: u8,
    }, // double -> signed int
    fcvt_wu_d {
        rd: u8,
        rs1: u8,
    }, // double -> unsigned int

    fcvt_s_d {
        rd: u8,
        rs1: u8,
    }, // double -> single
    fcvt_d_s {
        rd: u8,
        rs1: u8,
    }, // single -> double

    // floating point compares (set int reg to 1 if true, else 0)
    feq_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    flt_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fle_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    feq_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    flt_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },
    fle_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
    },

    // loads: imm is 12-bit signed immediate (I-type)
    flw {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    fld {
        rd: u8,
        rs1: u8,
        imm: i32,
    },
    // stores: S-type with 12-bit immediate
    fsw {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    fsd {
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
}

#[derive(Debug)]
pub enum InstructionDecodeError {
    InvalidOpcode,
    InvalidFunct3,
    InvalidFunct7,
}

impl Instruction {
    // Main dispatcher: calls one decode fn per opcode.
    pub fn decode(inst: u32) -> Result<Instruction, InstructionDecodeError> {
        let opcode = inst & 0x7F;
        let rd = ((inst >> 7) & 0x1F) as u8;
        let funct3 = ((inst >> 12) & 0x7) as u8;
        let rs1 = ((inst >> 15) & 0x1F) as u8;
        let rs2 = ((inst >> 20) & 0x1F) as u8;
        let funct7 = ((inst >> 25) & 0x7F) as u8;
        let funct7_fp = funct7 & 0xFE; // Remove bottom bit for FP instructions

        let imm = ((inst as i32) >> 20); // Sign-extended 12-bit immediate
        let imm_u = (inst & 0xFFFFF000) as i32;

        let sh = imm >> 5;
        let shamt = (imm & 5) as u8;

        match opcode {
            0b0110111 => Ok(Instruction::lui { rd, imm: imm_u }),
            0b0010111 => Ok(Instruction::auipc { rd, imm: imm_u }),
            0b1101111 => {
                // let imm_j = ((((inst >> 21) & 0x3FF)
                //     | (((inst >> 20) & 1) << 10)
                //     | (((inst >> 12) & 0xFF) << 11)
                //     | (((inst >> 31) & 1) << 19))
                //     << 1) as i32;
                let imm20 = ((inst >> 31) & 1) << 20;
                let imm10_1 = ((inst >> 21) & 0x3FF) << 1;
                let imm11 = ((inst >> 20) & 1) << 11;
                let imm19_12 = ((inst >> 12) & 0xFF) << 12;

                let imm = imm20 | imm19_12 | imm11 | imm10_1;

                // Sign-extend from 21-bit to i32
                let imm_j = ((imm as i32) << 11) >> 11;
                Ok(Instruction::jal { rd, imm: imm_j })
            }
            0b1100111 if funct3 == 0 => Ok(Instruction::jalr { rd, rs1, imm }),
            0b0010011 => match funct3 {
                0b000 => Ok(Instruction::addi { rd, rs1, imm }),
                0b001 => Ok(Instruction::slli { rd, rs1, shamt }),
                0b101 => match sh {
                    0b0000000 => Ok(Instruction::srli { rd, rs1, shamt }),
                    0b0100000 => Ok(Instruction::srai { rd, rs1, shamt }),
                    _ => panic!("bad shift"),
                },
                0b010 => Ok(Instruction::slti { rd, rs1, imm }),
                0b011 => Ok(Instruction::sltiu { rd, rs1, imm }),
                0b100 => Ok(Instruction::xori { rd, rs1, imm }),
                0b110 => Ok(Instruction::ori { rd, rs1, imm }),
                0b111 => Ok(Instruction::andi { rd, rs1, imm }),
                _ => Err(InstructionDecodeError::InvalidFunct3),
            },
            0b0110011 => match (funct3, funct7) {
                (0b000, 0b0000000) => Ok(Instruction::add { rd, rs1, rs2 }),
                (0b000, 0b0100000) => Ok(Instruction::sub { rd, rs1, rs2 }),
                (0b001, 0b0000000) => Ok(Instruction::sll { rd, rs1, rs2 }),
                (0b010, 0b0000000) => Ok(Instruction::slt { rd, rs1, rs2 }),
                (0b011, 0b0000000) => Ok(Instruction::sltu { rd, rs1, rs2 }),
                (0b100, 0b0000000) => Ok(Instruction::xor { rd, rs1, rs2 }),
                (0b101, 0b0000000) => Ok(Instruction::srl { rd, rs1, rs2 }),
                (0b101, 0b0100000) => Ok(Instruction::sra { rd, rs1, rs2 }),
                (0b110, 0b0000000) => Ok(Instruction::or { rd, rs1, rs2 }),
                (0b111, 0b0000000) => Ok(Instruction::and { rd, rs1, rs2 }),
                (0b000, 0b0000001) => Ok(Instruction::mul { rd, rs1, rs2 }),
                (0b001, 0b0000001) => Ok(Instruction::mulh { rd, rs1, rs2 }),
                (0b010, 0b0000001) => Ok(Instruction::mulhsu { rd, rs1, rs2 }),
                (0b011, 0b0000001) => Ok(Instruction::mulhu { rd, rs1, rs2 }),
                (0b100, 0b0000001) => Ok(Instruction::div { rd, rs1, rs2 }),
                (0b101, 0b0000001) => Ok(Instruction::divu { rd, rs1, rs2 }),
                (0b110, 0b0000001) => Ok(Instruction::rem { rd, rs1, rs2 }),
                (0b111, 0b0000001) => Ok(Instruction::remu { rd, rs1, rs2 }),
                _ => Err(InstructionDecodeError::InvalidFunct7),
            },
            0b1100011 => {
                let imm12 = ((inst >> 31) & 1) << 12;
                let imm10_5 = ((inst >> 25) & 0b11111) << 5;
                let imm11 = ((inst >> 7) & 1) << 11;
                let imm4_1 = ((inst >> 8) & 0b1111) << 1;

                let imm = imm12 | imm11 | imm10_5 | imm4_1;

                // Sign-extend from 21-bit to i32
                let b_imm = ((imm as i32) << 11) >> 11;

                match funct3 {
                    0b000 => Ok(Instruction::beq {
                        rs1,
                        rs2,
                        imm: b_imm,
                    }),
                    0b001 => Ok(Instruction::bne {
                        rs1,
                        rs2,
                        imm: b_imm,
                    }),
                    0b100 => Ok(Instruction::blt {
                        rs1,
                        rs2,
                        imm: b_imm,
                    }),
                    0b101 => Ok(Instruction::bge {
                        rs1,
                        rs2,
                        imm: b_imm,
                    }),
                    0b110 => Ok(Instruction::bltu {
                        rs1,
                        rs2,
                        imm: b_imm,
                    }),
                    0b111 => Ok(Instruction::bgeu {
                        rs1,
                        rs2,
                        imm: b_imm,
                    }),
                    _ => Err(InstructionDecodeError::InvalidFunct3),
                }
            }
            0b1010011 => match (funct3, funct7_fp) {
                (0b000, 0b0000000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fadd_s {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                } else {
                    Instruction::fadd_d {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                }),
                (0b001, 0b0000100) => Ok(if funct7 & 1 == 0 {
                    Instruction::fsub_s {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                } else {
                    Instruction::fsub_d {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                }),
                (0b010, 0b0001000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fmul_s {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                } else {
                    Instruction::fmul_d {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                }),
                (0b011, 0b0001100) => Ok(if funct7 & 1 == 0 {
                    Instruction::fdiv_s {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                } else {
                    Instruction::fdiv_d {
                        rd,
                        rs1,
                        rs2,
                        rm: funct3,
                    }
                }),
                (0b000, 0b1010000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fsgnj_s { rd, rs1, rs2 }
                } else {
                    Instruction::fsgnj_d { rd, rs1, rs2 }
                }),
                (0b001, 0b1010000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fsgnjn_s { rd, rs1, rs2 }
                } else {
                    Instruction::fsgnjn_d { rd, rs1, rs2 }
                }),
                (0b010, 0b1010000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fsgnjx_s { rd, rs1, rs2 }
                } else {
                    Instruction::fsgnjx_d { rd, rs1, rs2 }
                }),
                (0b000, 0b1100000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fcvt_s_w { rd, rs1 }
                } else {
                    Instruction::fcvt_d_w { rd, rs1 }
                }),
                (0b000, 0b1100001) => Ok(if funct7 & 1 == 0 {
                    Instruction::fcvt_s_wu { rd, rs1 }
                } else {
                    Instruction::fcvt_d_wu { rd, rs1 }
                }),
                (0b000, 0b0100000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fcvt_s_d { rd, rs1 }
                } else {
                    Instruction::fcvt_d_s { rd, rs1 }
                }),
                (0b000, 0b1110000) => Ok(if funct7 & 1 == 0 {
                    Instruction::fmv_s_w { rd, rs1 }
                } else {
                    Instruction::fmv_d_x { rd, rs1 }
                }),
                _ => Err(InstructionDecodeError::InvalidFunct7),
            },
            0b0000011 => match funct3 {
                0b000 => Ok(Instruction::lb { rd, rs1, imm }),
                0b001 => Ok(Instruction::lh { rd, rs1, imm }),
                0b010 => Ok(Instruction::lw { rd, rs1, imm }),
                0b100 => Ok(Instruction::lbu { rd, rs1, imm }),
                0b101 => Ok(Instruction::lhu { rd, rs1, imm }),
                _ => Err(InstructionDecodeError::InvalidFunct3),
            },
            0b0100011 => match funct3 {
                0b000 => Ok(Instruction::sb { rs1, rs2, imm }),
                0b001 => Ok(Instruction::sh { rs1, rs2, imm }),
                0b010 => Ok(Instruction::sw { rs1, rs2, imm }),
                _ => Err(InstructionDecodeError::InvalidFunct3),
            },
            0b0000111 => match funct3 {
                0b010 => Ok(Instruction::flw { rd, rs1, imm }),
                0b011 => Ok(Instruction::fld { rd, rs1, imm }),
                _ => Err(InstructionDecodeError::InvalidFunct3),
            },
            0b0100111 => match funct3 {
                0b010 => Ok(Instruction::fsw { rs1, rs2, imm }),
                0b011 => Ok(Instruction::fsd { rs1, rs2, imm }),
                _ => Err(InstructionDecodeError::InvalidFunct3),
            },
            0b1110011 => match imm {
                0 => Ok(Instruction::ecall),
                1 => Ok(Instruction::ebreak),
                _ => Err(InstructionDecodeError::InvalidFunct3),
            },
            _ => Ok(Instruction::unknown(inst)),
        }
    }
}
