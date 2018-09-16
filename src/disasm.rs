use types::*;

#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct rv_comp_data {
    pub op: rv_op,
    pub constraints: &'static [rvc_constraint],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum rvcd_imm {
    none = 0x0,
    nz = 0x1,
    nz_hint = 0x2,
}

pub struct rv_opcode_data {
    pub name: &'static str,
    pub coded: rv_codec,
    pub format: &'static str,
    pub pseudo: rv_comp_data,
    pub decomp_rv32: rv_op,
    pub decomp_rv64: rv_op,
    pub decomp_rv128: rv_op,
    pub decomp_data: rvcd_imm,
}

/* register names */
const rv_ireg_name_sym: &[&str] = &[
    "zero", "ra",   "sp",   "gp",   "tp",   "t0",   "t1",   "t2",
    "s0",   "s1",   "a0",   "a1",   "a2",   "a3",   "a4",   "a5",
    "a6",   "a7",   "s2",   "s3",   "s4",   "s5",   "s6",   "s7",
    "s8",   "s9",   "s10",  "s11",  "t3",   "t4",   "t5",   "t6",
];

const rv_freg_name_sym: &[&str] = &[
    "ft0",  "ft1",  "ft2",  "ft3",  "ft4",  "ft5",  "ft6",  "ft7",
    "fs0",  "fs1",  "fa0",  "fa1",  "fa2",  "fa3",  "fa4",  "fa5",
    "fa6",  "fa7",  "fs2",  "fs3",  "fs4",  "fs5",  "fs6",  "fs7",
    "fs8",  "fs9",  "fs10", "fs11", "ft8",  "ft9",  "ft10", "ft11",
];

/* instruction formats */

const rv_fmt_none: &str                 = "O\t";
const rv_fmt_rs1: &str                  = "O\t1";
const rv_fmt_offset: &str               = "O\to";
const rv_fmt_pred_succ: &str            = "O\tp,s";
const rv_fmt_rs1_rs2: &str              = "O\t1,2";
const rv_fmt_rd_imm: &str               = "O\t0,i";
const rv_fmt_rd_offset: &str            = "O\t0,o";
const rv_fmt_rd_rs1_rs2: &str           = "O\t0,1,2";
const rv_fmt_frd_rs1: &str              = "O\t3,1";
const rv_fmt_rd_frs1: &str              = "O\t0,4";
const rv_fmt_rd_frs1_frs2: &str         = "O\t0,4,5";
const rv_fmt_frd_frs1_frs2: &str        = "O\t3,4,5";
const rv_fmt_rm_frd_frs1: &str          = "O\tr,3,4";
const rv_fmt_rm_frd_rs1: &str           = "O\tr,3,1";
const rv_fmt_rm_rd_frs1: &str           = "O\tr,0,4";
const rv_fmt_rm_frd_frs1_frs2: &str     = "O\tr,3,4,5";
const rv_fmt_rm_frd_frs1_frs2_frs: &str = "O\tr,3,4,5,6";
const rv_fmt_rd_rs1_imm: &str           = "O\t0,1,i";
const rv_fmt_rd_rs1_offset: &str        = "O\t0,1,i";
const rv_fmt_rd_offset_rs1: &str        = "O\t0,i(1)";
const rv_fmt_frd_offset_rs1: &str       = "O\t3,i(1)";
const rv_fmt_rd_csr_rs1: &str           = "O\t0,c,1";
const rv_fmt_rd_csr_zimm: &str          = "O\t0,c,7";
const rv_fmt_rs2_offset_rs1: &str       = "O\t2,i(1)";
const rv_fmt_frs2_offset_rs1: &str      = "O\t5,i(1)";
const rv_fmt_rs1_rs2_offset: &str       = "O\t1,2,o";
const rv_fmt_rs2_rs1_offset: &str       = "O\t2,1,o";
const rv_fmt_aqrl_rd_rs2_rs1: &str      = "OAR\t0,2,(1)";
const rv_fmt_aqrl_rd_rs1: &str          = "OAR\t0,(1)";
const rv_fmt_rd: &str                   = "O\t0";
const rv_fmt_rd_zimm: &str              = "O\t0,7";
const rv_fmt_rd_rs1: &str               = "O\t0,1";
const rv_fmt_rd_rs2: &str               = "O\t0,2";
const rv_fmt_rs1_offset: &str           = "O\t1,o";
const rv_fmt_rs2_offset: &str           = "O\t2,o";

/* pseudo-instruction constraints */
use types::rvc_constraint as rvcc;

