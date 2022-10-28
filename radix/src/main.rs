use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::env;

#[derive(Debug, Clone)]
struct Function {
    name: String,
    arguments: Vec<String>,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    address: String,
}

static mut FUNCTIONS: Vec<Function> = vec![];

fn main() {
    let args: Vec<String> = env::args().collect();

    let data: String = fs::read_to_string(&args[1]).expect("Unable to read file");
    let data: String = data.replace("\n", "").replace("\r", "").replace("\t", "").replace(" ", "");

    let mut data: Vec<char> = data.chars().collect();

    data.append(&mut vec![' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ']);

    let mut char = 0;
    
    unsafe {
        while char < data.len() {
            let mut token = "".to_owned();
            let mut j = 0;
            while j < 10 && char < data.len() - 10 {
                token.push(data[char + j]);
                j += 1;
            }

            if token.starts_with("/*") {
                let mut d_char = "".to_owned();
                while d_char != "*/" {
                    d_char = "".to_owned();
                    d_char.push(data[char]);
                    d_char.push(data[char + 1]);
                    char += 1;
                }
            }
   
            if token.starts_with("func") {
    
                char += 4;
                let mut name = "".to_owned();

                let mut args: Vec<String> = vec![];

                loop {
                    if data[char] == '(' {
                        break
                    }
                    name.push(data[char]);
                    char += 1;
                }

                let mut arg = "".to_owned();

                loop {
                    if data[char] == ')' {
                        break
                    }
                    char += 1;
                    loop {
                        if data[char] == ')' || data[char] == ',' {
                            break
                        }
                        arg.push(data[char]);
                        char += 1;
                    }
                    args.push(arg.to_owned());
                } 

                while data[char] != '{' {
                    char += 1;
                }

                let start = char;

                let mut block = 1;
                char += 1;

                while block != 0 {
                    if data[char] == '{' {
                        block += 1;
                    }
                    else if data[char] == '}' {
                        block -= 1;
                    }
                    char += 1;
                }

                let end = char;

                FUNCTIONS.push(
                    Function{
                        name: name,
                        arguments: args,
                        start: start,
                        end: end,
                })
            }
            
            char += 1;
        }

        let mut asm = "jmp main;".to_owned();

        println!("{:?}", FUNCTIONS);
    
        for function in FUNCTIONS.clone() {
            if function.name == "main" {
                asm.push_str(&compile(data.clone(), function));
            }
        }
        
        println!("{}", asm.replace(";", ";\n"))
    }
}

static mut VAR_NUMBER: u16 = 0;

unsafe fn compile(data: Vec<char>, function: Function) -> String {
    let mut variables: Vec<Variable> = vec![];
    let mut asm = "".to_owned();

    for arg in function.arguments {
        variables.push(Variable {
            name: arg.to_owned(),
            address: format!("a0x{:x}", VAR_NUMBER + 0xa000),
        });
        VAR_NUMBER += 1;
    }

    let mut label = "label ".to_owned();
    label.push_str(&function.name);
    label.push(';');

    asm.push_str(&label);

    let mut char = function.start;
    while char < function.end {
        let mut token = "".to_owned();
        let mut j = 0;
        while j < 10 {
            token.push(data[char + j]);
            j += 1;
        }
        if token.starts_with("/*") {
            let mut d_char = "".to_owned();
            while d_char != "*/" {
                d_char = "".to_owned();
                d_char.push(data[char]);
                d_char.push(data[char + 1]);
                char += 1;
            }
        }
        
        else if token.starts_with("let") {
            char += 3;

            let mut name = "".to_owned();

            loop {
                if data[char] == '=' {
                    break;
                }
                name.push(data[char]);
                char += 1;
            }
            char += 1;

            let mut value = "".to_owned();

            loop {
                if data[char] == ';' {
                    break;
                }
                value.push(data[char]);
                char += 1;
            }
            char += 1;

            let r = compile_value(asm, value, variables.clone());
            asm = r.0;
            value = r.1;

            variables.push(
                Variable {
                    name: name,
                    address: format!("a0x{:x}", VAR_NUMBER + 0xa000),
                }
            );  
            asm.push_str(&format!("put {} a0x{:x};", value, VAR_NUMBER + 0xa000));
            VAR_NUMBER += 1;
        }
        else if token.starts_with("") {

        }

        char += 1;
    }
    return asm;
}

unsafe fn compile_value(mut asm: String, mut value: String, variables: Vec<Variable>) -> (String, String) {
    if value.parse::<f64>().is_ok() {
        value = format!("0x{:x}", value.parse::<u16>().unwrap());
    }

    else if value.contains("+") | value.contains("-") | value.contains("/") | value.contains("*") {
        let mut v_chars: Vec<char> = value.chars().collect();
        v_chars.push(' ');
        
        let mut i = 0;
        let mut equation: Vec<String> = vec![];

        while i < v_chars.len() - 1 {
            let mut local_value = "".to_owned();

            while v_chars[i] != '+' && v_chars[i] != '-' && v_chars[i] != '/' && v_chars[i] != '*' && i < v_chars.len() - 1 {
                local_value.push(v_chars[i]);      
                i += 1;
            }
            equation.push(local_value);
            equation.push(v_chars[i].to_string());
            i += 1;
        }
        equation.pop();
        i = 0;
        while i < equation.len() {
            if equation[i].parse::<f64>().is_ok() {
                equation[i] = format!("0x{:x}", equation[i].parse::<u16>().unwrap());
            }
            else {
                for var in &variables {
                    if var.name == equation[i] {
                        equation[i] = var.address.clone()
                    }
                }
            }
            i += 2;
        }

        i = 1;
        while i < equation.len() - 1 {         
            asm.push_str(&format!("put {} r6;", equation[i + 1]));
            if i == 1 {
                asm.push_str(&format!("put {} r7;", equation[i - 1]));
                match equation[i].as_str() {
                    
                    "+" => {
                        asm.push_str("add r7 r6 r0;")
                    }
                    "-" => {
                        asm.push_str("sub r7 r6 r0;")
                    }
                    "/" => {
                        asm.push_str("div r7 r6 r0;")
                    }
                    "*" => {
                        asm.push_str("mlt r7 r6 r0;")
                    }

                    _ => {}
                }
            }
            else {
                match equation[i].as_str() {
                    
                    "+" => {
                        asm.push_str("add r0 r6 r0;")
                    }
                    "-" => {
                        asm.push_str("sub r0 r6 r0;")
                    }
                    "/" => {
                        asm.push_str("div r0 r6 r0;")
                    }
                    "*" => {
                        asm.push_str("mlt r0 r6 r0;")
                    }

                    _ => {}
                }
            }
            i += 2;
        }
        value = "r0".to_owned();
    }
    else {
        for var in &variables {
            if var.name == value {
                value = var.address.clone();
            } 
        }
        for func in FUNCTIONS.clone() {
            if func.name == value {
                let mut i = 0;
                while i < func.arguments.len() {
                    let r = compile_value(asm, func.arguments[i].clone(), variables.clone());
                    asm = r.0;
                    let arg = r.1;

                    asm.push_str(&format!("put {} a0x{:x}", arg, VAR_NUMBER + i as u16));
                    i += 1;
                }
            } 
        }
    }


    return (asm, value);
}
