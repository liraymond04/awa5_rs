pub mod awasm {
    use crate::{Awatism, Instruction, AWA_SCII};

    pub fn parse_lines(lines: impl Iterator<Item = String>) -> Vec<Instruction> {
        lines.flat_map(|line| parse_line(&line)).collect()
    }

    pub fn parse_line(line: &str) -> Vec<Instruction> {
        let line_without_comments = line.split(';').next().unwrap_or("");
        let trimmed = line_without_comments.trim();
        let tokens: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();

        if tokens.is_empty() || trimmed.starts_with(';') {
            return vec![];
        }

        let awatism = match tokens[0] {
            "nop" if tokens.len() == 1 => vec![Awatism::Nop],
            "prn" if tokens.len() == 1 => vec![Awatism::Prn],
            "pr1" if tokens.len() == 1 => vec![Awatism::Pr1],
            "red" if tokens.len() == 1 => vec![Awatism::Red],
            "r3d" if tokens.len() == 1 => vec![Awatism::R3d],
            "blo" if tokens.len() == 2 => {
                let b: i32 = tokens[1].parse().ok().unwrap();
                vec![Awatism::Blo(b as u8)]
            }
            "sbm" if tokens.len() == 2 => {
                let b: i32 = tokens[1].parse().ok().unwrap();
                vec![Awatism::Sbm(b as u8)]
            }
            "pop" if tokens.len() == 1 => vec![Awatism::Pop],
            "dpl" if tokens.len() == 1 => vec![Awatism::Dpl],
            "srn" if tokens.len() == 2 => {
                let b: i32 = tokens[1].parse().ok().unwrap();
                vec![Awatism::Srn(b as u8)]
            }
            "mrg" if tokens.len() == 1 => vec![Awatism::Mrg],
            "4dd" if tokens.len() == 1 => vec![Awatism::Add],
            "sub" if tokens.len() == 1 => vec![Awatism::Sub],
            "mul" if tokens.len() == 1 => vec![Awatism::Mul],
            "div" if tokens.len() == 1 => vec![Awatism::Div],
            "cnt" if tokens.len() == 1 => vec![Awatism::Cnt],
            "lbl" if tokens.len() == 2 => {
                let b: i32 = tokens[1].parse().ok().unwrap();
                vec![Awatism::Lbl(b as u8)]
            }
            "jmp" if tokens.len() == 2 => {
                let b: i32 = tokens[1].parse().ok().unwrap();
                vec![Awatism::Jmp(b as u8)]
            }
            "eql" if tokens.len() == 1 => vec![Awatism::Eql],
            "lss" if tokens.len() == 1 => vec![Awatism::Lss],
            "gr8" if tokens.len() == 1 => vec![Awatism::Gr8],
            "trm" if tokens.len() == 1 => vec![Awatism::Trm],
            // special macros
            "!str" if tokens.len() == 2 => {
                let mut string_content = tokens[1].trim();
                let replaced_string = string_content.replace("\\n", "\n");
                string_content = &replaced_string;

                let mut awascii = false;

                if string_content.chars().nth(0).unwrap() == 'a' {
                    string_content = &string_content[1..];
                    awascii = true;
                }

                if string_content.chars().nth(0).unwrap() != '"' {
                    panic!("Missing opening '\"'");
                }
                if string_content.len() == 1 || string_content.chars().nth_back(0).unwrap() != '"' {
                    panic!("Missing closing '\"'");
                }
                let string_content = &string_content[1..string_content.len() - 1];

                let mut res = Vec::new();
                for c in string_content.chars().rev() {
                    if awascii {
                        res.push(Awatism::Blo(AWA_SCII.find(c).unwrap() as u8));
                    } else {
                        res.push(Awatism::Blo(c as u8));
                    }
                }
                res.push(Awatism::Srn(res.len() as u8));

                res
            }
            _ => vec![],
        };

        awatism
            .into_iter()
            .map(|a| Instruction { awatism: a })
            .collect()
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
