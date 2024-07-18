use std::io;
use water_sort::{Color, WaterSorting};

pub fn main() {
    let mut w = WaterSorting::new();
    w.init_bottle_with_one_color(Color::Red);
    w.init_bottle_with_one_color(Color::Magenta);
    w.init_bottle_with_three_colors(Color::Red, Color::Red, Color::Red);
    w.init_bottle_with_three_colors(Color::Magenta, Color::Magenta, Color::Magenta);
    loop {
        println!("{}", w);
        if w.win() {
            println!("You Won!");
            break;
        }
        println!();
        println!("Provide source bottle: ");
        let mut line: String = Default::default();
        let result = io::stdin().read_line(&mut line);

        if result.is_err() {
            break;
        }

        let source_no = line.trim().parse::<u8>().unwrap() - 1;
        println!("Provide destination bottle: ");
        let mut line: String = Default::default();
        let result = io::stdin().read_line(&mut line);
        if result.is_err() {
            break;
        }
        let destination_no = line.trim().parse::<u8>().unwrap() - 1;
        println!("Pouring...");
        w.pour(source_no, destination_no);
    }
}