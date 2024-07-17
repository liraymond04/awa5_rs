pub mod awasm {
    use crate::{Awatism, Instruction};

    pub fn parse_lines(lines: impl Iterator<Item = String>) -> Vec<Instruction> {
        lines.filter_map(|line| parse_line(&line)).collect()
    }

    pub fn parse_line(line: &str) -> Option<Instruction> {
        let trimmed = line.trim();
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        if tokens.is_empty() || trimmed.starts_with(';') {
            return None;
        }

        let awatism = match tokens[0] {
            "nop" if tokens.len() == 1 => Awatism::Nop,
            "prn" if tokens.len() == 1 => Awatism::Prn,
            "pr1" if tokens.len() == 1 => Awatism::Pr1,
            "red" if tokens.len() == 1 => Awatism::Red,
            "r3d" if tokens.len() == 1 => Awatism::R3d,
            "blo" if tokens.len() == 2 => Awatism::Blo(tokens[1].parse().ok()?),
            "sbm" if tokens.len() == 2 => Awatism::Sbm(tokens[1].parse().ok()?),
            "pop" if tokens.len() == 1 => Awatism::Pop,
            "dpl" if tokens.len() == 1 => Awatism::Dpl,
            "srn" if tokens.len() == 2 => Awatism::Srn(tokens[1].parse().ok()?),
            "mrg" if tokens.len() == 1 => Awatism::Mrg,
            "4dd" if tokens.len() == 1 => Awatism::Add,
            "sub" if tokens.len() == 1 => Awatism::Sub,
            "mul" if tokens.len() == 1 => Awatism::Mul,
            "div" if tokens.len() == 1 => Awatism::Div,
            "cnt" if tokens.len() == 1 => Awatism::Cnt,
            "lbl" if tokens.len() == 2 => Awatism::Lbl(tokens[1].parse().ok()?),
            "jmp" if tokens.len() == 2 => Awatism::Jmp(tokens[1].parse().ok()?),
            "eql" if tokens.len() == 1 => Awatism::Eql,
            "lss" if tokens.len() == 1 => Awatism::Lss,
            "gr8" if tokens.len() == 1 => Awatism::Gr8,
            "trm" if tokens.len() == 1 => Awatism::Trm,
            _ => return None,
        };

        Some(Instruction { awatism })
    }
}

pub mod awatalk {
    use crate::{Awatism, Instruction};

    pub fn parse_string(content: &str) -> Vec<Instruction> {
        let mut instructions = Vec::new();

        let normalized = content
            .chars()
            .filter(|&c| c == 'a' || c == 'w' || c == ' ')
            .collect::<String>()
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ");

        if !normalized.starts_with("awa") {
            panic!("Not valid awatalk");
        }
        let mut remaining = &normalized[3..];

        let mapping = [(" awa", '0'), ("wa", '1')];

        let mut binary_str = String::new();
        let mut found_op = false;
        let mut num_bits = 0;
        let mut op_index = 0;
        let mut arg_index = 0;
        let mut current_index = 0;
        let mut arg_opcode = 0x00;
        while !remaining.is_empty() {
            let mut matched = false;
            let mut opcode = 0x00;
            for &(pattern, bit) in &mapping {
                if remaining.starts_with(pattern) {
                    binary_str.push(bit);
                    remaining = &remaining[pattern.len()..];
                    matched = true;
                    current_index += 1;

                    // finish reading args, add op code instruction
                    if found_op && current_index - arg_index == num_bits {
                        let op_str = &binary_str[arg_index..current_index];
                        let arg = u8::from_str_radix(op_str, 2).unwrap();
                        instructions.push(Instruction {
                            awatism: Awatism::from_u8(arg_opcode, arg).unwrap(),
                        });
                        op_index = current_index;
                        found_op = false;
                    }

                    // found op code
                    if !found_op && current_index - op_index == 5 {
                        let op_str = &binary_str[op_index..current_index];
                        opcode = u8::from_str_radix(op_str, 2).unwrap();
                        op_index = current_index;
                        found_op = true;
                    }

                    // set start of arg check
                    if found_op && Awatism::needs_args(opcode) {
                        arg_index = current_index;
                        arg_opcode = opcode;
                        num_bits = Awatism::arg_bits(opcode);
                    }

                    // does op code not needs args and continue checking, add instruction
                    if found_op && op_index == current_index && !Awatism::needs_args(opcode) {
                        instructions.push(Instruction {
                            awatism: Awatism::from_u8(opcode, 0x00).unwrap(),
                        });
                        found_op = false;
                    }

                    break;
                }
            }
            if !matched {
                break; // invalid awa code
            }
        }

        instructions
    }
}
