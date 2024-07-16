use crate::{Awatism, Instruction};

pub fn assemble_awatism(instruction: &Instruction) -> Vec<u8> {
    match instruction.awatism {
        Awatism::Nop => {
            let bytes = vec![0x01, 0x00];
            bytes
        }
        Awatism::Prn => {
            let bytes = vec![0x01, 0x00];
            bytes
        }
        Awatism::Pr1 => {
            let bytes = vec![0x02, 0x00];
            bytes
        }
        Awatism::Red => {
            let bytes = vec![0x03, 0x00];
            bytes
        }
        Awatism::R3d => {
            let bytes = vec![0x04, 0x00];
            bytes
        }
        Awatism::Blo(arg) => {
            let bytes = vec![0x05, arg as u8];
            bytes
        }
        Awatism::Sbm(arg) => {
            let bytes = vec![0x06, arg as u8];
            bytes
        }
        Awatism::Pop => {
            let bytes = vec![0x07, 0x00];
            bytes
        }
        Awatism::Dpl => {
            let bytes = vec![0x08, 0x00];
            bytes
        }
        Awatism::Srn(arg) => {
            let bytes = vec![0x09, arg as u8];
            bytes
        }
        Awatism::Mrg => {
            let bytes = vec![0x0A, 0x00];
            bytes
        }
        Awatism::Add => {
            let bytes = vec![0x0B, 0x00];
            bytes
        }
        Awatism::Sub => {
            let bytes = vec![0x0C, 0x00];
            bytes
        }
        Awatism::Mul => {
            let bytes = vec![0x0D, 0x00];
            bytes
        }
        Awatism::Div => {
            let bytes = vec![0x0E, 0x00];
            bytes
        }
        Awatism::Cnt => {
            let bytes = vec![0x0F, 0x00];
            bytes
        }
        Awatism::Lbl(arg) => {
            let bytes = vec![0x10, arg as u8];
            bytes
        }
        Awatism::Jmp(arg) => {
            let bytes = vec![0x11, arg as u8];
            bytes
        }
        Awatism::Eql => {
            let bytes = vec![0x12, 0x00];
            bytes
        }
        Awatism::Lss => {
            let bytes = vec![0x13, 0x00];
            bytes
        }
        Awatism::Gr8 => {
            let bytes = vec![0x14, 0x00];
            bytes
        }
        Awatism::Trm => {
            let bytes = vec![0x1F, 0x00];
            bytes
        }
    }
}

pub fn make_object_vec(instructions: &[Instruction]) -> Vec<u8> {
    let mut vec = Vec::new();

    for instruction in instructions {
        vec.extend(assemble_awatism(&instruction));
    }

    vec
}
