use std::fs::File;
use std::io::{self, BufRead, BufReader};
use clap::Parser;

/// A cat utility with extended functionality
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number all output lines
    #[arg(short, long)]
    number: bool,

    /// Number non-blank output lines
    #[arg(short = 'b', long)]
    number_nonblank: bool,

    /// Display $ at end of each line
    #[arg(short = 'E', long = "show-ends")]
    show_ends: bool,

    /// Suppress repeated empty output lines
    #[arg(short, long = "squeeze-blank")]
    squeeze_blank: bool,

    /// Display TAB characters as ^I
    #[arg(short = 'T', long = "show-tabs")]
    show_tabs: bool,

    /// Limit output to a maximum number of lines
    #[arg(short = 'm', long, value_name = "NUM")]
    max_lines: Option<usize>,

    /// Files to print
    #[arg(name = "FILE")]
    files: Vec<String>,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if args.files.is_empty() {
        // Handle standard input
        let stdin = io::stdin();
        let reader = stdin.lock();
        cat_input(reader, &args)?;
    } else {
        // Handle file inputs
        for file_path in &args.files {
            match File::open(file_path) {
                Ok(file) => {
                    let reader = BufReader::new(file);
                    if let Err(e) = cat_input(reader, &args) {
                        eprintln!("Error reading from {}: {}", file_path, e);
                    }
                }
                Err(e) => eprintln!("Error opening {}: {}", file_path, e),
            }
        }
    }

    Ok(())
}

fn cat_input<R: BufRead>(reader: R, args: &Args) -> io::Result<()> {
    let mut line_count = 0;
    let mut last_line_empty = false;

    for (index, line) in reader.lines().enumerate() {
        let mut line = line?;

        if args.squeeze_blank && line.trim().is_empty() && last_line_empty {
            continue;
        }

        if args.show_tabs {
            line = line.replace('\t', "^I");
        }

        if args.show_ends {
            line.push('$');
        }

        let should_number = if args.number_nonblank {
            !line.trim().is_empty()
        } else {
            args.number
        };

        if should_number {
            print!("{:6}\t", index + 1);
        }

        println!("{}", line);

        line_count += 1;
        if let Some(max) = args.max_lines {
            if line_count >= max {
                break;
            }
        }

        last_line_empty = line.trim().is_empty();
    }

    Ok(())
}



// cargo run -- -n sample.txt sample1.txt       # Number all lines
// cargo run -- -b sample.txt sample1.txt       # Number non-blank lines
// cargo run -- -E sample.txt sample1.txt       # Show line endings
// cargo run -- -s sample.txt sample1.txt       # Squeeze blank lines
// cargo run -- -T sample.txt sample1.txt       # Show tabs
// cargo run -- -m 10 sample.txt sample1.txt    # Limit to 10 lines
// cargo run -- -nET fsample.txt sample1.txt    # Combine multiple options