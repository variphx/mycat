use mycat::Arguments;
use std::{io::Write, process};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args = Arguments::new(&args);

    let file_contents = std::fs::read_to_string(args.filename).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    let mut stdout = std::io::stdout().lock();
    for line in file_contents.lines() {
        writeln!(&mut stdout, "{}", line).unwrap();
    }
}
