/* types */

#[allow(non_camel_case_types)]
pub type rv_inst=u64;
#[allow(non_camel_case_types)]
pub type rv_opcode=u16;

/* enums */

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rv_isa {
    rv32,
    rv64,
    rv128
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rv_rm {
    rne = 0,
    rtz = 1,
    rdn = 2,
    rup = 3,
    rmm = 4,
    dyn = 7,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rv_fence {
    i = 8,
    o = 4,
    r = 2,
    w = 1,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rv_ireg {
    zero,
    ra,
    sp,
    gp,
    tp,
    t0,
    t1,
    t2,
    s0,
    s1,
    a0,
    a1,
    a2,
    a3,
    a4,
    a5,
    a6,
    a7,
    s2,
    s3,
    s4,
    s5,
    s6,
    s7,
    s8,
    s9,
    s10,
    s11,
    t3,
    t4,
    t5,
    t6,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rvc_constraint {
    end,
    rd_eq_ra,
    rd_eq_x0,
    rs1_eq_x0,
    rs2_eq_x0,
    rs2_eq_rs1,
    rs1_eq_ra,
    imm_eq_zero,
    imm_eq_n1,
    imm_eq_p1,
    csr_eq_0x001,
    csr_eq_0x002,
    csr_eq_0x003,
    csr_eq_0xc00,
    csr_eq_0xc01,
    csr_eq_0xc02,
    csr_eq_0xc80,
    csr_eq_0xc81,
    csr_eq_0xc82,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rv_codec {
    illegal,
    none,
    u,
    uj,
    i,
    i_sh5,
    i_sh6,
    i_sh7,
    i_csr,
    s,
    sb,
    r,
    r_m,
    r4_m,
    r_a,
    r_l,
    r_f,
    cb,
    cb_imm,
    cb_sh5,
    cb_sh6,
    ci,
    ci_sh5,
    ci_sh6,
    ci_16sp,
    ci_lwsp,
    ci_ldsp,
    ci_lqsp,
    ci_li,
    ci_lui,
    ci_none,
    ciw_4spn,
    cj,
    cj_jal,
    cl_lw,
    cl_ld,
    cl_lq,
    cr,
    cr_mv,
    cr_jalr,
    cr_jr,
    cs,
    cs_sw,
    cs_sd,
    cs_sq,
    css_swsp,
    css_sdsp,
    css_sqsp,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum rv_op {
    illegal = 0,
    lui = 1,
    auipc = 2,
    jal = 3,
    jalr = 4,
    beq = 5,
    bne = 6,
    blt = 7,
    bge = 8,
    bltu = 9,
    bgeu = 10,
    lb = 11,
    lh = 12,
    lw = 13,
    lbu = 14,
    lhu = 15,
    sb = 16,
    sh = 17,
    sw = 18,
    addi = 19,
    slti = 20,
    sltiu = 21,
    xori = 22,
    ori = 23,
    andi = 24,
    slli = 25,
    srli = 26,
    srai = 27,
    add = 28,
    sub = 29,
    sll = 30,
    slt = 31,
    sltu = 32,
    xor = 33,
    srl = 34,
    sra = 35,
    or = 36,
    and = 37,
    fence = 38,
    fence_i = 39,
    lwu = 40,
    ld = 41,
    sd = 42,
    addiw = 43,
    slliw = 44,
    srliw = 45,
    sraiw = 46,
    addw = 47,
    subw = 48,
    sllw = 49,
    srlw = 50,
    sraw = 51,
    ldu = 52,
    lq = 53,
    sq = 54,
    addid = 55,
    sllid = 56,
    srlid = 57,
    sraid = 58,
    addd = 59,
    subd = 60,
    slld = 61,
    srld = 62,
    srad = 63,
    mul = 64,
    mulh = 65,
    mulhsu = 66,
    mulhu = 67,
    div = 68,
    divu = 69,
    rem = 70,
    remu = 71,
    mulw = 72,
    divw = 73,
    divuw = 74,
    remw = 75,
    remuw = 76,
    muld = 77,
    divd = 78,
    divud = 79,
    remd = 80,
    remud = 81,
    lr_w = 82,
    sc_w = 83,
    amoswap_w = 84,
    amoadd_w = 85,
    amoxor_w = 86,
    amoor_w = 87,
    amoand_w = 88,
    amomin_w = 89,
    amomax_w = 90,
    amominu_w = 91,
    amomaxu_w = 92,
    lr_d = 93,
    sc_d = 94,
    amoswap_d = 95,
    amoadd_d = 96,
    amoxor_d = 97,
    amoor_d = 98,
    amoand_d = 99,
    amomin_d = 100,
    amomax_d = 101,
    amominu_d = 102,
    amomaxu_d = 103,
    lr_q = 104,
    sc_q = 105,
    amoswap_q = 106,
    amoadd_q = 107,
    amoxor_q = 108,
    amoor_q = 109,
    amoand_q = 110,
    amomin_q = 111,
    amomax_q = 112,
    amominu_q = 113,
    amomaxu_q = 114,
    ecall = 115,
    ebreak = 116,
    uret = 117,
    sret = 118,
    hret = 119,
    mret = 120,
    dret = 121,
    sfence_vm = 122,
    sfence_vma = 123,
    wfi = 124,
    csrrw = 125,
    csrrs = 126,
    csrrc = 127,
    csrrwi = 128,
    csrrsi = 129,
    csrrci = 130,
    flw = 131,
    fsw = 132,
    fmadd_s = 133,
    fmsub_s = 134,
    fnmsub_s = 135,
    fnmadd_s = 136,
    fadd_s = 137,
    fsub_s = 138,
    fmul_s = 139,
    fdiv_s = 140,
    fsgnj_s = 141,
    fsgnjn_s = 142,
    fsgnjx_s = 143,
    fmin_s = 144,
    fmax_s = 145,
    fsqrt_s = 146,
    fle_s = 147,
    flt_s = 148,
    feq_s = 149,
    fcvt_w_s = 150,
    fcvt_wu_s = 151,
    fcvt_s_w = 152,
    fcvt_s_wu = 153,
    fmv_x_s = 154,
    fclass_s = 155,
    fmv_s_x = 156,
    fcvt_l_s = 157,
    fcvt_lu_s = 158,
    fcvt_s_l = 159,
    fcvt_s_lu = 160,
    fld = 161,
    fsd = 162,
    fmadd_d = 163,
    fmsub_d = 164,
    fnmsub_d = 165,
    fnmadd_d = 166,
    fadd_d = 167,
    fsub_d = 168,
    fmul_d = 169,
    fdiv_d = 170,
    fsgnj_d = 171,
    fsgnjn_d = 172,
    fsgnjx_d = 173,
    fmin_d = 174,
    fmax_d = 175,
    fcvt_s_d = 176,
    fcvt_d_s = 177,
    fsqrt_d = 178,
    fle_d = 179,
    flt_d = 180,
    feq_d = 181,
    fcvt_w_d = 182,
    fcvt_wu_d = 183,
    fcvt_d_w = 184,
    fcvt_d_wu = 185,
    fclass_d = 186,
    fcvt_l_d = 187,
    fcvt_lu_d = 188,
    fmv_x_d = 189,
    fcvt_d_l = 190,
    fcvt_d_lu = 191,
    fmv_d_x = 192,
    flq = 193,
    fsq = 194,
    fmadd_q = 195,
    fmsub_q = 196,
    fnmsub_q = 197,
    fnmadd_q = 198,
    fadd_q = 199,
    fsub_q = 200,
    fmul_q = 201,
    fdiv_q = 202,
    fsgnj_q = 203,
    fsgnjn_q = 204,
    fsgnjx_q = 205,
    fmin_q = 206,
    fmax_q = 207,
    fcvt_s_q = 208,
    fcvt_q_s = 209,
    fcvt_d_q = 210,
    fcvt_q_d = 211,
    fsqrt_q = 212,
    fle_q = 213,
    flt_q = 214,
    feq_q = 215,
    fcvt_w_q = 216,
    fcvt_wu_q = 217,
    fcvt_q_w = 218,
    fcvt_q_wu = 219,
    fclass_q = 220,
    fcvt_l_q = 221,
    fcvt_lu_q = 222,
    fcvt_q_l = 223,
    fcvt_q_lu = 224,
    fmv_x_q = 225,
    fmv_q_x = 226,
    c_addi4spn = 227,
    c_fld = 228,
    c_lw = 229,
    c_flw = 230,
    c_fsd = 231,
    c_sw = 232,
    c_fsw = 233,
    c_nop = 234,
    c_addi = 235,
    c_jal = 236,
    c_li = 237,
    c_addi16sp = 238,
    c_lui = 239,
    c_srli = 240,
    c_srai = 241,
    c_andi = 242,
    c_sub = 243,
    c_xor = 244,
    c_or = 245,
    c_and = 246,
    c_subw = 247,
    c_addw = 248,
    c_j = 249,
    c_beqz = 250,
    c_bnez = 251,
    c_slli = 252,
    c_fldsp = 253,
    c_lwsp = 254,
    c_flwsp = 255,
    c_jr = 256,
    c_mv = 257,
    c_ebreak = 258,
    c_jalr = 259,
    c_add = 260,
    c_fsdsp = 261,
    c_swsp = 262,
    c_fswsp = 263,
    c_ld = 264,
    c_sd = 265,
    c_addiw = 266,
    c_ldsp = 267,
    c_sdsp = 268,
    c_lq = 269,
    c_sq = 270,
    c_lqsp = 271,
    c_sqsp = 272,
    nop = 273,
    mv = 274,
    not = 275,
    neg = 276,
    negw = 277,
    sext_w = 278,
    seqz = 279,
    snez = 280,
    sltz = 281,
    sgtz = 282,
    fmv_s = 283,
    fabs_s = 284,
    fneg_s = 285,
    fmv_d = 286,
    fabs_d = 287,
    fneg_d = 288,
    fmv_q = 289,
    fabs_q = 290,
    fneg_q = 291,
    beqz = 292,
    bnez = 293,
    blez = 294,
    bgez = 295,
    bltz = 296,
    bgtz = 297,
    ble = 298,
    bleu = 299,
    bgt = 300,
    bgtu = 301,
    j = 302,
    ret = 303,
    jr = 304,
    rdcycle = 305,
    rdtime = 306,
    rdinstret = 307,
    rdcycleh = 308,
    rdtimeh = 309,
    rdinstreth = 310,
    frcsr = 311,
    frrm = 312,
    frflags = 313,
    fscsr = 314,
    fsrm = 315,
    fsflags = 316,
    fsrmi = 317,
    fsflagsi = 318,
}

/* structures */

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct rv_decode {
    pub pc: u64,
    pub inst: u64,
    pub imm: i32,
    pub op: rv_op,
    pub codec: u8,
    pub rd: u8,
    pub rs1: u8,
    pub rs2: u8,
    pub rs3: u8,
    pub rm: u8,
    pub pred: u8,
    pub succ: u8,
    pub aq: u8,
    pub rl: u8,
}


