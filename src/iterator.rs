use disasm::decode_inst_bytes;
use types::*;

#[derive(Debug, Clone)]
pub struct Disassembler<'a> {
    isa: rv_isa,
    slice: &'a [u8],
    ptr: usize,
    pc: u64,
}

impl<'a> Disassembler<'a> {
    pub fn new(isa: rv_isa, slice: &[u8], start_pc: u64) -> Disassembler {
        return Disassembler {
            isa: isa,
            slice: slice,
            ptr: 0,
            pc: start_pc,
        };
    }
}

impl<'a> Iterator for Disassembler<'a> {
    type Item = rv_decode;

    fn next(&mut self) -> Option<Self::Item> {
        let inst = self.slice.get(self.ptr..)?;
        let decoded = decode_inst_bytes(self.isa, self.pc, inst);
        if let Some(dec) = &decoded {
            let len = dec.len;
            self.ptr += len;
            self.pc += len as u64;
        }
        decoded
    }
}