const rvcc_null: &[rvcc] = &[ rvcc::end ];
const rvcc_jal: &[rvcc] = &[ rvcc::rd_eq_ra, rvcc::end ];
const rvcc_jalr: &[rvcc] = &[ rvcc::rd_eq_ra, rvcc::imm_eq_zero, rvcc::end ];
const rvcc_nop: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::rs1_eq_x0, rvcc::imm_eq_zero, rvcc::end ];
const rvcc_mv: &[rvcc] = &[ rvcc::imm_eq_zero, rvcc::end ];
const rvcc_not: &[rvcc] = &[ rvcc::imm_eq_n1, rvcc::end ];
const rvcc_neg: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::end ];
const rvcc_negw: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::end ];
const rvcc_sext_w: &[rvcc] = &[ rvcc::imm_eq_zero, rvcc::end ];
const rvcc_seqz: &[rvcc] = &[ rvcc::imm_eq_p1, rvcc::end ];
const rvcc_snez: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::end ];
const rvcc_sltz: &[rvcc] = &[ rvcc::rs2_eq_x0, rvcc::end ];
const rvcc_sgtz: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::end ];
const rvcc_fmv_s: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fabs_s: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fneg_s: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fmv_d: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fabs_d: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fneg_d: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fmv_q: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fabs_q: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_fneg_q: &[rvcc] = &[ rvcc::rs2_eq_rs1, rvcc::end ];
const rvcc_beqz: &[rvcc] = &[ rvcc::rs2_eq_x0, rvcc::end ];
const rvcc_bnez: &[rvcc] = &[ rvcc::rs2_eq_x0, rvcc::end ];
const rvcc_blez: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::end ];
const rvcc_bgez: &[rvcc] = &[ rvcc::rs2_eq_x0, rvcc::end ];
const rvcc_bltz: &[rvcc] = &[ rvcc::rs2_eq_x0, rvcc::end ];
const rvcc_bgtz: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::end ];
const rvcc_ble: &[rvcc] = &[ rvcc::end ];
const rvcc_bleu: &[rvcc] = &[ rvcc::end ];
const rvcc_bgt: &[rvcc] = &[ rvcc::end ];
const rvcc_bgtu: &[rvcc] = &[ rvcc::end ];
const rvcc_j: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::end ];
const rvcc_ret: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::rs1_eq_ra, rvcc::end ];
const rvcc_jr: &[rvcc] = &[ rvcc::rd_eq_x0, rvcc::imm_eq_zero, rvcc::end ];
const rvcc_rdcycle: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc00, rvcc::end ];
const rvcc_rdtime: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc01, rvcc::end ];
const rvcc_rdinstret: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc02, rvcc::end ];
const rvcc_rdcycleh: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc80, rvcc::end ];
const rvcc_rdtimeh: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc81, rvcc::end ];
const rvcc_rdinstreth: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0xc80, rvcc::end ];
const rvcc_frcsr: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0x003, rvcc::end ];
const rvcc_frrm: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0x002, rvcc::end ];
const rvcc_frflags: &[rvcc] = &[ rvcc::rs1_eq_x0, rvcc::csr_eq_0x001, rvcc::end ];
const rvcc_fscsr: &[rvcc] = &[ rvcc::csr_eq_0x003, rvcc::end ];
const rvcc_fsrm: &[rvcc] = &[ rvcc::csr_eq_0x002, rvcc::end ];
const rvcc_fsflags: &[rvcc] = &[ rvcc::csr_eq_0x001, rvcc::end ];
const rvcc_fsrmi: &[rvcc] = &[ rvcc::csr_eq_0x002, rvcc::end ];
const rvcc_fsflagsi: &[rvcc] = &[ rvcc::csr_eq_0x001, rvcc::end ];

/* pseudo-instruction metadata */

