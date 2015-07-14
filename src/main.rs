use std::io::{self, stdin, stdout, Write};

// TODO(tsion): Use the readline library.
fn prompt(line: &mut String) -> io::Result<usize> {
    let current_dir = try!(std::env::current_dir());
    print!("{}> ", current_dir.display());
    try!(stdout().flush());
    stdin().read_line(line)
}

#[derive(Clone, Debug)]
enum Ast {
    Empty,
    Call { command: String, args: Vec<String> }
}

fn parse(line: &str) -> Ast {
    let mut words = line.split_whitespace();

    match words.next() {
        Some(command) => Ast::Call {
            command: String::from(command),
            args: words.map(String::from).collect()
        },
        None => Ast::Empty
    }
}

fn execute(ast: &Ast) -> io::Result<()> {
    match *ast {
        Ast::Empty => Ok(()),
        Ast::Call { ref command, ref args } => {
            std::process::Command::new(command).args(args).status().map(|_| ())
        },
    }
}

fn main() {
    let mut line = String::new();
    loop {
        prompt(&mut line).unwrap();
        let ast = parse(&line);
        execute(&ast).unwrap();
        line.clear();
    }
}
