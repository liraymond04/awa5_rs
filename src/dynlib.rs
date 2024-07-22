use core::panic;
use std::{collections::HashMap, fs, path::PathBuf};

use libloading::{Library, Symbol};

use crate::{interpreter::Bubble, AWA_SCII};

#[cfg(target_os = "windows")]
const LIB_EXTENSION: &str = "dll";

#[cfg(not(target_os = "windows"))]
const LIB_EXTENSION: &str = "so";

pub fn parse_fn_name(data: &[u8]) -> String {
    String::from_utf8(data.to_vec()).unwrap()
}

pub fn parse_fn_args(bubble: &Bubble) -> Vec<u8> {
    let mut args = Vec::new();

    let bubbles = bubble.get_bubbles();
    for bubble in bubbles {
        let bubbles = bubble.get_bubbles();
        let arg_type = bubbles[0].get_val();

        match arg_type {
            // i32 or f32
            0x0 => {
                let byte_array = bubbles[1].to_u8_array();
                args.extend_from_slice(&byte_array);
            }
            // awascii char
            0x1 => {
                let char = AWA_SCII.chars().nth(bubbles[1].get_val() as usize).unwrap();
                args.push(char as u8);
            }
            // ascii char
            0x2 => {
                args.push(bubbles[1].get_val() as u8);
            }
            // awascii string
            0x3 => {
                let mut new_string = String::new();
                for b in bubbles[1].get_bubbles().iter().rev() {
                    new_string.push(AWA_SCII.chars().nth(b.get_val() as usize).unwrap());
                }
                args.extend_from_slice(new_string.as_bytes());
                args.push('\0' as u8)
            }
            // ascii string
            0x4 => {
                for b in bubbles[1].get_bubbles().iter().rev() {
                    args.push(b.get_val() as u8);
                }
                args.push('\0' as u8)
            }
            _ => {
                panic!("Invalid argument type provided: {}.", arg_type);
            }
        }
    }

    args
}

pub fn get_shared_library_paths(lib_dirs: &[&str]) -> Vec<String> {
    let mut lib_paths = Vec::new();

    for dir in lib_dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path: PathBuf = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == LIB_EXTENSION {
                            if let Some(path_str) = path.to_str() {
                                lib_paths.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    lib_paths
}


pub fn load_libs(lib_paths: &[&str]) -> HashMap<String, Library> {
    let mut libs = HashMap::new();

    for path in lib_paths {
        let lib = unsafe { Library::new(path).unwrap() };
        libs.insert(path.to_string(), lib);
    }

    libs
}

pub fn call_lib_fn(libs: &HashMap<String, Library>, fn_name: &str, args: Vec<u8>) {
    for (_path, lib) in libs {
        unsafe {
            if let Ok(f) =
                lib.get::<Symbol<unsafe extern "C" fn(*const u8, usize)>>(fn_name.as_bytes())
            {
                f(args.as_ptr(), args.len());
                return;
            }
        }
    }

    panic!("Function not found: {}", fn_name);
}