const rvcp_jal: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::j, constraints: rvcc_j },
    rv_comp_data { op: rv_op::jal, constraints: rvcc_jal },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_jalr: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::ret, constraints: rvcc_ret },
    rv_comp_data { op: rv_op::jr, constraints: rvcc_jr },
    rv_comp_data { op: rv_op::jalr, constraints: rvcc_jalr },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_beq: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::beqz, constraints: rvcc_beqz },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_bne: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bnez, constraints: rvcc_bnez },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_blt: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bltz, constraints: rvcc_bltz },
    rv_comp_data { op: rv_op::bgtz, constraints: rvcc_bgtz },
    rv_comp_data { op: rv_op::bgt, constraints: rvcc_bgt },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_bge: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::blez, constraints: rvcc_blez },
    rv_comp_data { op: rv_op::bgez, constraints: rvcc_bgez },
    rv_comp_data { op: rv_op::ble, constraints: rvcc_ble },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_bltu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bgtu, constraints: rvcc_bgtu },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_bgeu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::bleu, constraints: rvcc_bleu },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_addi: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::nop, constraints: rvcc_nop },
    rv_comp_data { op: rv_op::mv, constraints: rvcc_mv },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_sltiu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::seqz, constraints: rvcc_seqz },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_xori: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::not, constraints: rvcc_not },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_sub: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::neg, constraints: rvcc_neg },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_slt: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::sltz, constraints: rvcc_sltz },
    rv_comp_data { op: rv_op::sgtz, constraints: rvcc_sgtz },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_sltu: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::snez, constraints: rvcc_snez },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_addiw: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::sext_w, constraints: rvcc_sext_w },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_subw: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::negw, constraints: rvcc_negw },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_csrrw: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fscsr, constraints: rvcc_fscsr },
    rv_comp_data { op: rv_op::fsrm, constraints: rvcc_fsrm },
    rv_comp_data { op: rv_op::fsflags, constraints: rvcc_fsflags },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_csrrs: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::rdcycle, constraints: rvcc_rdcycle },
    rv_comp_data { op: rv_op::rdtime, constraints: rvcc_rdtime },
    rv_comp_data { op: rv_op::rdinstret, constraints: rvcc_rdinstret },
    rv_comp_data { op: rv_op::rdcycleh, constraints: rvcc_rdcycleh },
    rv_comp_data { op: rv_op::rdtimeh, constraints: rvcc_rdtimeh },
    rv_comp_data { op: rv_op::rdinstreth, constraints: rvcc_rdinstreth },
    rv_comp_data { op: rv_op::frcsr, constraints: rvcc_frcsr },
    rv_comp_data { op: rv_op::frrm, constraints: rvcc_frrm },
    rv_comp_data { op: rv_op::frflags, constraints: rvcc_frflags },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_csrrwi: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fsrmi, constraints: rvcc_fsrmi },
    rv_comp_data { op: rv_op::fsflagsi, constraints: rvcc_fsflagsi },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnj_s: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fmv_s, constraints: rvcc_fmv_s },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnjn_s: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fneg_s, constraints: rvcc_fneg_s },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnjx_s: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fabs_s, constraints: rvcc_fabs_s },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnj_d: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fmv_d, constraints: rvcc_fmv_d },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnjn_d: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fneg_d, constraints: rvcc_fneg_d },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnjx_d: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fabs_d, constraints: rvcc_fabs_d },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnj_q: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fmv_q, constraints: rvcc_fmv_q },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnjn_q: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fneg_q, constraints: rvcc_fneg_q },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

const rvcp_fsgnjx_q: &[rv_comp_data] = &[
    rv_comp_data { op: rv_op::fabs_q, constraints: rvcc_fabs_q },
    rv_comp_data { op: rv_op::illegal, constraints: rvcc_null }
];

/* CSR names */

