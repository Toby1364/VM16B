use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::env;

struct Label {
    name: String,
    line: i32,
}

fn add_bin(mut file: &File, data: &str) {
    if let Err(e) = writeln!(file, "{}", data) {
        eprintln!("Couldn't write to file: {}", e);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    let data: String = fs::read_to_string(&args[1]).expect("Unable to read file");
    let data: String = data.replace("\n", "").replace("\r", "");
    let data: Vec<&str> = data.split(";").collect();

    fs::write("boot.bin", "").expect("Unable to write file");
    let f = OpenOptions::new()
        .write(true)
        .append(true)
        .open("boot.bin")
        .unwrap();

    let mut line_num: i32 = 0;
    let mut labels: Vec<Label> = vec![];

    for line in data {
        let part: Vec<&str> = line.split(" ").collect();
        match part[0] {

            "label" => {
                labels.push(Label{
                name: part[1].to_owned(),
                line: line_num,
            })}

            "jmp" => {
                let mut jumped = false;
                for label in &labels {
                    if label.name == part[1] {
                        add_bin(&f, "0x001c");
                        let mut goto = "0x".to_owned();
                        goto.push_str(&format!("{:x}", label.line));
                        add_bin(&f, &goto);
                        jumped = true;
                    }
                }
                if !jumped {
                    add_bin(&f, "0x001c");
                    let mut label = "__LABEL__ ".to_owned();
                    label.push_str(part[1]);
                    add_bin(&f, &label);
                }
                line_num += 2;
            }

            "put" => {
                if part[1].starts_with("a0x") {
                    if part[2].starts_with("a0x") {
                        add_bin(&f, "0x000d");
                        add_bin(&f, &part[1].replace("a0x", "0x"));
                        add_bin(&f, "0x000c");
                        add_bin(&f, &part[2].replace("a0x", "0x"));
                        line_num += 4;
                    }
                    else if part[2] == "r1_adr" {
                        add_bin(&f, "0x000d");
                        add_bin(&f, &part[1].replace("a0x", "0x"));
                        add_bin(&f, "0x0025");
                        line_num += 3;
                    }
                    else {
                        match part[2] {
                            "r0" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                line_num += 2;
                            }
                            "r1" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x000e");
                                line_num += 3;
                            }
                            "r2" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x000f");
                                line_num += 3;
                            }
                            "r3" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x0010");
                                line_num += 3;
                            }
                            "r4" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x0011");
                                line_num += 3;
                            }
                            "r5" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x0012");
                                line_num += 3;
                            }
                            "r6" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x0013");
                                line_num += 3;
                            }
                            "r7" => {
                                add_bin(&f, "0x000d");
                                add_bin(&f, &part[1].replace("a0x", "0x"));
                                add_bin(&f, "0x0014");
                                line_num += 3;
                            }
                            _ => {}
                        }
                    }
                }
                else if part[1].starts_with("0x") {
                    if part[2].starts_with("a0x") {
                        add_bin(&f, "0x0000");
                        add_bin(&f, &part[1]);
                        add_bin(&f, "0x000c");
                        add_bin(&f, &part[2].replace("a0x", "0x"));
                        line_num += 4;
                    }
                    else if part[2] == "r1_adr" {
                        add_bin(&f, "0x0000");
                        add_bin(&f, &part[1]);
                        add_bin(&f, "0x0025");
                        line_num += 3;
                    }
                    else {
                        match part[2] {
                            "r0" => {
                                add_bin(&f, "0x0000");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r1" => {
                                add_bin(&f, "0x0001");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r2" => {
                                add_bin(&f, "0x0002");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r3" => {
                                add_bin(&f, "0x0003");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r4" => {
                                add_bin(&f, "0x0004");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r5" => {
                                add_bin(&f, "0x0005");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r6" => {
                                add_bin(&f, "0x0006");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }
                            "r7" => {
                                add_bin(&f, "0x0007");
                                add_bin(&f, part[1]);
                                line_num += 2;
                            }

                            _ => {}
                        }
                    }
                }
                else {
                    match part[1] {
                        "r1" => {
                            add_bin(&f, "0x0015");
                            line_num += 1;
                        }
                        "r2" => {
                            add_bin(&f, "0x0016");
                            line_num += 1;
                        }
                        "r3" => {
                            add_bin(&f, "0x0017");
                            line_num += 1;
                        }
                        "r4" => {
                            add_bin(&f, "0x0018");
                            line_num += 1;
                        }
                        "r5" => {
                            add_bin(&f, "0x0019");
                            line_num += 1;
                        }
                        "r6" => {
                            add_bin(&f, "0x001a");
                            line_num += 1;
                        }
                        "r7" => {
                            add_bin(&f, "0x001b");
                            line_num += 1;
                        }
                        _ => {}
                    }
                    if part[2].starts_with("a0x") {
                        add_bin(&f, "0x000c");
                        add_bin(&f, &part[2].replace("a0x", "0x"));
                        line_num += 2;
                    }
                    else if part[2] == "r1_adr" {
                        add_bin(&f, "0x0025");
                        line_num += 1;
                    }
                    else {
                        match part[2] {
                            "r1" => {
                                add_bin(&f, "0x000e");
                                line_num += 1;
                            }
                            "r2" => {
                                add_bin(&f, "0x000f");
                                line_num += 1;
                            }
                            "r3" => {
                                add_bin(&f, "0x0010");
                                line_num += 1;
                            }
                            "r4" => {
                                add_bin(&f, "0x0011");
                                line_num += 1;
                            }
                            "r5" => {
                                add_bin(&f, "0x0012");
                                line_num += 1;
                            }
                            "r6" => {
                                add_bin(&f, "0x0013");
                                line_num += 1;
                            }
                            "r7" => {
                                add_bin(&f, "0x0014");
                                line_num += 1;
                            }
                            _ => {}
                        }
                    }
                }
            }

            "add" => {
                match part[2] {
                    "r0" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x0016");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    _ => {}
                }

                match part[1] {
                    "r2" => {
                        add_bin(&f, "0x0016");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        line_num += 1;
                    }
                    _ => {}
                }
            
                add_bin(&f, "0x0008");
                add_bin(&f, "0x001d");
                line_num += 2;

                match part[3] {
                    "r1" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x000f");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0010");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0011");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0012");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x0013");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x0014");
                        line_num += 1;
                    }
                    _ => {}
                }
            }
            
            "sub" => {
                match part[2] {
                    "r0" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x0016");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    _ => {}
                }

                match part[1] {
                    "r2" => {
                        add_bin(&f, "0x0016");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        line_num += 1;
                    }
                    _ => {}
                }
            
                add_bin(&f, "0x0009");
                add_bin(&f, "0x001d");
                line_num += 2;

                match part[3] {
                    "r1" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x000f");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0010");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0011");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0012");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x0013");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x0014");
                        line_num += 1;
                    }
                    _ => {}
                }
            }

            "mlt" => {
                match part[2] {
                    "r0" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x0016");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    _ => {}
                }

                match part[1] {
                    "r2" => {
                        add_bin(&f, "0x0016");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        line_num += 1;
                    }
                    _ => {}
                }
            
                add_bin(&f, "0x000a");
                add_bin(&f, "0x001d");
                line_num += 2;

                match part[3] {
                    "r1" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x000f");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0010");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0011");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0012");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x0013");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x0014");
                        line_num += 1;
                    }
                    _ => {}
                }
            }

            "div" => {
                match part[2] {
                    "r0" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x0016");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        add_bin(&f, "0x000e");
                        line_num += 2;
                    }
                    _ => {}
                }

                match part[1] {
                    "r2" => {
                        add_bin(&f, "0x0016");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0017");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0018");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0019");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x001a");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x001b");
                        line_num += 1;
                    }
                    _ => {}
                }
            
                add_bin(&f, "0x000b");
                add_bin(&f, "0x001d");
                line_num += 2;

                match part[3] {
                    "r1" => {
                        add_bin(&f, "0x000e");
                        line_num += 1;
                    }
                    "r2" => {
                        add_bin(&f, "0x000f");
                        line_num += 1;
                    }
                    "r3" => {
                        add_bin(&f, "0x0010");
                        line_num += 1;
                    }
                    "r4" => {
                        add_bin(&f, "0x0011");
                        line_num += 1;
                    }
                    "r5" => {
                        add_bin(&f, "0x0012");
                        line_num += 1;
                    }
                    "r6" => {
                        add_bin(&f, "0x0013");
                        line_num += 1;
                    }
                    "r7" => {
                        add_bin(&f, "0x0014");
                        line_num += 1;
                    }
                    _ => {}
                }
            }

            "jmpif" => {
                if part[3].starts_with("0x") {
                    add_bin(&f, "0x0001");
                    add_bin(&f, part[3]);
                    line_num += 2;
                }

                else if part[3].starts_with("a0x") {
                    add_bin(&f, "0x000d");
                    add_bin(&f, &part[3].replace("a0x", "0x"));
                    add_bin(&f, "0x000e");
                    line_num += 3;
                }

                else {
                    match part[3] {
                        "r0" => {
                            add_bin(&f, "0x000e");
                            line_num += 1;
                        }
                        "r2" => {
                            add_bin(&f, "0x0016");
                            add_bin(&f, "0x000e");
                            line_num += 2;
                        }
                        "r3" => {
                            add_bin(&f, "0x0017");
                            add_bin(&f, "0x000e");
                            line_num += 2;
                        }
                        "r4" => {
                            add_bin(&f, "0x0018");
                            add_bin(&f, "0x000e");
                            line_num += 2;
                        }
                        "r5" => {
                            add_bin(&f, "0x0019");
                            add_bin(&f, "0x000e");
                            line_num += 2;
                        }
                        "r6" => {
                            add_bin(&f, "0x001a");
                            add_bin(&f, "0x000e");
                            line_num += 2;
                        }
                        "r7" => {
                            add_bin(&f, "0x001b");
                            add_bin(&f, "0x000e");
                            line_num += 2;
                        }
                        _ => {}
                    }
                }

                if part[1].starts_with("0x") {
                    add_bin(&f, "0x0000");
                    add_bin(&f, part[1]);
                    line_num += 2;
                }

                else if part[1].starts_with("a0x") {
                    add_bin(&f, "0x000d");
                    add_bin(&f, &part[1].replace("a0x", "0x"));
                    line_num += 2;
                }

                else {
                    match part[1] {
                        "r2" => {
                            add_bin(&f, "0x0016");
                            line_num += 1;
                        }
                        "r3" => {
                            add_bin(&f, "0x0017");
                            line_num += 1;
                        }
                        "r4" => {
                            add_bin(&f, "0x0018");
                            line_num += 1;
                        }
                        "r5" => {
                            add_bin(&f, "0x0019");
                            line_num += 1;
                        }
                        "r6" => {
                            add_bin(&f, "0x001a");
                            line_num += 1;
                        }
                        "r7" => {
                            add_bin(&f, "0x001b");
                            line_num += 1;
                        }
                        _ => {}
                    }
                }
            
                match part[2] {
                    "=" => {
                        add_bin(&f, "0x001e");
                        line_num += 1;

                        for label in &labels {
                            if label.name == part[4] {
                                let mut goto = "0x".to_owned();
                                goto.push_str(&format!("{:x}", label.line));
                                add_bin(&f, &goto);
                                line_num += 1;
                            }
                        }
                    }
                    ">" => {
                        add_bin(&f, "0x001f");
                        line_num += 1;

                        for label in &labels {
                            if label.name == part[4] {
                                let mut goto = "0x".to_owned();
                                goto.push_str(&format!("{:x}", label.line));
                                add_bin(&f, &goto);
                                line_num += 1;
                            }
                        }
                    }
                    "<" => {
                        add_bin(&f, "0x0020");
                        line_num += 1;

                        for label in &labels {
                            if label.name == part[4] {
                                let mut goto = "0x".to_owned();
                                goto.push_str(&format!("{:x}", label.line));
                                add_bin(&f, &goto);
                                line_num += 1;
                            }
                        }
                    }
                    "!=" => {
                        add_bin(&f, "0x0021");
                        line_num += 1;

                        for label in &labels {
                            if label.name == part[4] {
                                let mut goto = "0x".to_owned();
                                goto.push_str(&format!("{:x}", label.line));
                                add_bin(&f, &goto);
                                line_num += 1;
                            }
                        }
                    }
                    _ => {}
                }
            }

            "push" => {
                if part[1].starts_with("a0x") {
                    add_bin(&f, "0x000d");
                    add_bin(&f, &part[1].replace("a0x", "0x"));
                    add_bin(&f, "0x0023");
                    line_num += 3;
                }
                if part[1].starts_with("0x") {
                    add_bin(&f, "0x0000");
                    add_bin(&f,part[1]);
                    add_bin(&f, "0x0023");
                    line_num += 3;
                }
                else {
                    match part[1] {
                        "r0" => {
                            add_bin(&f, "0x0023");
                            line_num += 1;
                        }
                        "r1" => {
                            add_bin(&f, "0x0015");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        "r2" => {
                            add_bin(&f, "0x0016");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        "r3" => {
                            add_bin(&f, "0x0017");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        "r4" => {
                            add_bin(&f, "0x0018");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        "r5" => {
                            add_bin(&f, "0x0019");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        "r6" => {
                            add_bin(&f, "0x001a");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        "r7" => {
                            add_bin(&f, "0x001b");
                            add_bin(&f, "0x0023");
                            line_num += 2;
                        }
                        _ => {}
                    }
                }
            }

            "pop" => {
                add_bin(&f, "0x0024");
                add_bin(&f, "0x001d");
                line_num += 2;
                if part[1].starts_with("a0x") {
                    add_bin(&f, "0x000c");
                    add_bin(&f, &part[1].replace("a0x", "0x"));
                    line_num += 2;
                }
                else {
                    match part[1] {
                        "r1" => {
                            add_bin(&f, "0x000e");
                            line_num += 1;
                        }
                        "r2" => {
                            add_bin(&f, "0x000f");
                            line_num += 1;
                        }
                        "r3" => {
                            add_bin(&f, "0x0010");
                            line_num += 1;
                        }
                        "r4" => {
                            add_bin(&f, "0x0011");
                            line_num += 1;
                        }
                        "r5" => {
                            add_bin(&f, "0x0012");
                            line_num += 1;
                        }
                        "r6" => {
                            add_bin(&f, "0x0013");
                            line_num += 1;
                        }
                        "r7" => {
                            add_bin(&f, "0x0014");
                            line_num += 1;
                        }

                        _ => {}
                    }
                }
            }

            "return" => {
                add_bin(&f, "0x0026");
                line_num += 1;
            }

            "hlt" => {
                add_bin(&f, "0x0027");
                line_num += 1;
            }
            
            "crr" => {
                add_bin(&f, "0x0022");
            }

            _ => {}
        }
    }

    let data: String = fs::read_to_string("boot.bin").expect("Unable to read file");
    let data: String = data.replace("\r", "");
    let data: Vec<&str> = data.split("\n").collect();

    let mut replace: Vec<&str> = vec![];
    let mut with: Vec<String> = vec![];

    for line in data {
        let part: Vec<&str> = line.split(" ").collect();
        if part[0] == "__LABEL__" {
            for label in &labels {
                if label.name == part[1] {
                    let mut goto = "0x".to_owned();
                    goto.push_str(&format!("{:x}", label.line).clone());
                    replace.push(line);
                    with.push(goto);
                }
            }
        }
    }

    let mut data: String = fs::read_to_string("boot.bin").expect("Unable to read file");
    
    let mut i = 0;
    while i < replace.len() {
        data = data.replace(replace[i], &with[i]);
        i += 1;
    }

    let mut file = File::create("boot.bin").expect("Couldn't make file");
    file.write_all(data.as_bytes()).expect("Couldn't write to file");
}
