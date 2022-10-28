use std::{thread, time};
use std::fs;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, PressEvent, ReleaseEvent, RenderArgs, RenderEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL, Texture, TextureSettings};

use std::path::Path;
use std::io;
use std::io::prelude::*;


static mut RAM: [u16; 0x10000] = [0x0000; 0x10000];

unsafe fn ram_get(address: u16) -> u16 {
    return RAM[address as usize];
}

unsafe fn ram_put(address: u16, value: u16) {
    RAM[address as usize] = value;
}


// GPU

    pub struct App {
        gl: GlGraphics,

        font: Vec<Texture>,
        pointer: u16,
    }

    impl App {

        unsafe fn render_update(&mut self, args: &RenderArgs) {

            { //CPU
                let mut i = 0;
                while i < 20 {
                    thread::spawn(|| {exe()});
                    i += 1;
                }
            }

            { // Text Mode
                if ram_get(0xeffe) == 0x0000 {
                    use graphics::*;
                    self.gl.draw(args.viewport(), |_c, gl| {clear([0.0, 0.0, 0.0, 1.0], gl);});

                    let mut address = 0xeffe;
                    
                    let mut x: f64 = 0.0;
                    let mut y: f64 = -1.0;

                    while address < 0xffff {
                        address += 0x0001;

                        x += 1.0;
                        if (address - 0xefff) as f64 % 80.0 == 0.0 {
                            x = 0.0;
                            y += 1.0;
                        }

                        let image = Image::new().rect([16.0 * x + 3.0, 19.5 * y, 8.0, 14.0]);

                        let mut texture = &self.font[&self.font.len() - 1];

                        if &(ram_get(address) as usize) < &self.font.len() {
                            texture = &self.font[ram_get(address) as usize];
                        }

                        
                        self.gl.draw(args.viewport(), |c, gl| {image.draw(texture, &DrawState::new_alpha(), c.transform, gl);});
                    }
                }
            }

            { // Graphics Mode
                if ram_get(0xeffe) == 0x0001 {
                    use graphics::*;

                    self.gl.draw(args.viewport(), |_c, gl| {clear([0.0, 0.0, 0.0, 1.0], gl);});
                    match ram_get(self.pointer) {
                        0x0000 => {

                            let x_1 = ram_get(self.pointer + 1) as f64;
                            let y_1 = ram_get(self.pointer + 2) as f64;
                            let x_2 = ram_get(self.pointer + 3) as f64;
                            let y_2 = ram_get(self.pointer + 4) as f64;

                            let r = (ram_get(self.pointer + 5) as f32) / 0xffff as f32;
                            let g = (ram_get(self.pointer + 6) as f32) / 0xffff as f32;
                            let b = (ram_get(self.pointer + 7) as f32) / 0xffff as f32;
                            let t = (ram_get(self.pointer + 8) as f32) / 0xffff as f32;

                            self.gl.draw(args.viewport(), |c, gl| {rectangle(
                                [r, g, b, t],
                                [x_1, y_1, x_2, y_2],
                                c.transform,gl,);});

                            self.pointer += 8;
                        }
                        _ => {}
                    }
                    self.pointer += 0x0001;
                    
                    if self.pointer > ram_get(0xefff) {
                        self.pointer = 0xf000;
                    }
                }
            }
        
        }

        unsafe fn press(&mut self, args: &Button) {       
            if let &Button::Keyboard(key) = args {
                match key {
                    Key::Q => {ram_put(0xeffd, 0x0001)}
                    Key::W => {ram_put(0xeffc, 0x0001)}
                    Key::E => {ram_put(0xeffb, 0x0001)}
                    Key::R => {ram_put(0xeffa, 0x0001)}
                    Key::T => {ram_put(0xeff9, 0x0001)}
                    Key::Z => {ram_put(0xeff8, 0x0001)}
                    Key::U => {ram_put(0xeff7, 0x0001)}
                    Key::I => {ram_put(0xeff6, 0x0001)}
                    Key::O => {ram_put(0xeff5, 0x0001)}
                    Key::P => {ram_put(0xeff4, 0x0001)}
                    Key::A => {ram_put(0xeff3, 0x0001)}
                    Key::S => {ram_put(0xeff2, 0x0001)}
                    Key::D => {ram_put(0xeff1, 0x0001)}
                    Key::F => {ram_put(0xeff0, 0x0001)}
                    Key::G => {ram_put(0xefef, 0x0001)}
                    Key::H => {ram_put(0xefee, 0x0001)}
                    Key::J => {ram_put(0xefed, 0x0001)}
                    Key::K => {ram_put(0xefec, 0x0001)}
                    Key::L => {ram_put(0xefeb, 0x0001)}
                    Key::Y => {ram_put(0xefea, 0x0001)}
                    Key::X => {ram_put(0xefe9, 0x0001)}
                    Key::C => {ram_put(0xefe8, 0x0001)}
                    Key::V => {ram_put(0xefe7, 0x0001)}
                    Key::B => {ram_put(0xefe6, 0x0001)}
                    Key::N => {ram_put(0xefe5, 0x0001)}
                    Key::M => {ram_put(0xefe4, 0x0001)}
                    _=>{}
                }
            }
        }

        unsafe fn release(&mut self, args: &Button) {       
            if let &Button::Keyboard(key) = args {
                match key {
                    Key::Q => {ram_put(0xeffd, 0x0000)}
                    Key::W => {ram_put(0xeffc, 0x0000)}
                    Key::E => {ram_put(0xeffb, 0x0000)}
                    Key::R => {ram_put(0xeffa, 0x0000)}
                    Key::T => {ram_put(0xeff9, 0x0000)}
                    Key::Z => {ram_put(0xeff8, 0x0000)}
                    Key::U => {ram_put(0xeff7, 0x0000)}
                    Key::I => {ram_put(0xeff6, 0x0000)}
                    Key::O => {ram_put(0xeff5, 0x0000)}
                    Key::P => {ram_put(0xeff4, 0x0000)}
                    Key::A => {ram_put(0xeff3, 0x0000)}
                    Key::S => {ram_put(0xeff2, 0x0000)}
                    Key::D => {ram_put(0xeff1, 0x0000)}
                    Key::F => {ram_put(0xeff0, 0x0000)}
                    Key::G => {ram_put(0xefef, 0x0000)}
                    Key::H => {ram_put(0xefee, 0x0000)}
                    Key::J => {ram_put(0xefed, 0x0000)}
                    Key::K => {ram_put(0xefec, 0x0000)}
                    Key::L => {ram_put(0xefeb, 0x0000)}
                    Key::Y => {ram_put(0xefea, 0x0000)}
                    Key::X => {ram_put(0xefe9, 0x0000)}
                    Key::C => {ram_put(0xefe8, 0x0000)}
                    Key::V => {ram_put(0xefe7, 0x0000)}
                    Key::B => {ram_put(0xefe6, 0x0000)}
                    Key::N => {ram_put(0xefe5, 0x0000)}
                    Key::M => {ram_put(0xefe4, 0x0000)}
                    _=>{}
                }
            }
        }

    }

    unsafe fn render() {
        let mut window: GlutinWindow = WindowSettings::new("Virtual Machine", [1280.0, 1024.0])
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap();

        let opengl = OpenGL::V3_2;

        let path: String = "font\\".to_owned();

        let font_paths: Vec<&str> = vec![
            "0",
            "1",
            "2",
            "3",
            "4",
            "5",
            "6",
            "7",
            "8",
            "9",
            "a", "-a",
            "b", "-b",
            "c", "-c",
            "d", "-d",
            "e", "-e",
            "f", "-f",
            "g", "-g",
            "h", "-h",
            "i", "-i",
            "j", "-j",
            "k", "-k",
            "l", "-l",
            "m", "-m",
            "n", "-n",
            "o", "-o",
            "p", "-p",
            "q", "-q",
            "r", "-r",
            "s", "-s",
            "t", "-t",
            "u", "-u",
            "v", "-v",
            "w", "-w",
            "x", "-x",
            "y", "-y",
            "z", "-z",

            "empty",
            "unknown",
        ];

        let mut font: Vec<Texture> = vec![];

        for char in font_paths {
            let mut temp_path = path.clone();
            temp_path.push_str(char);
            temp_path.push_str("-export.png");
            font.push(Texture::from_path(Path::new(&temp_path), &TextureSettings::new()).unwrap())
        }
        
        let mut app = App {
            gl: GlGraphics::new(opengl),

            font: font,
            pointer: 0xf000,
        };

        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut window) {
            if let Some(r) = e.render_args() {
                app.render_update(&r);
            }
            if let Some(b) = e.press_args() {
                app.press(&b);
            }

            if let Some(b) = e.release_args() {
                app.release(&b);
            }
        }
    }

