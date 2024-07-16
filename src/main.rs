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
            0x15 => Some(Awatism::Trm),
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

fn main() {
    let matches = Command::new("awa5_0")
        .version("1.0")
        .about("An AWA5.0 CLI tool")
        .arg(Arg::new("input").index(1).required(true).num_args(1))
        .arg(
            Arg::new("output_object")
                .short('o')
                .long("output")
                .help("Output code compilation to machine code object file")
                .num_args(1),
        )
        .get_matches();

    if matches.contains_id("input") {
        let input_file = matches.get_one::<String>("input").unwrap();

        let file_type = detect_file_format(input_file);

        match file_type {
            "awa" => {
                let lines = read_lines(input_file).unwrap();

                let instructions = parser::awa::parse_lines(lines);

                // let label_map = parser::awa::resolve_labels(&instructions);

                println!("Parsed commands: {:#?}", instructions);
                // println!("Label map: {:#?}", label_map);

                let object_vec = assembler::make_object_vec(&instructions);

                if matches.contains_id("output_object") {
                    let output_file = matches.get_one::<String>("output_object").unwrap();

                    let _ = write_object_file(output_file, object_vec);
                } else {
                    interpet_object(object_vec);
                }
            }
            "awatalk" => {
                let content = fs::read_to_string(input_file).unwrap();

                let instructions = parser::awatalk::parse_string(&content);

                let object_vec = assembler::make_object_vec(&instructions);

                if matches.contains_id("output_object") {
                    let output_file = matches.get_one::<String>("output_object").unwrap();

                    let _ = write_object_file(output_file, object_vec);
                } else {
                    interpet_object(object_vec);
                }
            }
            "o" => match read_binary_file(input_file) {
                Ok(binary_data) => {
                    interpet_object(binary_data);
                }
                Err(err) => {
                    eprintln!("Error reading file: {}", err);
                }
            },
            _ => panic!("Could not autodetect file type"),
        }
    }
}

fn detect_file_format(filename: &str) -> &str {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .unwrap();
    match extension {
        "awa" => return "awa",
        "awatalk" => return "awatalk",
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
        return "awatalk";
    }

    "awa"
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
