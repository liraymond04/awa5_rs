use std::collections::HashMap;

use crate::{
    dynlib::{self, call_lib_fn},
    Awatism, AWA_SCII,
};

#[derive(Debug)]
struct Instruction {
    op: u8,
    arg: u8,
}

#[derive(Clone, Debug)]
pub enum Bubble {
    Simple(i32),
    Double(Vec<Bubble>),
}

impl Bubble {
    pub fn is_double(&self) -> bool {
        match self {
            Bubble::Simple(_) => false,
            Bubble::Double(_) => true,
        }
    }

    pub fn get_bubbles(&self) -> &Vec<Bubble> {
        match self {
            Bubble::Double(bubbles) => bubbles,
            _ => panic!("Expected bubble to be double bubble"),
        }
    }

    pub fn get_val(&self) -> i32 {
        match self {
            Bubble::Simple(val) => *val,
            _ => panic!("Expected bubble to be single bubble"),
        }
    }

    pub fn to_u8_array(&self) -> Vec<u8> {
        let mut vec = Vec::new();
        match self {
            Bubble::Simple(val) => vec.push(*val as u8),
            Bubble::Double(bubbles) => {
                for bubble in bubbles {
                    vec.push(bubble.get_val() as u8);
                }
            }
        }
        vec
    }
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

