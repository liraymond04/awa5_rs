extern crate clap;

mod assembler;
mod interpreter;
mod parser;

use clap::{Arg, Command};
use interpreter::interpet_object;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;

#[derive(Debug)]
pub enum Awatism {
    Nop,
    Prn,
    Pr1,
    Red,
    R3d,
    Blo(u8), // s8
    Sbm(u8), // u5
    Pop,
    Dpl,
    Srn(u8),
    Mrg,
    Add,
    Sub,
    Mul,
    Div,
    Cnt,
    Lbl(u8),
    Jmp(u8),
    Eql,
    Lss,
    Gr8,
    Trm,
}

impl Awatism {
    fn from_u8(value: u8, arg: u8) -> Option<Self> {
        match value {
            0x00 => Some(Awatism::Nop),
            0x01 => Some(Awatism::Prn),
            0x02 => Some(Awatism::Pr1),
            0x03 => Some(Awatism::Red),
            0x04 => Some(Awatism::R3d),
            0x05 => Some(Awatism::Blo(arg)),
            0x06 => Some(Awatism::Sbm(arg)),
            0x07 => Some(Awatism::Pop),
            0x08 => Some(Awatism::Dpl),
            0x09 => Some(Awatism::Srn(arg)),
            0x0a => Some(Awatism::Mrg),
            0x0b => Some(Awatism::Add),
            0x0c => Some(Awatism::Sub),
            0x0d => Some(Awatism::Mul),
            0x0e => Some(Awatism::Div),
            0x0f => Some(Awatism::Cnt),
            0x10 => Some(Awatism::Lbl(arg)),
            0x11 => Some(Awatism::Jmp(arg)),
            0x12 => Some(Awatism::Eql),
            0x13 => Some(Awatism::Lss),
            0x14 => Some(Awatism::Gr8),
            0x1F => Some(Awatism::Trm),
            _ => None,
        }
    }

    fn needs_args(value: u8) -> bool {
        match value {
            0x05 => true,
            0x06 => true,
            0x09 => true,
            0x10 => true,
            0x11 => true,
            _ => false,
        }
    }

