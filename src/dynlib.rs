use core::panic;
use std::collections::HashMap;

use libloading::{Library, Symbol};

use crate::interpreter::Bubble;

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
            _ => {}
        }
    }

    args
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
