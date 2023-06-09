use clap::Parser;

mod solve;
use crate::solve::solve;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    grid: String,
}

fn main() {
    // Current local usage:
    // cargo run -- -g 530070000600195000098000060800060003400803001700020006060000280000419005000080079
    let args = Args::parse();

    // Parse grid input of form 123456789... where 0 is an empty cell
    let grid: Vec<u8> = args
        .grid
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect();

    // Tranform grid into form [[u8; 9]; 9]
    let grid: [[u8; 9]; 9] = grid
        .chunks(9)
        .map(|chunk| {
            let mut row = [0; 9];
            row.copy_from_slice(chunk);
            row
        })
        .collect::<Vec<[u8; 9]>>()
        .try_into()
        .unwrap();

    println!("{:?}", solve(grid));
}