fn csr_name(csrno: i32) -> Option<&'static str> {
    match csrno {
        0x0000 => Some("ustatus"),
        0x0001 => Some("fflags"),
        0x0002 => Some("frm"),
        0x0003 => Some("fcsr"),
        0x0004 => Some("uie"),
        0x0005 => Some("utvec"),
        0x0040 => Some("uscratch"),
        0x0041 => Some("uepc"),
        0x0042 => Some("ucause"),
        0x0043 => Some("utval"),
        0x0044 => Some("uip"),
        0x0100 => Some("sstatus"),
        0x0102 => Some("sedeleg"),
        0x0103 => Some("sideleg"),
        0x0104 => Some("sie"),
        0x0105 => Some("stvec"),
        0x0106 => Some("scounteren"),
        0x0140 => Some("sscratch"),
        0x0141 => Some("sepc"),
        0x0142 => Some("scause"),
        0x0143 => Some("stval"),
        0x0144 => Some("sip"),
        0x0180 => Some("satp"),
        0x0200 => Some("hstatus"),
        0x0202 => Some("hedeleg"),
        0x0203 => Some("hideleg"),
        0x0204 => Some("hie"),
        0x0205 => Some("htvec"),
        0x0240 => Some("hscratch"),
        0x0241 => Some("hepc"),
        0x0242 => Some("hcause"),
        0x0243 => Some("hbadaddr"),
        0x0244 => Some("hip"),
        0x0300 => Some("mstatus"),
        0x0301 => Some("misa"),
        0x0302 => Some("medeleg"),
        0x0303 => Some("mideleg"),
        0x0304 => Some("mie"),
        0x0305 => Some("mtvec"),
        0x0306 => Some("mcounteren"),
        0x0320 => Some("mucounteren"),
        0x0321 => Some("mscounteren"),
        0x0322 => Some("mhcounteren"),
        0x0323 => Some("mhpmevent3"),
        0x0324 => Some("mhpmevent4"),
        0x0325 => Some("mhpmevent5"),
        0x0326 => Some("mhpmevent6"),
        0x0327 => Some("mhpmevent7"),
        0x0328 => Some("mhpmevent8"),
        0x0329 => Some("mhpmevent9"),
        0x032a => Some("mhpmevent10"),
        0x032b => Some("mhpmevent11"),
        0x032c => Some("mhpmevent12"),
        0x032d => Some("mhpmevent13"),
        0x032e => Some("mhpmevent14"),
        0x032f => Some("mhpmevent15"),
        0x0330 => Some("mhpmevent16"),
        0x0331 => Some("mhpmevent17"),
        0x0332 => Some("mhpmevent18"),
        0x0333 => Some("mhpmevent19"),
        0x0334 => Some("mhpmevent20"),
        0x0335 => Some("mhpmevent21"),
        0x0336 => Some("mhpmevent22"),
        0x0337 => Some("mhpmevent23"),
        0x0338 => Some("mhpmevent24"),
        0x0339 => Some("mhpmevent25"),
        0x033a => Some("mhpmevent26"),
        0x033b => Some("mhpmevent27"),
        0x033c => Some("mhpmevent28"),
        0x033d => Some("mhpmevent29"),
        0x033e => Some("mhpmevent30"),
        0x033f => Some("mhpmevent31"),
        0x0340 => Some("mscratch"),
        0x0341 => Some("mepc"),
        0x0342 => Some("mcause"),
        0x0343 => Some("mtval"),
        0x0344 => Some("mip"),
        0x0380 => Some("mbase"),
        0x0381 => Some("mbound"),
        0x0382 => Some("mibase"),
        0x0383 => Some("mibound"),
        0x0384 => Some("mdbase"),
        0x0385 => Some("mdbound"),
        0x03a0 => Some("pmpcfg0"),
        0x03a1 => Some("pmpcfg1"),
        0x03a2 => Some("pmpcfg2"),
        0x03a3 => Some("pmpcfg3"),
        0x03b0 => Some("pmpaddr0"),
        0x03b1 => Some("pmpaddr1"),
        0x03b2 => Some("pmpaddr2"),
        0x03b3 => Some("pmpaddr3"),
        0x03b4 => Some("pmpaddr4"),
        0x03b5 => Some("pmpaddr5"),
        0x03b6 => Some("pmpaddr6"),
        0x03b7 => Some("pmpaddr7"),
        0x03b8 => Some("pmpaddr8"),
        0x03b9 => Some("pmpaddr9"),
        0x03ba => Some("pmpaddr10"),
        0x03bb => Some("pmpaddr11"),
        0x03bc => Some("pmpaddr12"),
        0x03bd => Some("pmpaddr14"),
        0x03be => Some("pmpaddr13"),
        0x03bf => Some("pmpaddr15"),
        0x0780 => Some("mtohost"),
        0x0781 => Some("mfromhost"),
        0x0782 => Some("mreset"),
        0x0783 => Some("mipi"),
        0x0784 => Some("miobase"),
        0x07a0 => Some("tselect"),
        0x07a1 => Some("tdata1"),
        0x07a2 => Some("tdata2"),
        0x07a3 => Some("tdata3"),
        0x07b0 => Some("dcsr"),
        0x07b1 => Some("dpc"),
        0x07b2 => Some("dscratch"),
        0x0b00 => Some("mcycle"),
        0x0b01 => Some("mtime"),
        0x0b02 => Some("minstret"),
        0x0b03 => Some("mhpmcounter3"),
        0x0b04 => Some("mhpmcounter4"),
        0x0b05 => Some("mhpmcounter5"),
        0x0b06 => Some("mhpmcounter6"),
        0x0b07 => Some("mhpmcounter7"),
        0x0b08 => Some("mhpmcounter8"),
        0x0b09 => Some("mhpmcounter9"),
        0x0b0a => Some("mhpmcounter10"),
        0x0b0b => Some("mhpmcounter11"),
        0x0b0c => Some("mhpmcounter12"),
        0x0b0d => Some("mhpmcounter13"),
        0x0b0e => Some("mhpmcounter14"),
        0x0b0f => Some("mhpmcounter15"),
        0x0b10 => Some("mhpmcounter16"),
        0x0b11 => Some("mhpmcounter17"),
        0x0b12 => Some("mhpmcounter18"),
        0x0b13 => Some("mhpmcounter19"),
        0x0b14 => Some("mhpmcounter20"),
        0x0b15 => Some("mhpmcounter21"),
        0x0b16 => Some("mhpmcounter22"),
        0x0b17 => Some("mhpmcounter23"),
        0x0b18 => Some("mhpmcounter24"),
        0x0b19 => Some("mhpmcounter25"),
        0x0b1a => Some("mhpmcounter26"),
        0x0b1b => Some("mhpmcounter27"),
        0x0b1c => Some("mhpmcounter28"),
        0x0b1d => Some("mhpmcounter29"),
        0x0b1e => Some("mhpmcounter30"),
        0x0b1f => Some("mhpmcounter31"),
        0x0b80 => Some("mcycleh"),
        0x0b81 => Some("mtimeh"),
        0x0b82 => Some("minstreth"),
        0x0b83 => Some("mhpmcounter3h"),
        0x0b84 => Some("mhpmcounter4h"),
        0x0b85 => Some("mhpmcounter5h"),
        0x0b86 => Some("mhpmcounter6h"),
        0x0b87 => Some("mhpmcounter7h"),
        0x0b88 => Some("mhpmcounter8h"),
        0x0b89 => Some("mhpmcounter9h"),
        0x0b8a => Some("mhpmcounter10h"),
        0x0b8b => Some("mhpmcounter11h"),
        0x0b8c => Some("mhpmcounter12h"),
        0x0b8d => Some("mhpmcounter13h"),
        0x0b8e => Some("mhpmcounter14h"),
        0x0b8f => Some("mhpmcounter15h"),
        0x0b90 => Some("mhpmcounter16h"),
        0x0b91 => Some("mhpmcounter17h"),
        0x0b92 => Some("mhpmcounter18h"),
        0x0b93 => Some("mhpmcounter19h"),
        0x0b94 => Some("mhpmcounter20h"),
        0x0b95 => Some("mhpmcounter21h"),
        0x0b96 => Some("mhpmcounter22h"),
        0x0b97 => Some("mhpmcounter23h"),
        0x0b98 => Some("mhpmcounter24h"),
        0x0b99 => Some("mhpmcounter25h"),
        0x0b9a => Some("mhpmcounter26h"),
        0x0b9b => Some("mhpmcounter27h"),
        0x0b9c => Some("mhpmcounter28h"),
        0x0b9d => Some("mhpmcounter29h"),
        0x0b9e => Some("mhpmcounter30h"),
        0x0b9f => Some("mhpmcounter31h"),
        0x0c00 => Some("cycle"),
        0x0c01 => Some("time"),
        0x0c02 => Some("instret"),
        0x0c80 => Some("cycleh"),
        0x0c81 => Some("timeh"),
        0x0c82 => Some("instreth"),
        0x0d00 => Some("scycle"),
        0x0d01 => Some("stime"),
        0x0d02 => Some("sinstret"),
        0x0d80 => Some("scycleh"),
        0x0d81 => Some("stimeh"),
        0x0d82 => Some("sinstreth"),
        0x0e00 => Some("hcycle"),
        0x0e01 => Some("htime"),
        0x0e02 => Some("hinstret"),
        0x0e80 => Some("hcycleh"),
        0x0e81 => Some("htimeh"),
        0x0e82 => Some("hinstreth"),
        0x0f11 => Some("mvendorid"),
        0x0f12 => Some("marchid"),
        0x0f13 => Some("mimpid"),
        0x0f14 => Some("mhartid"),
        _ => None,
    }
}

