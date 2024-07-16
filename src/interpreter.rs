use std::collections::HashMap;

use crate::{AwaSCII, Awatism};

#[derive(Debug)]
struct Instruction {
    op: u8,
    arg: u8,
}

#[derive(Clone, Debug)]
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
            Awatism::Nop => {}
            Awatism::Prn => {
                let bubble = bubble_abyss.top().unwrap().clone();
                print_bubble(&mut bubble_abyss, &bubble, false);
            }
            Awatism::Pr1 => {
                let bubble = bubble_abyss.top().unwrap().clone();
                print_bubble(&mut bubble_abyss, &bubble, true);
            }
            Awatism::Red => {}
            Awatism::R3d => {}
            Awatism::Blo(arg) => {
                bubble_abyss.push(Bubble::Simple(arg as i32));
                // println!("op {} arg {}", op, arg);
            }
            Awatism::Pop => {}
            Awatism::Dpl => {}
            Awatism::Srn(arg) => {
                let mut bubbles = Vec::new();
                for _ in 0..arg {
                    bubbles.insert(0, bubble_abyss.pop().unwrap().clone())
                }
                bubble_abyss.push(Bubble::Double(bubbles))
            }
            Awatism::Mrg => {}
            Awatism::Add => {}
            Awatism::Sub => {}
            Awatism::Mul => {}
            Awatism::Div => {}
            Awatism::Cnt => {}
            Awatism::Lbl(arg) => {}
            Awatism::Jmp(arg) => {}
            Awatism::Eql => {}
            Awatism::Lss => {}
            Awatism::Gr8 => {}
            Awatism::Trm => {}
            _ => {}
        }
    }

    // println!("{:#?}", instructions);
}

fn print_bubble(bubble_abyss: &mut BubbleAbyss, bubble: &Bubble, number: bool) {
    match bubble {
        Bubble::Simple(val) => {
            if number {
                print!("{} ", val);
            } else if *val >= 0 && (*val as usize) < AwaSCII.len() {
                print!("{}", AwaSCII.chars().nth(*val as usize).unwrap());
            }
            bubble_abyss.pop();
        }
        Bubble::Double(bubbles) => {
            for i in (0..bubbles.len()).rev() {
                print_bubble(bubble_abyss, &bubbles[i], number);
            }
        }
    }
}
