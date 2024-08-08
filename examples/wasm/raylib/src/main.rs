use std::collections::HashSet;

use awa5_rs::*;

fn main() {
    let path = "/usr/local/lib;/assets";
    let include_paths = "/assets";
    let lines = read_lines("/assets/raylib.awasm").unwrap();

    let mut macro_table = parser::awasm::MacroTable::new();
    let mut already_included: HashSet<String> = HashSet::new();
    let mut label_included: HashSet<String> = HashSet::new();
    let instructions = parser::awasm::parse_lines(
        &mut macro_table,
        &mut already_included,
        &mut label_included,
        include_paths,
        "",
        lines,
    );

    let object_vec = assembler::make_object_vec(&instructions);

    interpet_object(object_vec, path);
}
