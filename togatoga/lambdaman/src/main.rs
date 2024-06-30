use std::path::PathBuf;

use clap::Parser;
use lambdaman::grid_graph::LambdamanCommand;
use lambdaman::scanner;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct LamdamanSolverCli {
    /// Path to the input file
    /// e.g. data/lamdaman/lamadaman1.in
    #[arg(short, long)]
    input: PathBuf,

    /// Path to the output file
    /// e.g. data/lamdaman/lamadaman1.out
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Group size
    #[arg(short, long, default_value = "1")]
    group_size: usize,

    #[arg(short, long, default_value = "false")]
    verbose: bool,
}

fn main() {
    let cli = LamdamanSolverCli::parse();
    let input = std::fs::File::open(cli.input).unwrap();
    let mut scn = scanner::Scanner::new(std::io::BufReader::new(input));
    let mut board = vec![];
    // map while until scn.read_line() is empty
    loop {
        let line = scn.read_line();
        if line.is_empty() {
            break;
        }
        board.push(line.chars().collect::<Vec<char>>());
    }
    let config = lambdaman::Config {
        group_size: cli.group_size,
        verbose: cli.verbose,
    };
    let cmds = lambdaman::Solver::new(board).solve(&config);

    let cmd_str = cmds
        .iter()
        .map(|cmd| match cmd {
            LambdamanCommand::Up => "U",
            LambdamanCommand::Down => "D",
            LambdamanCommand::Right => "R",
            LambdamanCommand::Left => "L",
        })
        .collect::<String>();

    // stdout or write file

    match cli.output {
        Some(file) => {
            std::fs::write(file, cmd_str).expect("Failed to write file");
        }
        None => {
            println!("{}", cmd_str);
        }
    }
    eprintln!("Cmd size: {}", cmds.len());
}
