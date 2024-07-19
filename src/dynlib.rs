use core::panic;
use std::collections::HashMap;

use libloading::{Library, Symbol};

pub fn parse_fn_ins_arg(data: &[u8]) -> (String, Vec<Vec<u8>>) {
    let mut cursor = 0;

    let fn_name_len = data[cursor] as usize;
    cursor += 1;

    let fn_name = String::from_utf8(data[cursor..(cursor + fn_name_len)].to_vec()).unwrap();
    cursor += fn_name_len;

    // TODO handle multiple args properly with macros
    let mut args = Vec::new();
    while cursor < data.len() {
        let arg_len = data[cursor] as usize;
        cursor += 1;
        let arg = data[cursor..(cursor + arg_len)].to_vec();
        args.push(arg);
        cursor += arg_len;
    }

    (fn_name, args)
}

pub fn load_libs(lib_paths: &[&str]) -> HashMap<String, Library> {
    let mut libs = HashMap::new();

    for path in lib_paths {
        let lib = unsafe { Library::new(path).unwrap() };
        libs.insert(path.to_string(), lib);
    }

    libs
}

pub fn call_lib_fn(libs: &HashMap<String, Library>, fn_name: &str, args: Vec<Vec<u8>>) {
    for (_path, lib) in libs {
        unsafe {
            if let Ok(f) = lib.get::<Symbol<unsafe extern "C" fn(Vec<Vec<u8>>)>>(fn_name.as_bytes())
            {
                f(args.clone());
                return;
            }
        }
    }

    panic!("Function not found: {}", fn_name);
}
