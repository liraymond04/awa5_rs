use std::collections::HashMap;

use crate::Awatism;

#[derive(Debug)]
struct Instruction {
    op: u8,
    arg: u8,
}

#[derive(Debug)]
enum Bubble {
    Simple(i32),
    Double(Vec<Bubble>),
}

#[derive(Debug)]
struct BubbleAbyss {
    bubbles: Vec<Bubble>,
}

impl BubbleAbyss {
    pub fn new() -> Self {
        BubbleAbyss {
            bubbles: Vec::new(),
        }
    }

    pub fn push(&mut self, bubble: Bubble) {
        self.bubbles.push(bubble);
    }

    pub fn pop(&mut self) -> Option<Bubble> {
        self.bubbles.pop()
    }

    pub fn top(&self) -> Option<&Bubble> {
        self.bubbles.last()
    }

    pub fn is_empty(&self) -> bool {
        self.bubbles.is_empty()
    }
}

pub fn interpet_object(object_vec: Vec<u8>) {
    let mut label_map: HashMap<u8, usize> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut bubble_abyss = BubbleAbyss::new();

    // create label map
    for (index, chunk) in object_vec.chunks_exact(2).enumerate() {
        let op = chunk[0];
        let arg = chunk[1];

        let instruction = Instruction { op, arg };
        instructions.push(instruction);

        match Awatism::from_u8(op, arg).unwrap() {
            Awatism::Lbl(arg) => {
                label_map.insert(arg, index);
            }
            _ => {}
        }
    }

    for instruction in &instructions {
        let op = instruction.op;
        let arg = instruction.arg;

        match Awatism::from_u8(op, arg).unwrap() {
            Awatism::Pr1 => {
                let bubble = bubble_abyss.top().unwrap();
                match bubble {
                    Bubble::Simple(val) => {
                        println!("{}", val);
                    }
                    Bubble::Double(bubbles) => {}
                }
            }
            Awatism::Blo(arg) => {
                bubble_abyss.push(Bubble::Simple(arg as i32));
                // println!("op {} arg {}", op, arg);
            }
            _ => {}
        }
    }

    println!("{:#?}", instructions);
}
