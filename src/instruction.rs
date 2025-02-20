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

    frrm {
        rd: u8,
    },
    fsrm {
        rd: u8,
        rs1: u8,
    },

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
    fnmadd_s {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    fnmsub_s {
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
    fnmadd_d {
        rd: u8,
        rs1: u8,
        rs2: u8,
        rs3: u8,
        rm: u8,
    },
    fnmsub_d {
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
                Instruction::lui { rd, imm }
            }
            0x17 => {
                // auipc
                let rd = ((inst >> 7) & 0x1f) as u8;
                let imm = (inst & 0xfffff000) as i32;
                Instruction::auipc { rd, imm }
            }
            0x6f => {
                // jal
                let rd = ((inst >> 7) & 0x1f) as u8;
                let imm_raw = (((inst >> 31) & 0x1) << 20)
                    | (((inst >> 12) & 0xff) << 12)
                    | (((inst >> 20) & 0x1) << 11)
                    | (((inst >> 21) & 0x3ff) << 1);
                let imm = sign_extend(imm_raw, 21);
                Instruction::jal { rd, imm }
            }
            0x67 => {
                // jalr (i-type)
                if ((inst >> 12) & 0x7) == 0 {
                    let rd = ((inst >> 7) & 0x1f) as u8;
                    let rs1 = ((inst >> 15) & 0x1f) as u8;
                    let imm = sign_extend((inst >> 20) & 0xfff, 12);
                    Instruction::jalr { rd, rs1, imm }
                } else {
                    Instruction::unknown(inst)
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
                    0 => Instruction::beq { rs1, rs2, imm },
                    1 => Instruction::bne { rs1, rs2, imm },
                    4 => Instruction::blt { rs1, rs2, imm },
                    5 => Instruction::bge { rs1, rs2, imm },
                    6 => Instruction::bltu { rs1, rs2, imm },
                    7 => Instruction::bgeu { rs1, rs2, imm },
                    _ => Instruction::unknown(inst),
                }
            }
            0x03 => {
                // load (i-type)
                let funct3 = (inst >> 12) & 0x7;
                let rd = ((inst >> 7) & 0x1f) as u8;
                let rs1 = ((inst >> 15) & 0x1f) as u8;
                let imm = sign_extend((inst >> 20) & 0xfff, 12);
                match funct3 {
                    0 => Instruction::lb { rd, rs1, imm },
                    1 => Instruction::lh { rd, rs1, imm },
                    2 => Instruction::lw { rd, rs1, imm },
                    4 => Instruction::lbu { rd, rs1, imm },
                    5 => Instruction::lhu { rd, rs1, imm },
                    _ => Instruction::unknown(inst),
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
                    0 => Instruction::sb { rs1, rs2, imm },
                    1 => Instruction::sh { rs1, rs2, imm },
                    2 => Instruction::sw { rs1, rs2, imm },
                    _ => Instruction::unknown(inst),
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
                        Instruction::addi { rd, rs1, imm }
                    }
                    2 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::slti { rd, rs1, imm }
                    }
                    3 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::sltiu { rd, rs1, imm }
                    }
                    4 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::xori { rd, rs1, imm }
                    }
                    6 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::ori { rd, rs1, imm }
                    }
                    7 => {
                        let imm = sign_extend((inst >> 20) & 0xfff, 12);
                        Instruction::andi { rd, rs1, imm }
                    }
                    1 => {
                        // slli; ensure upper bits are 0
                        let shamt = ((inst >> 20) & 0x1f) as u8;
                        Instruction::slli { rd, rs1, shamt }
                    }
                    5 => {
                        // srli / srai
                        let shamt = ((inst >> 20) & 0x1f) as u8;
                        let funct7 = inst >> 25;
                        if funct7 == 0 {
                            Instruction::srli { rd, rs1, shamt }
                        } else if funct7 == 0x20 {
                            Instruction::srai { rd, rs1, shamt }
                        } else {
                            Instruction::unknown(inst)
                        }
                    }
                    _ => Instruction::unknown(inst),
                }
            }
            0x33 => {
                // m-extension: only when funct7==0x1
                if funct7 == 0x1 {
                    match funct3 {
                        0x0 => Instruction::mul { rd, rs1, rs2 },
                        0x1 => Instruction::mulh { rd, rs1, rs2 },
                        0x2 => Instruction::mulhsu { rd, rs1, rs2 },
                        0x3 => Instruction::mulhu { rd, rs1, rs2 },
                        0x4 => Instruction::div { rd, rs1, rs2 },
                        0x5 => Instruction::divu { rd, rs1, rs2 },
                        0x6 => Instruction::rem { rd, rs1, rs2 },
                        0x7 => Instruction::remu { rd, rs1, rs2 },
                        _ => Instruction::unknown(inst),
                    }
                } else {
                    match funct3 {
                        0 => {
                            if funct7 == 0 {
                                Instruction::add { rd, rs1, rs2 }
                            } else if funct7 == 0x20 {
                                Instruction::sub { rd, rs1, rs2 }
                            } else {
                                Instruction::unknown(inst)
                            }
                        }
                        1 => Instruction::sll { rd, rs1, rs2 },
                        2 => Instruction::slt { rd, rs1, rs2 },
                        3 => Instruction::sltu { rd, rs1, rs2 },
                        4 => Instruction::xor { rd, rs1, rs2 },
                        5 => {
                            if funct7 == 0 {
                                Instruction::srl { rd, rs1, rs2 }
                            } else if funct7 == 0x20 {
                                Instruction::sra { rd, rs1, rs2 }
                            } else {
                                Instruction::unknown(inst)
                            }
                        }
                        6 => Instruction::or { rd, rs1, rs2 },
                        7 => Instruction::and { rd, rs1, rs2 },
                        _ => Instruction::unknown(inst),
                    }
                }
            }
            0x0f => {
                // fence / fence.i
                let funct3 = (inst >> 12) & 0x7;
                if funct3 == 0 {
                    let pred = ((inst >> 24) & 0xf) as u8;
                    let succ = ((inst >> 20) & 0xf) as u8;
                    Instruction::fence { pred, succ }
                } else if funct3 == 1 {
                    Instruction::fence_i
                } else {
                    Instruction::unknown(inst)
                }
            }
            0x73 => {
                let funct3 = (inst >> 12) & 0x7;
                let imm = (inst >> 20) & 0xfff;
                match (funct3, imm) {
                    (0b000000000000, 0b000) => Instruction::ecall,
                    (0b000000000000, 0b001) => Instruction::ebreak,
                    (0b000000000010, 0b010) => Instruction::frrm { rd },
                    (0b000000000010, 0b001) => Instruction::fsrm { rd, rs1 },
                    _ => Instruction::unknown(inst),
                }
            }
            0x53 => match funct7 {
                // Single-precision arithmetic
                0x00 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fadd_s { rd, rs1, rs2, rm }
                }

                0x04 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fsub_s { rd, rs1, rs2, rm }
                }

                0x08 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fmul_s { rd, rs1, rs2, rm }
                }

                0x0c => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fdiv_s { rd, rs1, rs2, rm }
                }

                // Single-precision sign-injection ops
                0x10 => match funct3 {
                    0x0 => Instruction::fsgnj_s { rd, rs1, rs2 },
                    0x1 => Instruction::fsgnjn_s { rd, rs1, rs2 },
                    0x2 => Instruction::fsgnjx_s { rd, rs1, rs2 },
                    _ => Instruction::unknown(inst),
                },

                0x11 => match funct3 {
                    0x0 => Instruction::fsgnj_d { rd, rs1, rs2 },
                    0x1 => Instruction::fsgnjn_d { rd, rs1, rs2 },
                    0x2 => Instruction::fsgnjx_d { rd, rs1, rs2 },
                    _ => Instruction::unknown(inst),
                },

                // Single-precision min/max
                0x14 => match funct3 {
                    0x0 => Instruction::fmin_s { rd, rs1, rs2 },
                    0x1 => Instruction::fmax_s { rd, rs1, rs2 },
                    _ => Instruction::unknown(inst),
                },

                0x15 => match funct3 {
                    0x0 => Instruction::fmin_d { rd, rs1, rs2 },
                    0x1 => Instruction::fmax_d { rd, rs1, rs2 },
                    _ => Instruction::unknown(inst),
                },

                // Single-precision comparisons
                0x50 => match funct3 {
                    0x0 => Instruction::fle_s { rd, rs1, rs2 },
                    0x1 => Instruction::flt_s { rd, rs1, rs2 },
                    0x2 => Instruction::feq_s { rd, rs1, rs2 },
                    _ => Instruction::unknown(inst),
                },

                // Double-precision arithmetic
                0x01 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fadd_d { rd, rs1, rs2, rm }
                }
                0x05 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fsub_d { rd, rs1, rs2, rm }
                }
                0x09 => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fmul_d { rd, rs1, rs2, rm }
                }
                0x0d => {
                    let rm = ((inst >> 12) & 0x7) as u8;
                    Instruction::fdiv_d { rd, rs1, rs2, rm }
                }

                // Double-precision comparisons
                0x51 => match funct3 {
                    0x0 => Instruction::fle_d { rd, rs1, rs2 },
                    0x1 => Instruction::flt_d { rd, rs1, rs2 },
                    0x2 => Instruction::feq_d { rd, rs1, rs2 },
                    _ => Instruction::unknown(inst),
                },

                // Conversions & moves (same for both)
                0x60 => match funct3 {
                    0x0 => Instruction::fcvt_w_s { rd, rs1 },
                    0x1 => Instruction::fcvt_wu_s { rd, rs1 },
                    _ => Instruction::unknown(inst),
                },
                0x68 => match funct3 {
                    0x0 => Instruction::fcvt_s_w { rd, rs1 },
                    0x1 => Instruction::fcvt_s_wu { rd, rs1 },
                    _ => Instruction::unknown(inst),
                },
                0x61 => match funct3 {
                    0x0 => Instruction::fcvt_w_d { rd, rs1 },
                    0x1 => Instruction::fcvt_wu_d { rd, rs1 },
                    _ => Instruction::unknown(inst),
                },
                0x69 => match funct3 {
                    0x0 => Instruction::fcvt_d_w { rd, rs1 },
                    0x1 => Instruction::fcvt_d_wu { rd, rs1 },
                    _ => Instruction::unknown(inst),
                },
                0x78 => Instruction::fmv_w_s { rd, rs1 },
                0x70 => Instruction::fmv_s_w { rd, rs1 },

                0x7d => Instruction::fcvt_wu_d { rd, rs1 },
                0x20 => Instruction::fcvt_s_d { rd, rs1 },
                0x21 => Instruction::fcvt_d_s { rd, rs1 },
                _ => Instruction::unknown(inst),
            },
            0x43 => match sz {
                0x0 => Instruction::fmadd_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::fmadd_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::unknown(inst),
            },
            0x47 => match sz {
                0x0 => Instruction::fmsub_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::fmsub_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::unknown(inst),
            },

            0x4F => match sz {
                0x0 => Instruction::fnmadd_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::fnmadd_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::unknown(inst),
            },
            0x4B => match sz {
                0x0 => Instruction::fnmsub_s {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                0x1 => Instruction::fnmsub_d {
                    rd,
                    rs1,
                    rs2,
                    rs3,
                    rm,
                },
                _ => Instruction::unknown(inst),
            },
            0x07 => {
                // fp loads (I-type): imm in bits[31:20]
                let imm = sign_extend(inst >> 20, 12);
                match funct3 {
                    0x2 => Instruction::flw { rd, rs1, imm },
                    0x3 => Instruction::fld { rd, rs1, imm },
                    _ => Instruction::unknown(inst),
                }
            }
            0x27 => {
                // fp stores (S-type): imm = {inst[31:25], inst[11:7]}
                let imm_val = ((inst >> 25) << 5) | ((inst >> 7) & 0x1f);
                let imm = sign_extend(imm_val, 12);
                match funct3 {
                    0x2 => Instruction::fsw { rs1, rs2, imm },
                    0x3 => Instruction::fsd { rs1, rs2, imm },
                    _ => Instruction::unknown(inst),
                }
            }
            _ => Instruction::unknown(inst),
        }
    }
}