    fn arg_bits(value: u8) -> usize {
        match value {
            0x05 => 8,
            0x06 => 5,
            0x09 => 5,
            0x10 => 5,
            0x11 => 5,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    awatism: Awatism,
}

pub static AWA_SCII: &str = "AWawJELYHOSIUMjelyhosiumPCNTpcntBDFGRbdfgr0123456789 .,!'()~_/;\n";

fn main() {
    let mut cmd = Command::new("awa5_rs")
        .version("1.0")
        .about("An AWA5.0 CLI tool written in Rust (btw)")
        .arg(
            Arg::new("input")
                .index(1)
                .help("File to interpret or convert")
                .num_args(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Output to file with new format .awasm .awa .o")
                .num_args(1),
        )
        .arg(
            Arg::new("string")
                .short('s')
                .long("string")
                .help("String to interpret or convert")
                .num_args(1),
        )
        .arg(
            Arg::new("awasm")
                .long("awasm")
                .help("Parse string as awasm")
                .num_args(0),
        )
        .arg(
            Arg::new("awa")
                .long("awa")
                .help("Parse string as awatalk")
                .num_args(0),
        );

    let matches = cmd.clone().get_matches();

    if matches.contains_id("input") && matches.contains_id("string") {
        eprintln!(
            "Warning: Both file input and 'string' flag are given. Please provide only one.\n"
        );
        cmd.print_help().unwrap();
        std::process::exit(1);
    }

    if matches.contains_id("input") {
        let input_file = matches.get_one::<String>("input").unwrap();

        let file_type = detect_file_format(input_file);

        match file_type {
            "awasm" => {
                let lines = read_lines(input_file).unwrap();

                let instructions = parser::awasm::parse_lines(lines);

                let object_vec = assembler::make_object_vec(&instructions);

                if matches.contains_id("output") {
                    let output_file = matches.get_one::<String>("output").unwrap();

                    match Path::new(output_file)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap()
                    {
                        "awasm" => {
                            let _ = fs::copy(input_file, output_file);
                        }
                        "awa" => {
                            let result = assembler::object_to_awa(&object_vec);
                            let _ = write_string_file(output_file, &result);
                        }
                        "o" => {
                            let _ = write_object_file(output_file, object_vec);
                        }
                        _ => {}
                    }
                } else {
                    interpet_object(object_vec);
                }
            }
            "awa" => {
                let content = fs::read_to_string(input_file).unwrap();

                let instructions = parser::awatalk::parse_string(&content);

                let object_vec = assembler::make_object_vec(&instructions);

                if matches.contains_id("output") {
                    let output_file = matches.get_one::<String>("output").unwrap();

                    match Path::new(output_file)
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap()
                    {
                        "awasm" => {
                            let result = assembler::object_to_awasm(&object_vec);
                            let _ = write_string_file(output_file, &result);
                        }
                        "awa" => {
                            let _ = fs::copy(input_file, output_file);
                        }
                        "o" => {
                            let _ = write_object_file(output_file, object_vec);
                        }
                        _ => {}
                    }
                } else {
                    interpet_object(object_vec);
                }
            }
            "o" => match read_binary_file(input_file) {
                Ok(binary_data) => {
                    if matches.contains_id("output") {
                        let output_file = matches.get_one::<String>("output").unwrap();

                        match Path::new(output_file)
                            .extension()
                            .and_then(|ext| ext.to_str())
                            .unwrap()
                        {
                            "awasm" => {
                                let result = assembler::object_to_awasm(&binary_data);
                                let _ = write_string_file(output_file, &result);
                            }
                            "awa" => {
                                let result = assembler::object_to_awa(&binary_data);
                                let _ = write_string_file(output_file, &result);
                            }
                            "o" => {
                                let _ = fs::copy(input_file, output_file);
                            }
                            _ => {}
                        }
                    } else {
                        interpet_object(binary_data);
                    }
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                }
            },
            _ => panic!("Could not autodetect file type"),
        }
    } else {
        let mut input_string = String::new();
        if matches.contains_id("string") {
            input_string = matches.get_one::<String>("string").unwrap().clone();
        } else {
            let _ = std::io::stdin().read_line(&mut input_string);
        }

        if matches.get_flag("awasm") && matches.get_flag("awa") {
            eprintln!(
                "Warning: Both 'awasm' and 'awa' flags are given. Please provide only one.\n"
            );
            cmd.print_help().unwrap();
            std::process::exit(1);
        }

        if !matches.get_flag("awasm") && !matches.get_flag("awa") {
            eprintln!("Warning: Neither 'awasm' and 'awa' flags are given. Please provide at least one.\n");
            cmd.print_help().unwrap();
            std::process::exit(1);
        }

        if matches.get_flag("awasm") {
            let lines = input_string.lines().map(|line| line.to_string());

            let instructions = parser::awasm::parse_lines(lines);

            let object_vec = assembler::make_object_vec(&instructions);

            if matches.contains_id("output") {
                let output_file = matches.get_one::<String>("output").unwrap();

                match Path::new(output_file)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap()
                {
                    "awasm" => {
                        let result = assembler::object_to_awasm(&object_vec);
                        let _ = write_string_file(output_file, &result);
                    }
                    "awa" => {
                        let result = assembler::object_to_awa(&object_vec);
                        let _ = write_string_file(output_file, &result);
                    }
                    "o" => {
                        let _ = write_object_file(output_file, object_vec);
                    }
                    _ => {}
                }
            } else {
                interpet_object(object_vec);
            }
        }

        if matches.get_flag("awa") {
            let instructions = parser::awatalk::parse_string(&input_string);

            let object_vec = assembler::make_object_vec(&instructions);

            if matches.contains_id("output") {
                let output_file = matches.get_one::<String>("output").unwrap();

                match Path::new(output_file)
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .unwrap()
                {
                    "awasm" => {
                        let result = assembler::object_to_awasm(&object_vec);
                        let _ = write_string_file(output_file, &result);
                    }
                    "awa" => {
                        let result = assembler::object_to_awa(&object_vec);
                        let _ = write_string_file(output_file, &result);
                    }
                    "o" => {
                        let _ = write_object_file(output_file, object_vec);
                    }
                    _ => {}
                }
            } else {
                interpet_object(object_vec);
            }
        }
    }
}

fn detect_file_format(filename: &str) -> &str {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();
    match extension {
        "awasm" => return "awasm",
        "awa" => return "awa",
        "o" => return "o",
        _ => {}
    }

    let content = match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => return "",
    };

    let normalized = content
        .chars()
        .filter(|&c| c == 'a' || c == 'w' || c == ' ')
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ");

    if normalized.starts_with("awa ") {
        return "awa";
    }

    "awasm"
}

fn read_binary_file(filename: &str) -> Result<Vec<u8>, io::Error> {
    let file_content = fs::read(filename)?;
    Ok(file_content)
}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(Result::ok))
}

fn write_object_file(filename: &str, vec: Vec<u8>) -> io::Result<()> {
    let mut file = File::create(filename).unwrap();

    file.write_all(&vec)
}

fn write_string_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = File::create(filename).unwrap();

    file.write_all(content.as_bytes())
}
