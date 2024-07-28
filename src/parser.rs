pub mod awasm {
    use core::panic;
    use std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{self, BufRead},
        path::Path,
    };

    use crate::{Awatism, Instruction, AWA_SCII};

    enum MacroResult {
        VecOnly(Vec<Awatism>),
        VecAndBool(Vec<Awatism>, bool),
    }

    impl MacroResult {
        fn get_vec(&self) -> Vec<Awatism> {
            match self {
                MacroResult::VecOnly(v) => v.to_owned(),
                MacroResult::VecAndBool(v, _) => v.to_owned(),
            }
        }

        fn get_vec_and_bool(&self) -> (Vec<Awatism>, bool) {
            match self {
                MacroResult::VecOnly(v) => (v.to_owned(), false),
                MacroResult::VecAndBool(v, b) => (v.to_owned(), *b),
            }
        }
    }

    type MacroFn = fn(&str) -> MacroResult;

    #[derive(Debug)]
    struct UserMacro {
        args: Vec<String>,
        lines: Vec<String>,
    }

    #[derive(Debug)]
    pub struct MacroTable {
        builtins: HashMap<String, MacroFn>,
        user_def: HashMap<String, UserMacro>,
    }

    impl MacroTable {
        pub fn new() -> Self {
            let mut table = MacroTable {
                builtins: HashMap::new(),
                user_def: HashMap::new(),
            };

            table.builtins.insert("!i32".to_string(), process_i32);
            table.builtins.insert("!f32".to_string(), process_f32);
            table.builtins.insert("!chr".to_string(), process_chr);
            table.builtins.insert("!str".to_string(), process_str);
            table.builtins.insert("!_i32".to_string(), |token| {
                let mut res = vec![Awatism::Blo(0x0)];
                res.extend(process_i32(token).get_vec());
                res.push(Awatism::Srn(2));
                MacroResult::VecOnly(res)
            });
            table.builtins.insert("!_f32".to_string(), |token| {
                let mut res = vec![Awatism::Blo(0x0)];
                res.extend(process_f32(token).get_vec());
                res.push(Awatism::Srn(2));
                MacroResult::VecOnly(res)
            });
            table.builtins.insert("!_chr".to_string(), |token| {
                let (process, awascii) = process_chr(token).get_vec_and_bool();
                let mut res = vec![Awatism::Blo(if awascii { 0x1 } else { 0x2 })];
                res.extend(process);
                res.push(Awatism::Srn(2));
                MacroResult::VecAndBool(res, awascii)
            });
            table.builtins.insert("!_str".to_string(), |token| {
                let (process, awascii) = process_str(token).get_vec_and_bool();
                let mut res = vec![Awatism::Blo(if awascii { 0x3 } else { 0x4 })];
                res.extend(process);
                res.push(Awatism::Srn(2));
                MacroResult::VecAndBool(res, awascii)
            });

            table
        }

        fn is_builtin(&self, key: &str) -> bool {
            self.builtins.contains_key(key)
        }

        fn get_builtin(&self, key: &str) -> Option<MacroFn> {
            self.builtins.get(key).copied()
        }

        fn add_user_def(&mut self, key: &str, args: Vec<String>, lines: Vec<String>) {
            self.user_def
                .insert(key.to_string(), UserMacro { args, lines });
        }

        fn get_user_def(&self, key: &str) -> Option<&UserMacro> {
            self.user_def.get(key)
        }
    }

    fn parse_function_call(s: &str) -> (&str, Vec<String>) {
        if let Some(pos) = s.find('(') {
            let function_name = &s[..pos];

            let args_str = &s[pos + 1..s.len() - 1];

            let args: Vec<String> = args_str
                .split(',')
                .map(|arg| arg.trim().to_string())
                .collect();

            (function_name, args)
        } else {
            (s, Vec::new())
        }
    }

    pub fn parse_lines(
        macro_table: &mut MacroTable,
        already_included: &mut HashSet<String>,
        include_paths: &str,
        current_path: &str,
        lines: impl Iterator<Item = String>,
    ) -> Vec<Instruction> {
        let mut result = vec![];
        let mut defining = false;
        let mut define_str = String::new();
        let mut cur_macro = vec![];

        for line in lines {
            let line_without_comments = line.split(';').next().unwrap_or("");
            let trimmed = line_without_comments.trim();
            let tokens: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();

            if tokens.is_empty() || trimmed.starts_with(';') {
                continue;
            }

            if tokens[0] == "!once" {
                if already_included.contains(current_path) {
                    break;
                }
                continue;
            }

            if tokens[0] == "!include" {
                result.extend(include_file(
                    macro_table,
                    already_included,
                    include_paths,
                    tokens[1],
                ));
                continue;
            }

            if tokens[0] == "!def" {
                defining = true;
                define_str = tokens[1].to_string();
                continue;
            }

            if tokens[0] == "!end" {
                defining = false;
                let (define_name, define_args) = parse_function_call(&define_str);
                macro_table.add_user_def(&define_name, define_args, cur_macro);
                cur_macro = vec![];
                continue;
            }

            if defining {
                cur_macro.push(trimmed.to_string());
                continue;
            }

            result.extend(parse_line(&macro_table, &tokens));
        }

        result
            .into_iter()
            .map(|a| Instruction { awatism: a })
            .collect()
    }

    fn parse_line(macro_table: &MacroTable, tokens: &Vec<&str>) -> Vec<Awatism> {
        // is macro
        if tokens[0].starts_with("!") {
            if macro_table.is_builtin(tokens[0]) {
                if let Some(process_fn) = macro_table.get_builtin(tokens[0]) {
                    return process_fn(tokens[1]).get_vec();
                }
            } else {
                if let Some(_macro) = macro_table.get_user_def(&tokens[0][1..]) {
                    let args: Vec<&str> = tokens[1].split(',').collect();
                    if _macro.args.len() != args.len() {
                        panic!(
                            "Macro !{} expected {} arguments, received {}",
                            &tokens[0][1..],
                            _macro.args.len(),
                            args.len()
                        );
                    }
                    return expand_macro(macro_table, _macro, args);
                } else {
                    panic!("Macro !{} not found", &tokens[0][1..]);
                }
            }
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
            "lib" if tokens.len() == 1 => vec![Awatism::Lib],
            "trm" if tokens.len() == 1 => vec![Awatism::Trm],
            _ => vec![],
        };

        awatism
    }

    fn parse_include_paths(include_paths: &str) -> Vec<&str> {
        include_paths.split(';').collect()
    }

    fn find_and_read_file(relative_path: &str, paths: &[&str]) -> io::Result<Option<Vec<String>>> {
        for path in paths {
            let full_path = Path::new(path).join(relative_path);
            if full_path.exists() {
                let file = File::open(full_path)?;
                let lines: Vec<String> = io::BufReader::new(file)
                    .lines()
                    .filter_map(Result::ok)
                    .collect();
                return Ok(Some(lines));
            }
        }
        Ok(None)
    }

    fn include_file(
        macro_table: &mut MacroTable,
        already_included: &mut HashSet<String>,
        include_paths: &str,
        path_str: &str,
    ) -> Vec<Awatism> {
        let replaced_string = path_str.replace("\\n", "\n");
        let path = replaced_string.trim();

        if path.chars().nth(0).unwrap() != '<' {
            panic!("Missing opening \"<\"");
        }
        if path.len() == 1 || path.chars().nth_back(0).unwrap() != '>' {
            panic!("Missing closing \">\"");
        }
        let path = &path[1..path.len() - 1];

        let paths = parse_include_paths(include_paths);

        if let Some(lines) = find_and_read_file(path, &paths).unwrap() {
            let instructions = parse_lines(
                macro_table,
                already_included,
                include_paths,
                path,
                lines.into_iter(),
            );
            already_included.insert(path.to_string());
            instructions.into_iter().map(|i| i.awatism).collect()
        } else {
            panic!("Awasm source file {} not found", path);
        }
    }

    fn expand_macro(macro_table: &MacroTable, _macro: &UserMacro, args: Vec<&str>) -> Vec<Awatism> {
        let mut res = vec![];
        for line in &_macro.lines {
            let mut expanded_line = line.to_string();
            for i in 0..args.len() {
                let arg_name = &_macro.args[i];
                let arg_val = args[i].trim();
                let placeholder = format!("${}", arg_name);
                expanded_line = expanded_line.replace(&placeholder, arg_val);
            }

            let line_without_comments = expanded_line.split(';').next().unwrap_or("");
            let trimmed = line_without_comments.trim();
            let tokens: Vec<&str> = trimmed.splitn(2, char::is_whitespace).collect();
            res.extend(parse_line(macro_table, &tokens));
        }
        res
    }

    fn process_i32(token: &str) -> MacroResult {
        let value = token.parse().ok().unwrap();
        let mut res = Vec::new();
        for byte in i32::to_le_bytes(value) {
            res.push(Awatism::Blo(byte));
        }
        res.push(Awatism::Srn(4));
        MacroResult::VecOnly(res)
    }

    fn process_f32(token: &str) -> MacroResult {
        let value = token.parse().ok().unwrap();
        let mut res = Vec::new();
        for byte in f32::to_le_bytes(value) {
            res.push(Awatism::Blo(byte));
        }
        res.push(Awatism::Srn(4));
        MacroResult::VecOnly(res)
    }

    fn process_chr(string_content: &str) -> MacroResult {
        let mut res = Vec::new();
        let replaced_string = string_content.replace("\\n", "\n");
        let mut string_content = replaced_string.trim();

        let mut awascii = false;
        if string_content.chars().nth(0).unwrap() == 'a' {
            string_content = &string_content[1..];
            awascii = true;
        }

        if string_content.chars().nth(0).unwrap() != '\'' {
            panic!("Missing opening \"'\"");
        }
        if string_content.len() == 1 || string_content.chars().nth_back(0).unwrap() != '\'' {
            panic!("Missing closing \"'\"");
        }

        if string_content.len() > 3 {
            panic!("!char expects a single character")
        }

        let string_content = &string_content[1..string_content.len() - 1];
        let c = string_content.chars().nth(0).unwrap();

        if awascii {
            res.push(Awatism::Blo(AWA_SCII.find(c).unwrap() as u8));
        } else {
            res.push(Awatism::Blo(c as u8));
        }

        MacroResult::VecAndBool(res, awascii)
    }

    fn process_str(string_content: &str) -> MacroResult {
        let mut res = Vec::new();
        let replaced_string = string_content.replace("\\n", "\n");
        let mut string_content = replaced_string.trim();

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

        let mut temp_res = Vec::new();
        for c in string_content.chars() {
            if awascii {
                temp_res.push(Awatism::Blo(AWA_SCII.find(c).unwrap() as u8));
            } else {
                temp_res.push(Awatism::Blo(c as u8));
            }
        }

        let mut i = 0;
        let chunk_size = 31; // max allowed value of u5 for srn arg
        while !temp_res.is_empty() {
            let len = temp_res.len();
            let end = if len > chunk_size {
                len - chunk_size
            } else {
                0
            };

            for _ in end..len {
                res.push(temp_res.pop().unwrap());
            }

            res.push(Awatism::Srn((len - end) as u8));

            if i > 0 {
                res.push(Awatism::Mrg);
            }
            i += 1;
        }

        MacroResult::VecAndBool(res, awascii)
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