// GPU END


// CPU

    fn _pause() {
        let mut stdin = io::stdin();
        let mut stdout = io::stdout();

        write!(stdout, "").unwrap();
        stdout.flush().unwrap();

        let _ = stdin.read(&mut [0u8]).unwrap();
    }

    #[derive(Debug, Clone)]
    struct Cpu {
        r0: u16,    pointer: u16,
        r2: u16,    
        r1: u16,    stack: Vec<u16>,
        r3: u16,    stack_ptr: u16,
        r4: u16,
        r5: u16,    jmp_from: Vec<u16>,
        r6: u16, 
        r7: u16,    result: u16,
    }

    static mut CPU: Cpu = Cpu {
        r0: 0x0000, pointer: 0x0000,
        r1: 0x0000, 
        r2: 0x0000, stack: vec![],
        r3: 0x0000, stack_ptr: 0x0000,
        r4: 0x0000,
        r5: 0x0000, jmp_from: vec![],
        r6: 0x0000, 
        r7: 0x0000, result: 0x0000,
    };

    unsafe fn debug(cpu: Cpu) {
        println!("r0: 0x{:0x} Pointer: 0x{:0x}", cpu.r0, cpu.pointer);
        println!("r1: 0x{:0x}", cpu.r1);
        println!("r2: 0x{:0x} Stack: ...", cpu.r2);
        println!("r3: 0x{:0x} Stack_ptr: 0x{:0x}", cpu.r3, cpu.stack_ptr);
        println!("r4: 0x{:0x}", cpu.r4);
        println!("r5: 0x{:0x} Jmp from: {:?}", cpu.r5, cpu.jmp_from);
        println!("r6: 0x{:0x}", cpu.r6);
        println!("r7: 0x{:0x} Result: 0x{:0x}", cpu.r7, cpu.result);
        println!("\n");
        println!("Value at address: 0x{:0x} ~ 0x{:0x}", cpu.pointer, ram_get(cpu.pointer));
        println!("\n");

        //_pause();

        thread::sleep(time::Duration::from_millis(800));

    }

    unsafe fn exe() {
        let cmd: u16 = ram_get(CPU.pointer);

        //debug(CPU.clone());

        match cmd {
            0x0000 => {
                CPU.pointer += 0x0001;
                CPU.r0 = ram_get(CPU.pointer);
            }
            0x0001 => {
                CPU.pointer += 0x0001;
                CPU.r1 = ram_get(CPU.pointer);
            }
            0x0002 => {
                CPU.pointer += 0x0001;
                CPU.r2 = ram_get(CPU.pointer);
            }
            0x0003 => {
                CPU.pointer += 0x0001;
                CPU.r3 = ram_get(CPU.pointer);
            }
            0x0004 => {
                CPU.pointer += 0x0001;
                CPU.r4 = ram_get(CPU.pointer);
            }
            0x0005 => {
                CPU.pointer += 0x0001;
                CPU.r5 = ram_get(CPU.pointer);
            }
            0x0006 => {
                CPU.pointer += 0x0001;
                CPU.r6 = ram_get(CPU.pointer);
            }
            0x0007 => {
                CPU.pointer += 0x0001;
                CPU.r7 = ram_get(CPU.pointer);
            }

            0x0008 => {
                CPU.result = CPU.r0 + CPU.r1;
            }
            0x0009 => {
                CPU.result = CPU.r0 - CPU.r1;
            }
            0x000a => {
                CPU.result = CPU.r0 * CPU.r1;
            }
            0x000b => {
                CPU.result = CPU.r0 / CPU.r1;
            }

            0x000c => {
                CPU.pointer += 0x0001;
                ram_put(ram_get(CPU.pointer), CPU.r0);
            }

            0x000d => {
                CPU.pointer += 0x0001;
                CPU.r0 = ram_get(CPU.pointer);
            }

            0x000e => {
                CPU.r1 = CPU.r0;
            }
            0x000f => {
                CPU.r2 = CPU.r0;
            }
            0x0010 => {
                CPU.r3 = CPU.r0;
            }
            0x0011 => {
                CPU.r4 = CPU.r0;
            }
            0x0012 => {
                CPU.r5 = CPU.r0;
            }
            0x0013 => {
                CPU.r6 = CPU.r0;
            }
            0x0014 => {
                CPU.r7 = CPU.r0;
            }

            0x0015 => {
                CPU.r0 = CPU.r1;
            }
            0x0016 => {
                CPU.r0 = CPU.r2;
            }
            0x0017 => {
                CPU.r0 = CPU.r3;
            }
            0x0018 => {
                CPU.r0 = CPU.r4;
            }
            0x0019 => {
                CPU.r0 = CPU.r5;
            }
            0x001a => {
                CPU.r0 = CPU.r6;
            }
            0x001b => {
                CPU.r0 = CPU.r7;
            }

            0x001c => {
                CPU.pointer += 0x0001;
                CPU.jmp_from.push(CPU.pointer + 1);
                let temp = ram_get(CPU.pointer) - 0x0001;
                CPU.pointer = temp;
            }

            0x001d => {
                CPU.r0 = CPU.result;
            }

            0x001e => {
                CPU.pointer += 0x0001;
                if CPU.r0 == CPU.r1 {
                    CPU.jmp_from.push(CPU.pointer + 1);
                    let temp = ram_get(CPU.pointer) - 0x0001;
                    CPU.pointer = temp;
                }
            }
            0x001f => {
                CPU.pointer += 0x0001;
                if CPU.r0 > CPU.r1 {
                    CPU.jmp_from.push(CPU.pointer + 1);
                    let temp = ram_get(CPU.pointer) - 0x0001;
                    CPU.pointer = temp;
                }
            }
            0x0020 => {
                CPU.pointer += 0x0001;
                if CPU.r0 < CPU.r1 {
                    CPU.jmp_from.push(CPU.pointer + 1);
                    let temp = ram_get(CPU.pointer) - 0x0001;
                    CPU.pointer = temp;
                }
            }
            0x0021 => {
                CPU.pointer += 0x0001;
                if CPU.r0 != CPU.r1 {
                    CPU.jmp_from.push(CPU.pointer + 1);
                    let temp = ram_get(CPU.pointer) - 0x0001;
                    CPU.pointer = temp;
                }
            }

            0x0022 => {
                let mut address = 0xeffe;
                while address < 0xffff {
                    address += 0x0001;
                    ram_put(address, 0x003e);
                }
            }

            0x0023 => {
                CPU.stack.push(CPU.r0);
                CPU.stack_ptr += 1;
            }
            0x0024 => {
                CPU.stack_ptr -= 0x0001;
                CPU.result = CPU.stack[CPU.stack_ptr as usize];
            }

            0x0025 => {
                ram_put(CPU.r1, CPU.r0);
            }

            0x0026 => {
                CPU.pointer = CPU.jmp_from.pop().unwrap() - 0x0001;
            }

            0x0027 => {
                CPU.pointer = 0xffff;
            }

            _ => {}
        }
        CPU.pointer += 0x0001;
    }
    
// CPU END

fn main() {
    unsafe {
        let data = fs::read_to_string("boot.bin").expect("Unable to read boot file");
        let boot_instrc: Vec<&str> = data.split("\n").collect();

        let mut i = 0;
        while i < boot_instrc.len() {

            let raw: &str = boot_instrc[i];
            let raw = raw.replace("\r", "");

            if raw != "" {
                let without_prefix = raw.trim_start_matches("0x");
                let z = u16::from_str_radix(without_prefix, 16);

                let value: u16 = z.unwrap();

                ram_put(i as u16, value);
            }
            i += 1;
        }   
        render()
    }
}
