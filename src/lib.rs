#![crate_type = "lib"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
// Coding conventions
#![deny(missing_debug_implementations)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//#![deny(unused_mut)]
//#![warn(missing_docs)]

mod disasm;
mod format;
mod iterator;
mod opcode_data;
mod types;

pub use disasm::{decode_inst, decode_inst_bytes, inst_length, inst_length_first_byte};
pub use format::{disasm_inst, disasm_inst_bytes, format_inst, format_inst_opt};
pub use iterator::Disassembler;
pub use types::{
  rv_codec, rv_decode, rv_fence, rv_freg, rv_ireg, rv_isa, rv_op, rv_options, rv_rm,
};