/* decode opcode */

fn decode_inst_opcode(inst: u64, isa: rv_isa) -> rv_op {
    use types::rv_isa::*;
    match (inst >> 0) & 0b11 {
    0 => {
        match (inst >> 13) & 0b111 {
            0 => rv_op::c_addi4spn,
            1 => {
                if isa == rv128 {
                    rv_op::c_lq
                } else {
                    rv_op::c_fld
                }
            },
            2 => rv_op::c_lw,
            3 => {
                if isa == rv32 {
                    rv_op::c_flw
                } else {
                    rv_op::c_ld
                }
            },
            5 => {
                if isa == rv128 {
                    rv_op::c_sq
                } else {
                    rv_op::c_fsd
                }
            },
            6 => rv_op::c_sw,
            7 => {
                if isa == rv32 {
                    rv_op::c_fsw
                } else {
                    rv_op::c_sd
                }
            },
            _ => { rv_op::illegal },
        }
    },
    1 => {
        match (inst >> 13) & 0b111 {
            0 => {
                match (inst >> 2) & 0b11111111111 {
                    0 => rv_op::c_nop,
                    _ => rv_op::c_addi,
                }
            },
            1 => {
                if isa == rv32 {
                    rv_op::c_jal
                } else {
                    rv_op::c_addiw
                }
            },   
            2 => { rv_op::c_li },
            3 => {
                match (inst >> 7) & 0b11111 {
                    2 => rv_op::c_addi16sp,
                    _ => rv_op::c_lui,
                }
            },    
            4 => {
                match (inst >> 10) & 0b11 {
                    0 => rv_op::c_srli,
                    1 => rv_op::c_srai,
                    2 => rv_op::c_andi,
                    3 => {
                        match ((inst >> 10) & 0b100) | ((inst >> 5) & 0b011) {
                            0 => rv_op::c_sub,
                            1 => rv_op::c_xor,
                            2 => rv_op::c_or,
                            3 => rv_op::c_and,
                            4 => rv_op::c_subw,
                            5 => rv_op::c_addw,
                            _ => rv_op::illegal,
                        }
                    },
                    _ => rv_op::illegal,
                }
            },
            5 => rv_op::c_j,
            6 => rv_op::c_beqz,
            7 => rv_op::c_bnez,
            _ => rv_op::illegal,
        }
    },
    2 => {
        match (inst >> 13) & 0b111 {
            0 => rv_op::c_slli,
            1 => {
                if isa == rv128 {
                    rv_op::c_lqsp
                } else {
                    rv_op::c_fldsp
                }
            },
            2 => rv_op::c_lwsp,
            3 => {
                if isa == rv32 {
                    rv_op::c_flwsp
                } else {
                    rv_op::c_ldsp
                }
            },
            4 => {
                match (inst >> 12) & 0b1 {
                    0 => {
                        match (inst >> 2) & 0b11111 {
                            0 => rv_op::c_jr,
                            _ => rv_op::c_mv,
                        }
                    },
                    1 => {
                        match (inst >> 2) & 0b11111 {
                            0 => {
                                match (inst >> 7) & 0b11111 {
                                    0 => rv_op::c_ebreak,
                                    _ => rv_op::c_jalr,
                                }
                            },
                            _ => rv_op::c_add,
                        }
                    }
                    _ => rv_op::illegal,
                }
            },
            5 => {
                if isa == rv128 {
                    rv_op::c_sqsp
                } else {
                    rv_op::c_fsdsp
                }
            },
            6 => rv_op::c_swsp,
            7 => {
                if isa == rv32 {
                    rv_op::c_fswsp
                } else {
                    rv_op::c_sdsp
                }
            },
            _ => rv_op::illegal,
        }
    },
    3 => {
        match (inst >> 2) & 0b11111 {
        0 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::lb,
            1 => rv_op::lh,
            2 => rv_op::lw,
            3 => rv_op::ld,
            4 => rv_op::lbu,
            5 => rv_op::lhu,
            6 => rv_op::lwu,
            7 => rv_op::ldu,
            _ => rv_op::illegal,
            }
        },           
        1 => {
            match (inst >> 12) & 0b111 {
            2 => rv_op::flw,
            3 => rv_op::fld,
            4 => rv_op::flq,
            _ => rv_op::illegal,
            }
        },
        3 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::fence,
            1 => rv_op::fence_i,
            2 => rv_op::lq,
            _ => rv_op::illegal,
            }
        },           
        4 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::addi,
            1 => {
                match (inst >> 27) & 0b11111 {
                0 => rv_op::slli,
                _ => rv_op::illegal,
                }
            },
            2 => rv_op::slti,
            3 => rv_op::sltiu,
            4 => rv_op::xori,
            5 => {
                match (inst >> 27) & 0b11111 {
                0 => rv_op::srli,
                8 => rv_op::srai,
                _ => rv_op::illegal,
                }
            },
            6 => rv_op::ori,
            7 => rv_op::andi,
            _ => rv_op::illegal,
            }
        },           
        5 => rv_op::auipc,
        6 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::addiw,
            1 => {
                match (inst >> 25) & 0b1111111 {
                0 => rv_op::slliw,
                _ => rv_op::illegal,
                }
            },
            5 => {
                match (inst >> 25) & 0b1111111 {
                0 => rv_op::srliw,
                32 => rv_op::sraiw,
                _ => rv_op::illegal,
                }
            },
            _ => rv_op::illegal,
            }
        },    
        8 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::sb,
            1 => rv_op::sh,
            2 => rv_op::sw,
            3 => rv_op::sd,
            4 => rv_op::sq,
            _ => rv_op::illegal,
            }
        },
        9 => {
            match (inst >> 12) & 0b111 {
            2 => rv_op::fsw,
            3 => rv_op::fsd,
            4 => rv_op::fsq,
            _ => rv_op::illegal,
            }
        },
        11 => {
            match ((inst >> 24) & 0b11111000) | ((inst >> 12) & 0b00000111) {
            2 => rv_op::amoadd_w,
            3 => rv_op::amoadd_d,
            4 => rv_op::amoadd_q,
            10 => rv_op::amoswap_w,
            11 => rv_op::amoswap_d,
            12 => rv_op::amoswap_q,
            18 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::lr_w,
                _ => rv_op::illegal,
                }
            },
            19 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::lr_d,
                _ => rv_op::illegal,
                }
            },
            20 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::lr_q,
                _ => rv_op::illegal,
                }
            },
            26 => rv_op::sc_w,
            27 => rv_op::sc_d,
            28 => rv_op::sc_q,
            34 => rv_op::amoxor_w,
            35 => rv_op::amoxor_d,
            36 => rv_op::amoxor_q,
            66 => rv_op::amoor_w,
            67 => rv_op::amoor_d,
            68 => rv_op::amoor_q,
            98 => rv_op::amoand_w,
            99 => rv_op::amoand_d,
            100 => rv_op::amoand_q,
            130 => rv_op::amomin_w,
            131 => rv_op::amomin_d,
            132 => rv_op::amomin_q,
            162 => rv_op::amomax_w,
            163 => rv_op::amomax_d,
            164 => rv_op::amomax_q,
            194 => rv_op::amominu_w,
            195 => rv_op::amominu_d,
            196 => rv_op::amominu_q,
            226 => rv_op::amomaxu_w,
            227 => rv_op::amomaxu_d,
            228 => rv_op::amomaxu_q,
            _ => rv_op::illegal,
            }
        },
        12 => {
            match ((inst >> 22) & 0b1111111000) | ((inst >> 12) & 0b0000000111) {
            0 => rv_op::add,
            1 => rv_op::sll,
            2 => rv_op::slt,
            3 => rv_op::sltu,
            4 => rv_op::xor,
            5 => rv_op::srl,
            6 => rv_op::or,
            7 => rv_op::and,
            8 => rv_op::mul,
            9 => rv_op::mulh,
            10 => rv_op::mulhsu,
            11 => rv_op::mulhu,
            12 => rv_op::div,
            13 => rv_op::divu,
            14 => rv_op::rem,
            15 => rv_op::remu,
            256 => rv_op::sub,
            261 => rv_op::sra,
            _ => rv_op::illegal,
            }
        },
        13 => rv_op::lui,
        14 => {
            match ((inst >> 22) & 0b1111111000) | ((inst >> 12) & 0b0000000111) {
            0 => rv_op::addw,
            1 => rv_op::sllw,
            5 => rv_op::srlw,
            8 => rv_op::mulw,
            12 => rv_op::divw,
            13 => rv_op::divuw,
            14 => rv_op::remw,
            15 => rv_op::remuw,
            256 => rv_op::subw,
            261 => rv_op::sraw,
            _ => rv_op::illegal,
            }
        },
        16 => {
            match (inst >> 25) & 0b11 {
            0 => rv_op::fmadd_s,
            1 => rv_op::fmadd_d,
            3 => rv_op::fmadd_q,
            _ => rv_op::illegal,
            }
        },
        17 => {
            match (inst >> 25) & 0b11 {
            0 => rv_op::fmsub_s,
            1 => rv_op::fmsub_d,
            3 => rv_op::fmsub_q,
            _ => rv_op::illegal,
            }
        },
        18 => {
            match (inst >> 25) & 0b11 {
            0 => rv_op::fnmsub_s,
            1 => rv_op::fnmsub_d,
            3 => rv_op::fnmsub_q,
            _ => rv_op::illegal,
            }
        },
        19 => {
            match (inst >> 25) & 0b11 {
            0 => rv_op::fnmadd_s,
            1 => rv_op::fnmadd_d,
            3 => rv_op::fnmadd_q,
            _ => rv_op::illegal,
            }
        },
        20 => {
            match (inst >> 25) & 0b1111111 {
            0 => rv_op::fadd_s,
            1 => rv_op::fadd_d,
            3 => rv_op::fadd_q,
            4 => rv_op::fsub_s,
            5 => rv_op::fsub_d,
            7 => rv_op::fsub_q,
            8 => rv_op::fmul_s,
            9 => rv_op::fmul_d,
            11 => rv_op::fmul_q,
            12 => rv_op::fdiv_s,
            13 => rv_op::fdiv_d,
            15 => rv_op::fdiv_q,
            16 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fsgnj_s,
                1 => rv_op::fsgnjn_s,
                2 => rv_op::fsgnjx_s,
                _ => rv_op::illegal,
                }
            },
            17 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fsgnj_d,
                1 => rv_op::fsgnjn_d,
                2 => rv_op::fsgnjx_d,
                _ => rv_op::illegal,
                }
            },
            19 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fsgnj_q,
                1 => rv_op::fsgnjn_q,
                2 => rv_op::fsgnjx_q,
                _ => rv_op::illegal,
                }
            },
            20 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fmin_s,
                1 => rv_op::fmax_s,
                _ => rv_op::illegal,
                }
            },
            21 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fmin_d,
                1 => rv_op::fmax_d,
                _ => rv_op::illegal,
                }
            },
            23 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fmin_q,
                1 => rv_op::fmax_q,
                _ => rv_op::illegal,
                }
            },
            32 => {
                match (inst >> 20) & 0b11111 {
                1 => rv_op::fcvt_s_d,
                3 => rv_op::fcvt_s_q,
                _ => rv_op::illegal,
                }
            },
            33 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_d_s,
                3 => rv_op::fcvt_d_q,
                _ => rv_op::illegal,
                }
            },
            35 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_q_s,
                1 => rv_op::fcvt_q_d,
                _ => rv_op::illegal,
                }
            },
            44 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fsqrt_s,
                _ => rv_op::illegal,
                }
            },
            45 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fsqrt_d,
                _ => rv_op::illegal,
                }
            },
            47 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fsqrt_q,
                _ => rv_op::illegal,
                }
            },
            80 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fle_s,
                1 => rv_op::flt_s,
                2 => rv_op::feq_s,
                _ => rv_op::illegal,
                }
            },
            81 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fle_d,
                1 => rv_op::flt_d,
                2 => rv_op::feq_d,
                _ => rv_op::illegal,
                }
            },
            83 => {
                match (inst >> 12) & 0b111 {
                0 => rv_op::fle_q,
                1 => rv_op::flt_q,
                2 => rv_op::feq_q,
                _ => rv_op::illegal,
                }
            },
            96 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_w_s,
                1 => rv_op::fcvt_wu_s,
                2 => rv_op::fcvt_l_s,
                3 => rv_op::fcvt_lu_s,
                _ => rv_op::illegal,
                }
            },
            97 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_w_d,
                1 => rv_op::fcvt_wu_d,
                2 => rv_op::fcvt_l_d,
                3 => rv_op::fcvt_lu_d,
                _ => rv_op::illegal,
                }
            },
            99 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_w_q,
                1 => rv_op::fcvt_wu_q,
                2 => rv_op::fcvt_l_q,
                3 => rv_op::fcvt_lu_q,
                _ => rv_op::illegal,
                }
            },
            104 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_s_w,
                1 => rv_op::fcvt_s_wu,
                2 => rv_op::fcvt_s_l,
                3 => rv_op::fcvt_s_lu,
                _ => rv_op::illegal,
                }
            },
            105 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_d_w,
                1 => rv_op::fcvt_d_wu,
                2 => rv_op::fcvt_d_l,
                3 => rv_op::fcvt_d_lu,
                _ => rv_op::illegal,
                }
            },
            107 => {
                match (inst >> 20) & 0b11111 {
                0 => rv_op::fcvt_q_w,
                1 => rv_op::fcvt_q_wu,
                2 => rv_op::fcvt_q_l,
                3 => rv_op::fcvt_q_lu,
                _ => rv_op::illegal,
                }
            },
            112 => {
                match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                0 => rv_op::fmv_x_s,
                1 => rv_op::fclass_s,
                _ => rv_op::illegal,
                }
            },
            113 => {
                match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                0 => rv_op::fmv_x_d,
                1 => rv_op::fclass_d,
                _ => rv_op::illegal,
                }
            },
            115 => {
                match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                0 => rv_op::fmv_x_q,
                1 => rv_op::fclass_q,
                _ => rv_op::illegal,
                }
            },
            120 => {
                match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                0 => rv_op::fmv_s_x,
                _ => rv_op::illegal,
                }
            },
            121 => {
                match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                0 => rv_op::fmv_d_x,
                _ => rv_op::illegal,
                }
            },
            123 => {
                match ((inst >> 17) & 0b11111000) | ((inst >> 12) & 0b00000111) {
                0 => rv_op::fmv_q_x,
                _ => rv_op::illegal,
                }
            },
            _ => rv_op::illegal,
            }
        },
        22 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::addid,
            1 => {
                match (inst >> 26) & 0b111111 {
                0 => rv_op::sllid,
                _ => rv_op::illegal,
                }
            },               
            5 => {
                match (inst >> 26) & 0b111111 {
                0 => rv_op::srlid,
                16 => rv_op::sraid,
                _ => rv_op::illegal,
                }
            },
            _ => rv_op::illegal,
            }
        },
        24 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::beq,
            1 => rv_op::bne,
            4 => rv_op::blt,
            5 => rv_op::bge,
            6 => rv_op::bltu,
            7 => rv_op::bgeu,
            _ => rv_op::illegal,
            }
        },
        25 => {
            match (inst >> 12) & 0b111 {
            0 => rv_op::jalr,
            _ => rv_op::illegal,
            }
        },
        27 => rv_op::jal,
        28 => {
            match (inst >> 12) & 0b111 {
            0 => {
                match ((inst >> 20) & 0b111111100000) | ((inst >> 7) & 0b000000011111) {
                0 => {
                    match (inst >> 15) & 0b1111111111 {
                    0 => rv_op::ecall,
                    32 => rv_op::ebreak,
                    64 => rv_op::uret,
                    _ => rv_op::illegal,
                    }
                },
                256 => {
                    match (inst >> 20) & 0b11111 {
                    2 => {
                        match (inst >> 15) & 0b11111 {
                        0 => rv_op::sret,
                        _ => rv_op::illegal,
                        }
                    }
                    4 => rv_op::sfence_vm,
                    5 => {
                        match (inst >> 15) & 0b11111 {
                        0 => rv_op::wfi,
                        _ => rv_op::illegal,
                        }
                    }
                    _ => rv_op::illegal,
                    }
                },
                288 => rv_op::sfence_vma,
                512 => {
                    match (inst >> 15) & 0b1111111111 {
                    64 => rv_op::hret,
                    _ => rv_op::illegal,
                    }
                },
                768 => {
                    match (inst >> 15) & 0b1111111111 {
                    64 => rv_op::mret,
                    _ => rv_op::illegal,
                    }
                },
                1952 => {
                    match (inst >> 15) & 0b1111111111 {
                    576 => rv_op::dret,
                    _ => rv_op::illegal,
                    }
                },
                _ => rv_op::illegal,
                }
            },
            1 => rv_op::csrrw,
            2 => rv_op::csrrs,
            3 => rv_op::csrrc,
            5 => rv_op::csrrwi,
            6 => rv_op::csrrsi,
            7 => rv_op::csrrci,
            _ => rv_op::illegal,
            }
        },
        30 => {
            match ((inst >> 22) & 0b1111111000) | ((inst >> 12) & 0b0000000111) {
            0 => rv_op::addd,
            1 => rv_op::slld,
            5 => rv_op::srld,
            8 => rv_op::muld,
            12 => rv_op::divd,
            13 => rv_op::divud,
            14 => rv_op::remd,
            15 => rv_op::remud,
            256 => rv_op::subd,
            261 => rv_op::srad,
            _ => rv_op::illegal,
            }
        },
        _ => rv_op::illegal,
        }
    },
    _ => rv_op::illegal,
    }
}
