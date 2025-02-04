use std::io;
use water_sort::{Color, WaterSolver, WaterSorting};

pub fn main() {
    let mut w = WaterSorting::new();
    w.init_bottle_with_four_colors(Color::Red, Color::Magenta, Color::Magenta, Color::Orange);
    w.init_bottle_with_four_colors(Color::Yellow, Color::Brown, Color::Blue, Color::Green);
    w.init_bottle_with_four_colors(Color::Brown, Color::Red, Color::Orange, Color::Red);
    w.init_bottle_with_four_colors(Color::Brown, Color::Blue, Color::Blue, Color::Orange);
    w.init_bottle_with_four_colors(Color::Green, Color::Green, Color::Orange, Color::Yellow);
    w.init_bottle_with_four_colors(Color::Red, Color::Yellow, Color::Magenta, Color::Magenta);
    w.init_bottle_with_four_colors(Color::Blue, Color::Green, Color::Brown, Color::Yellow);
    w.init_empty_bottle();
    w.init_empty_bottle();
    loop {
        println!("{}", w);
        if w.win() {
            println!("You Won!");
            break;
        }
        println!();
        println!("Provide next move (src -> desc) or solve: ");
        let mut line: String = Default::default();
        let result = io::stdin().read_line(&mut line);

        if result.is_err() {
            break;
        }

        if line.trim_end() == "solve" {
            let solver = WaterSolver::new(&w);
            let solution = solver.solve(20);
            println!("Solution: ");
            solution.iter().for_each(|pour| {
                let from = pour.from;
                let to = pour.to;
                println!("{:?}", pour);
                w.pour(from, to);
            } );

        } else {
            let moves = line
                .trim_end()
                .split("->")
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>();
            if moves.iter().count() != 2 {
                println!("Wrong move!");
                continue;
            }

            let source_no = moves[0] - 1;
            let destination_no = moves[1] - 1;
            println!("Pouring...");
            w.pour(source_no, destination_no);
        }
    }
}