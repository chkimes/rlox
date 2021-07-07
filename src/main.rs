mod chunk;
mod compiler;
mod debug;
mod scanner;
mod value;
mod vm;

use chunk::*;
use std::io::Write;
use vm::*;

fn main() {
    let mut chunk = Chunk::new();
    let mut vm = VM::new();
    let args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        repl(&mut vm);
    } else if args.len() == 2 {
        run_file(&args[1], &mut vm);
    } else {
        println!("Usage: rlox [path]");
        std::process::exit(64);
    }
}

fn repl(vm: &mut VM) {
    let mut line = String::new();
    loop {
        print!("> ");
        std::io::stdout().lock().flush().unwrap();

        match std::io::stdin().read_line(&mut line) {
            Ok(_)  => vm.interpret(&line),
            Err(_) => { print!("\n"); break }
        };

        line.clear();
    }
}

fn run_file(path: &String, vm: &mut VM) {
    let source = std::fs::read_to_string(path).unwrap();
    let result = vm.interpret(&source);

    match result {
        InterpretResult::CompileError => std::process::exit(65),
        InterpretResult::RuntimeError => std::process::exit(70),
        _ => { }
    }
}
