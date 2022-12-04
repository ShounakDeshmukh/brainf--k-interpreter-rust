use std::{char, collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    // println!("{}", contents);

    let mut loop_stack: Vec<i32> = vec![];
    let mut loop_begin_index: i32;
    let mut loop_table: HashMap<usize, usize> = HashMap::new();
    for (ip, instruction) in contents.chars().enumerate() {
        if instruction == '[' {
            loop_stack.push(ip as i32)
        } else if instruction == ']' {
            loop_begin_index = loop_stack.pop().unwrap();

            loop_table.insert(loop_begin_index as usize, ip);
            loop_table.insert(ip, loop_begin_index as usize);
        }
    }

    let mut tape: Vec<i32> = vec![0];
    let mut cell_index = 0;
    let mut instruction_pointer = 0;
    let mut instruction: char;

    while instruction_pointer < contents.len() {
        instruction = contents.as_bytes()[instruction_pointer] as char;

        if instruction == '+' {
            tape[cell_index] += 1;
            if tape[cell_index] == 256 {
                tape[cell_index] = 0;
            }   
        } else if instruction == '-' {
            tape[cell_index] -= 1;
            if tape[cell_index] == -1 {
                tape[cell_index] = 255;
            }
        } else if instruction == '<' {
            cell_index -= 1;
        } else if instruction == '>' {
            cell_index += 1;
            if cell_index >= tape.len() {
                tape.push(0);
            }
        } else if instruction == '.' {
            let output = char::from_u32(tape[cell_index].try_into().unwrap()).unwrap();
            print!("{}",output)
        } else if instruction == '[' {
            if tape[cell_index] == 0 {
                instruction_pointer = loop_table[&instruction_pointer]
            }
        } else if instruction == ']' {
            if tape[cell_index] != 0 {
                instruction_pointer = loop_table[&instruction_pointer]
            }
        }
        instruction_pointer+=1;
    }
}
