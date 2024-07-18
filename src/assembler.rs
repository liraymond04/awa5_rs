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

pub fn object_to_awasm(vec: &Vec<u8>) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < vec.len() {
        let op = vec[i];
        let arg = vec[i + 1];
        match Awatism::from_u8(op, arg).unwrap() {
            Awatism::Nop => {
                result += "nop";
            }
            Awatism::Prn => {
                result += "prn";
            }
            Awatism::Pr1 => {
                result += "pr1";
            }
            Awatism::Red => {
                result += "red";
            }
            Awatism::R3d => {
                result += "r3d";
            }
            Awatism::Blo(arg) => {
                result += &format!("blo {}", arg);
            }
            Awatism::Sbm(arg) => {
                result += &format!("sbm {}", arg);
            }
            Awatism::Pop => {
                result += "pop";
            }
            Awatism::Dpl => {
                result += "dpl";
            }
            Awatism::Srn(arg) => {
                result += &format!("srn {}", arg);
            }
            Awatism::Mrg => {
                result += "mrg";
            }
            Awatism::Add => {
                result += "4dd";
            }
            Awatism::Sub => {
                result += "sub";
            }
            Awatism::Mul => {
                result += "mul";
            }
            Awatism::Div => {
                result += "div";
            }
            Awatism::Cnt => {
                result += "cnt";
            }
            Awatism::Lbl(arg) => {
                result += &format!("lbl {}", arg);
            }
            Awatism::Jmp(arg) => {
                result += &format!("jmp {}", arg);
            }
            Awatism::Eql => {
                result += "eql";
            }
            Awatism::Lss => {
                result += "lss";
            }
            Awatism::Gr8 => {
                result += "gr8";
            }
            Awatism::Trm => {
                result += "trm";
            }
        }
        result += "\n";
        i += 2;
    }
    result
}

pub fn object_to_awa(vec: &Vec<u8>) -> String {
    let mut result = String::from("awa");
    let mut i = 0;
    while i < vec.len() {
        let op = vec[i];
        let arg = vec[i + 1];

        result += &to_mapping_op(op);

        if Awatism::needs_args(op) {
            result += &to_mapping_arg(arg, Awatism::arg_bits(op));
        }

        i += 2;
    }
    result += "\n";
    result
}

fn to_mapping_op(num: u8) -> String {
    let mapping = [" awa", "wa"];
    let mut result = String::new();

    let start_pos = 8 - 5;
    for i in start_pos..8 {
        let bit = (num >> (7 - i)) & 1;
        result += mapping[bit as usize];
    }
    result
}

fn to_mapping_arg(num: u8, size: usize) -> String {
    let mapping = [" awa", "wa"];
    let mut result = String::new();

    let start_pos = 8 - size;
    for i in start_pos..8 {
        let bit = (num >> (7 - i)) & 1;
        result += mapping[bit as usize];
    }
    result
}