    pub fn before_top(&self) -> Option<&Bubble> {
        if self.bubbles.len() >= 2 {
            self.bubbles.get(self.bubbles.len() - 2)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.bubbles.is_empty()
    }
}

pub fn interpet_object(object_vec: Vec<u8>, path: &str) {
    let mut label_map: HashMap<u8, usize> = HashMap::new();
    let mut instructions: Vec<Instruction> = Vec::new();

    let mut bubble_abyss = BubbleAbyss::new();

    let paths: Vec<&str> = path.split(';').collect();
    let lib_paths = dynlib::get_shared_library_paths(&paths);
    let lib_paths_str: Vec<&str> = lib_paths.iter().map(AsRef::as_ref).collect();
    let libs = dynlib::load_libs(&lib_paths_str);

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

    let mut index = 0;
    while index < instructions.len() {
        let instruction = &instructions[index];
        let op = instruction.op;
        let arg = instruction.arg;

        match Awatism::from_u8(op, arg).unwrap() {
            Awatism::Nop => {}
            Awatism::Prn => {
                let bubble = bubble_abyss.top().unwrap().clone();
                print_bubble(&mut bubble_abyss, &bubble, false, false);
            }
            Awatism::Pr1 => {
                let bubble = bubble_abyss.top().unwrap().clone();
                print_bubble(&mut bubble_abyss, &bubble, true, false);
            }
            Awatism::Red => {
                let mut buffer = String::new();
                let _ = std::io::stdin().read_line(&mut buffer);

                let mut bubbles = Vec::new();
                for c in buffer.chars() {
                    if !AWA_SCII.contains(c) {
                        break;
                    }
                    bubbles.insert(
                        0,
                        Bubble::Simple(AWA_SCII.chars().position(|f| f == c).unwrap() as i32),
                    );
                }

                bubble_abyss.push(Bubble::Double(bubbles));
            }
            Awatism::R3d => {
                let mut buffer = String::new();
                let _ = std::io::stdin().read_line(&mut buffer);

                let mut negative = 1;

                let mut num = String::new();
                for (i, c) in buffer.chars().enumerate() {
                    if i == 0 && c == '-' {
                        negative = -1;
                        continue;
                    }
                    if !c.is_ascii_digit() {
                        break;
                    }
                    num += &c.to_string();
                }

                bubble_abyss.push(Bubble::Simple(negative * num.parse::<i32>().unwrap()));
            }
            Awatism::Blo(arg) => {
                bubble_abyss.push(Bubble::Simple((arg as i8) as i32));
            }
            Awatism::Sbm(arg) => {
                let bubble = bubble_abyss.pop().unwrap();
                if arg == 0 {
                    bubble_abyss.bubbles.insert(0, bubble);
                } else {
                    bubble_abyss
                        .bubbles
                        .insert(bubble_abyss.bubbles.len() - arg as usize, bubble);
                }
            }
            Awatism::Pop => {
                let bubble = bubble_abyss.pop().unwrap();
                match bubble {
                    Bubble::Double(mut bubbles) => {
                        let removed = bubbles.remove(0);
                        bubble_abyss.push(removed);
                    }
                    _ => {}
                }
            }
            Awatism::Dpl => bubble_abyss.push(bubble_abyss.top().unwrap().clone()),
            Awatism::Srn(arg) => {
                let mut bubbles = Vec::new();
                for _ in 0..arg {
                    bubbles.insert(0, bubble_abyss.pop().unwrap().clone())
                }
                bubble_abyss.push(Bubble::Double(bubbles))
            }
            Awatism::Mrg => {
                let bubble1 = bubble_abyss.pop().unwrap();
                let bubble2 = bubble_abyss.pop().unwrap();

                let is_b1_double = bubble1.is_double();
                let is_b2_double = bubble2.is_double();

                if !is_b1_double && !is_b2_double {
                    let mut vec = Vec::new();
                    vec.push(bubble2);
                    vec.push(bubble1);
                    bubble_abyss.push(Bubble::Double(vec));
                } else if is_b1_double && !is_b2_double {
                    let mut bubbles = bubble1.get_bubbles().clone();
                    bubbles.insert(0, bubble2);
                    bubble_abyss.push(Bubble::Double(bubbles));
                } else if !is_b1_double && is_b2_double {
                    let mut bubbles = bubble2.get_bubbles().clone();
                    bubbles.insert(0, bubble1);
                    bubble_abyss.push(Bubble::Double(bubbles));
                } else {
                    let mut bubbles1 = bubble1.get_bubbles().clone();
                    let mut bubbles2 = bubble2.get_bubbles().clone();
                    while bubbles1.len() > 0 {
                        bubbles2.push(bubbles1.remove(0));
                    }
                    bubble_abyss.push(Bubble::Double(bubbles2));
                }
            }
            Awatism::Add => {
                let bubble1 = bubble_abyss.pop().unwrap();
                let bubble2 = bubble_abyss.pop().unwrap();
                let result = operate_bubbles(&add_bubbles, &bubble1, &bubble2);
                bubble_abyss.push(result);
            }
            Awatism::Sub => {
                let bubble1 = bubble_abyss.pop().unwrap();
                let bubble2 = bubble_abyss.pop().unwrap();
                let result = operate_bubbles(&sub_bubbles, &bubble1, &bubble2);
                bubble_abyss.push(result);
            }
            Awatism::Mul => {
                let bubble1 = bubble_abyss.pop().unwrap();
                let bubble2 = bubble_abyss.pop().unwrap();
                let result = operate_bubbles(&mul_bubbles, &bubble1, &bubble2);
                bubble_abyss.push(result);
            }
            Awatism::Div => {
                let bubble1 = bubble_abyss.pop().unwrap();
                let bubble2 = bubble_abyss.pop().unwrap();
                let result = operate_bubbles(&div_bubbles, &bubble1, &bubble2);
                bubble_abyss.push(result);
            }
            Awatism::Cnt => {
                let bubble = bubble_abyss.top().unwrap();
                match bubble {
                    Bubble::Simple(_) => bubble_abyss.push(Bubble::Simple(0)),
                    Bubble::Double(_) => {
                        bubble_abyss.push(Bubble::Simple(bubble.get_bubbles().len() as i32))
                    }
                }
            }
            Awatism::Lbl(_arg) => {}
            Awatism::Jmp(arg) => {
                let jump_position = label_map.get(&arg).unwrap();
                index = *jump_position;
            }
            Awatism::Eql => {
                let top = bubble_abyss.top().unwrap();
                let before_top = bubble_abyss.before_top().unwrap();

                if !top.is_double()
                    && !before_top.is_double()
                    && top.get_val() == before_top.get_val()
                {
                    // execute next line
                } else {
                    index += 1;
                }
            }
            Awatism::Lss => {
                let top = bubble_abyss.top().unwrap();
                let before_top = bubble_abyss.before_top().unwrap();

                if !top.is_double()
                    && !before_top.is_double()
                    && top.get_val() < before_top.get_val()
                {
                    // execute next line
                } else {
                    index += 1;
                }
            }
            Awatism::Gr8 => {
                let top = bubble_abyss.top().unwrap();
                let before_top = bubble_abyss.before_top().unwrap();

                if !top.is_double()
                    && !before_top.is_double()
                    && top.get_val() > before_top.get_val()
                {
                    // execute next line
                } else {
                    index += 1;
                }
            }
            Awatism::Lib => {
                let top = bubble_abyss.pop().unwrap();
                match top {
                    Bubble::Simple(_) => {}
                    Bubble::Double(bubbles) => {
                        let mut fn_name = bubbles[0].to_u8_array();
                        fn_name.reverse();
                        let fn_name = dynlib::parse_fn_name(&fn_name);

                        if bubbles.len() == 1 {
                            call_lib_fn(&libs, &fn_name, vec![]);
                        } else {
                            let fn_args = dynlib::parse_fn_args(&bubbles[1]);
                            call_lib_fn(&libs, &fn_name, fn_args);
                        }
                    }
                }
            }
            Awatism::Trm => {
                break;
            }
        }

        index += 1;
    }
}

fn print_bubble(
    bubble_abyss: &mut BubbleAbyss,
    bubble: &Bubble,
    number: bool,
    current_double: bool,
) {
    match bubble {
        Bubble::Simple(val) => {
            if number {
                print!("{} ", val);
            } else if *val >= 0 && (*val as usize) < AWA_SCII.len() {
                print!("{}", AWA_SCII.chars().nth(*val as usize).unwrap());
            }
            if !current_double {
                bubble_abyss.pop();
            }
        }
        Bubble::Double(bubbles) => {
            for i in (0..bubbles.len()).rev() {
                print_bubble(bubble_abyss, &bubbles[i], number, true);
            }
            bubble_abyss.pop();
        }
    }
}

fn add_bubbles(bubble1: &Bubble, bubble2: &Bubble) -> Bubble {
    Bubble::Simple(bubble1.get_val() + bubble2.get_val())
}

fn sub_bubbles(bubble1: &Bubble, bubble2: &Bubble) -> Bubble {
    Bubble::Simple(bubble1.get_val() - bubble2.get_val())
}

fn mul_bubbles(bubble1: &Bubble, bubble2: &Bubble) -> Bubble {
    Bubble::Simple(bubble1.get_val() * bubble2.get_val())
}

fn div_bubbles(bubble1: &Bubble, bubble2: &Bubble) -> Bubble {
    let mut bubbles = Vec::new();
    let val1 = bubble1.get_val();
    let val2 = bubble2.get_val();
    bubbles.push(Bubble::Simple(val1 % val2)); // remainder
    let result = val1 as f32 / val2 as f32;
    let result = if result < 0.0 {
        // rounded dividend
        result.ceil() as i32
    } else {
        result.floor() as i32
    };
    bubbles.push(Bubble::Simple(result));
    Bubble::Double(bubbles)
}

fn operate_bubbles(
    operation: &dyn Fn(&Bubble, &Bubble) -> Bubble,
    bubble1: &Bubble,
    bubble2: &Bubble,
) -> Bubble {
    let is_b1_double = bubble1.is_double();
    let is_b2_double = bubble2.is_double();

    if !is_b1_double && !is_b2_double {
        return operation(bubble1, bubble2);
    } else if is_b1_double && !is_b2_double {
        let mut bubbles = Vec::new();
        for b in bubble1.get_bubbles() {
            bubbles.push(operate_bubbles(operation, b, bubble2));
        }
        return Bubble::Double(bubbles);
    } else if !is_b1_double && is_b2_double {
        let mut bubbles = Vec::new();
        for b in bubble2.get_bubbles() {
            bubbles.push(operate_bubbles(operation, bubble1, b));
        }
        return Bubble::Double(bubbles);
    } else {
        let mut bubbles: Vec<Bubble> = Vec::new();
        let bubbles1 = bubble1.get_bubbles();
        let bubbles2 = bubble2.get_bubbles();
        for i in 0..std::cmp::min(bubbles1.len(), bubbles2.len()) {
            let result = operate_bubbles(
                operation,
                &bubbles1[bubbles1.len() - i - 1],
                &bubbles2[bubbles2.len() - i - 1],
            );
            bubbles.insert(0, result);
        }
        return Bubble::Double(bubbles);
    }
}
