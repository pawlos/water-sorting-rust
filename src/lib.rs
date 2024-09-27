use core::fmt::Debug;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    Empty,
    Blue,
    Red,
    Gray,
    Orange,
    Brown,
    Yellow,
    Green,
    Magenta,
    Teal
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Blue => write!(f, "🟦"),
            Self::Red => write!(f, "🟥"),
            Self::Gray => write!(f, "🔳"),
            Self::Orange => write!(f, "🟧"),
            Self::Brown => write!(f, "🟫"),
            Self::Yellow => write!(f, "🟨"),
            Self::Green => write!(f, "🟩"),
            Self::Magenta => write!(f, "🟪"),
            Self::Teal => write!(f, "⏹"),
            Self::Empty => write!(f, ""),
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq, Copy, Clone)]
struct Bottle {
    index: Option<usize>,
    bottom: Option<Color>,
    l1: Option<Color>,
    l2: Option<Color>,
    top: Option<Color>,
}

#[wasm_bindgen]
#[derive(Debug, Copy, Clone)]
struct Pour {
    from: usize,
    to: usize
}

impl Pour {
    pub fn new(from: usize, to: usize) -> Self {
        Pour { from, to }
    }
}

impl PartialEq for Pour {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Debug for Bottle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:#?}{:#?}{:#?}{:#?}",
            self.bottom.unwrap_or(Color::Empty),
            self.l1.unwrap_or(Color::Empty),
            self.l2.unwrap_or(Color::Empty),
            self.top.unwrap_or(Color::Empty)
        )
    }
}

impl Display for Bottle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}",self)
    }
}

impl Bottle {
    pub fn new(
        index: Option<usize>,
        bottom: Option<Color>,
        l1: Option<Color>,
        l2: Option<Color>,
        top: Option<Color>,
    ) -> Self {
        Bottle {
            index,
            bottom,
            l1,
            l2,
            top,
        }
    }

    pub fn empty(idx: usize) -> Self {
        Bottle::new(Some(idx), None, None, None, None)
    }

    pub fn with_one_color(idx: usize, c: Color) -> Self {
        Bottle::new(Some(idx), Some(c), None, None, None)
    }

    pub fn with_two_colors(idx: usize, b: Color, l1: Color) -> Self {
        Bottle::new(Some(idx), Some(b), Some(l1), None, None)
    }

    pub fn with_three_colors(idx: usize, b: Color, l1: Color, l2: Color) -> Self {
        Bottle::new(Some(idx), Some(b), Some(l1), Some(l2), None)
    }

    pub fn with_four_colors(idx: usize, b: Color, l1: Color, l2: Color, t: Color) -> Self {
        Bottle::new(Some(idx), Some(b), Some(l1), Some(l2), Some(t))
    }

    pub fn is_empty(&self) -> bool {
        matches!(
            (self.bottom, self.l1, self.l2, self.top),
            (None, None, None, None)
        )
    }

    pub fn is_full(&self) -> bool {
        matches!((self.bottom, self.l1, self.l2, self.top), (Some(_), Some(_), Some(_), Some(_)))
    }

    pub fn top_color(&self) -> Option<Color> {
        match (self.top, self.l2, self.l1, self.bottom) {
            (None, None, None, None) => None,
            (None, None, None, s) => s,
            (None, None, s, _) => s,
            (None, s, _, _) => s,
            (s, _, _, _) => s,
        }
    }

    fn pop(&mut self) {
        match (self.top, self.l2, self.l1, self.bottom) {
            (None, None, None, None) => panic!("Should not happen"),
            (None, None, None, _) => self.bottom = None,
            (None, None, _, _) => self.l1 = None,
            (None, _, _, _) => self.l2 = None,
            (_, _, _, _) => self.top = None,
        };
    }

    pub fn pour(&mut self, c: Color) -> bool {
        match (self.top, self.l2, self.l1, self.bottom) {
            (None, None, None, None) => {
                self.bottom = Some(c);
                true
            }
            (None, None, None, Some(s)) if s == c => {
                self.l1 = Some(c);
                true
            }
            (None, None, Some(s), Some(_)) if s == c => {
                self.l2 = Some(c);
                true
            }
            (None, Some(s), Some(_), Some(_)) if s == c => {
                self.top = Some(c);
                true
            }
            _ => false,
        }
    }

    pub fn is_empty_or_one_color(&self) -> bool {
        match (self.bottom, self.l1, self.l2, self.top) {
            (None, None, None, None) => true,
            (Some(b), Some(l1), Some(l2), Some(t)) if t == l2 && l2 == l1 && l1 == b => true,
            (Some(b), Some(l1), Some(l2), None) if b == l2 && l2 == l1 => true,
            (Some(b), Some(l1), None, None) if b == l1 => true,
            (Some(_), None, None, None) => true,
            _ => false,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct WaterSorting {
    bottles: Vec<Bottle>,
    old_state: Option<Vec<Bottle>>,
    bottles_serialized: Vec<u8>,
}

impl Debug for WaterSorting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WaterSorting")
            .field("bottles", &self.bottles)
            .finish()
    }
}

impl Display for WaterSorting {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.bottles.iter().enumerate().for_each(|(i, x)|{
            f.write_fmt(format_args!("{:?}: {:?}\n", i+1, x)).expect("");
            ()
        });
        Ok(())
    }
}

#[wasm_bindgen]
impl WaterSorting {
    pub fn new() -> Self {
        WaterSorting {
            bottles: Vec::with_capacity(4),
            old_state: None,
            bottles_serialized: Vec::with_capacity(16)
        }
    }

    pub fn pour(&mut self, from_index: u8, to_index: u8) {
        self.old_state = Some(self.bottles.to_vec());
        loop {
            match self.bottles[from_index as usize].top_color() {
                None => break,
                Some(b) => {
                    let to_b = &mut self.bottles[to_index as usize];
                    let r = to_b.pour(b);
                    if !r {
                        break;
                    }
                    self.bottles[from_index as usize].pop();
                }
            }
        }
    }

    pub fn undo(&mut self) {
        match &self.old_state {
            None => {}
            Some(old) => {
                self.bottles = old.to_vec();
                self.old_state = None
            }
        }
    }

    pub fn undo_available(&self) -> bool {
        self.old_state.is_some()
    }

    pub fn win(&self) -> bool {
        self.bottles.iter().all(|b| b.is_empty_or_one_color() && (b.is_empty() || b.is_full()))
    }

    pub fn init_empty_bottle(&mut self) {
        self.bottles.push(Bottle::empty(self.bottles.iter().count()))
    }

    pub fn init_bottle_with_one_color(&mut self, c: Color) {
        self.bottles.push(Bottle::with_one_color(self.bottles.iter().count(), c))
    }

    pub fn init_bottle_with_two_colors(&mut self, b: Color, l1: Color) {
        self.bottles.push(Bottle::with_two_colors(self.bottles.iter().count(), b, l1))
    }

    pub fn init_bottle_with_three_colors(&mut self, b: Color, l1: Color, l2: Color) {
        self.bottles.push(Bottle::with_three_colors(self.bottles.iter().count(), b, l1, l2))
    }

    pub fn init_bottle_with_four_colors(&mut self, b: Color, l1: Color, l2: Color, t: Color) {
        self.bottles.push(Bottle::with_four_colors(self.bottles.iter().count(), b, l1, l2, t))
    }

    pub fn move_available(&self) -> bool {
        if self.bottles.iter().any(|b| b.is_empty()) {
            return true;
        }

        let top_colors = self.top_colors();
        for (src, t1) in top_colors.clone() {
            let (src_color, src_is_full) = t1;
            for (dst,t2) in top_colors.clone() {
                let (dst_color, dst_is_full) = t2;
                if src != dst && src_color == dst_color {
                    return !src_is_full || !dst_is_full
                }
            }
        }
        false
    }

    fn next_available_move(&self) -> Option<Pour> {
        self.next_available_move_except(&Vec::new())
    }

    fn next_available_moves(&self) -> Vec<Pour> {
        let mut moves = Vec::new();
        while let Some(pour) = self.next_available_move_except(&moves) {
            moves.push(pour);
        }

        moves
    }

    fn next_available_move_except(&self, moves: &Vec<Pour>) -> Option<Pour> {
        let empty_bottles = self.bottles.iter().filter(|b| b.is_empty());

        for empty in empty_bottles {
            let non_empty_bottles = self.bottles.iter().filter(|b| !b.is_empty());
            for non_empty in non_empty_bottles {
                let pour = Pour::new(non_empty.index.unwrap(), empty.index.unwrap());
                if !moves.contains(&pour) {
                    return Some(pour);
                }
            }
        }

        let top_colors = self.top_colors();
        for (src, t1) in top_colors.clone() {
            let (src_color, _) = t1;
            for (dst,t2) in top_colors.clone() {
                let (dst_color, dst_is_full) = t2;
                let pour = Pour::new(src, dst);
                if src != dst && src_color == dst_color && !dst_is_full && !moves.contains(&pour) {
                    return Some(pour)
                }
            }
        }
        None
    }

    fn top_colors(&self) -> Vec<(usize, (Color, bool))> {
        self.bottles
            .iter()
            .filter(|b| !b.is_empty())
            .map(|b| (b.top_color().unwrap(), b.is_full()))
            .enumerate()
            .collect::<Vec<(usize, (Color, bool))>>()
    }

    fn map_color_to_u8(c: Option<Color>) -> u8 {
        match c {
            None => 0,
            Some(s) => {
                match s {
                    Color::Empty => {0}
                    Color::Blue => {1}
                    Color::Red => {2}
                    Color::Gray => {3}
                    Color::Orange => {4}
                    Color::Brown => {5}
                    Color::Yellow => {6}
                    Color::Green => {7}
                    Color::Magenta => {8},
                    Color::Teal => {9},
                }
            }
        }
    }

    pub fn bottles(&mut self) -> *const u8 {

        self.bottles_serialized = self.bottles
            .iter()
            .flat_map(|b|  [Self::map_color_to_u8(b.bottom),
                            Self::map_color_to_u8(b.l1),
                            Self::map_color_to_u8(b.l2),
                            Self::map_color_to_u8(b.top)]).collect::<Vec<_>>();
        self.bottles_serialized.as_ptr()
    }

    pub fn bottles_count(&self) -> usize {
        self.bottles.iter().count()
    }

    pub fn reset(&mut self) {
        self.bottles.clear();
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[wasm_bindgen]
struct WaterSolver {
    level: WaterSorting,
}

impl WaterSolver {
    pub fn new(w: &WaterSorting) -> WaterSolver {
        WaterSolver{ level: w.clone() }
    }

    pub fn solve(&self) -> Vec<Pour> {
        let moves = Vec::new();
        let new_w = self.level.clone();
        self.solve_internal(new_w, moves, 0).unwrap_or_else(|| Vec::new())
    }

    fn solve_internal(&self, w: WaterSorting,  moves: Vec<Pour>, count: u8) -> Option<Vec<Pour>> {
        if w.win() {
            return Some(moves)
        }
        if count > 8 {
            return None
        }
        let next_available_moves = w.next_available_moves();
        for next_move in next_available_moves {
            let mut new_moves = moves.clone();
            let mut new_w = w.clone();
            new_moves.push(Pour::new(next_move.from, next_move.to));
            new_w.pour(next_move.from as u8, next_move.to as u8);
            let result = self.solve_internal(new_w, new_moves, count+1);
            match result {
                None => {}
                r => return r
            }
        }
        None
    }
}

#[cfg(test)]
mod next_available_moves_tests {
    use crate::{Color, Pour, WaterSorting};

    #[test]
    fn next_available_moves_returns_all_the_moves() {
        let mut w = WaterSorting::new();

        w.init_bottle_with_one_color(Color::Green);
        w.init_bottle_with_one_color(Color::Green);

        let moves = w.next_available_moves();

        let available_moves = [Pour::new(0,1), Pour::new(1,0)].to_vec();
        assert_eq!(moves, available_moves);
    }

    #[test]
    fn next_available_moves_with_empty_bottle_returns_all_the_moves() {
        let mut w = WaterSorting::new();

        w.init_bottle_with_one_color(Color::Green);
        w.init_bottle_with_one_color(Color::Green);
        w.init_empty_bottle();

        let moves = w.next_available_moves();

        let available_moves = [Pour::new(0,2), Pour::new(1,2), Pour::new(0,1), Pour::new(1,0)].to_vec();
        assert_eq!(moves, available_moves);
    }
}

#[cfg(test)]
mod auto_solve_tests {
    use crate::{Color, WaterSolver, WaterSorting};
    use crate::Pour;

    #[test]
    fn if_no_move_available_next_move_returns_none() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Blue, Color::Blue, Color::Blue, Color::Blue);
        w.init_bottle_with_four_colors(Color::Blue, Color::Blue, Color::Blue, Color::Red);

        let next_move = w.next_available_move();
        assert!(next_move.is_none())
    }

    #[test]
    fn if_first_is_full_bottle_and_second_one_is_empty_one_move_is_returned_as_from_0_to_1() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Blue, Color::Blue, Color::Blue, Color::Blue);
        w.init_empty_bottle();

        let next_move = w.next_available_move();
        assert!(next_move.is_some());
        assert_eq!(next_move.unwrap(), Pour::new(0usize, 1usize))
    }

    #[test]
    fn if_first_is_empty_bottle_and_second_is_full_bottle_move_is_returned_as_from_1_to_0() {
        let mut w = WaterSorting::new();
        w.init_empty_bottle();
        w.init_bottle_with_four_colors(Color::Blue, Color::Blue, Color::Blue, Color::Blue);

        let next_move = w.next_available_move();
        assert!(next_move.is_some());
        assert_eq!(next_move.unwrap(), Pour::new(1usize, 0usize))
    }

    #[test]
    fn if_there_are_no_empty_bottles_but_there_is_room_to_pour_move_is_returned() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_one_color(Color::Red);
        w.init_bottle_with_two_colors(Color::Red, Color::Blue);

        let next_move = w.next_available_move();
        assert_eq!(next_move.unwrap(), Pour::new(0usize, 2usize))
    }

    #[test]
    fn next_available_is_not_possible_if_destination_is_full() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_three_colors(Color::Green, Color::Green, Color::Green);
        w.init_bottle_with_four_colors(Color::Red, Color::Red, Color::Red, Color::Green);
        w.init_bottle_with_one_color(Color::Red);

        let next_move = w.next_available_move();
        assert_eq!(next_move.unwrap(), Pour::new(1usize, 0usize))
    }

    #[test]
    fn solves_with_loops_during_the_solution() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Yellow, Color::Magenta, Color::Brown, Color::Yellow);
        w.init_bottle_with_four_colors(Color::Magenta, Color::Magenta, Color::Brown, Color::Yellow);
        w.init_bottle_with_four_colors(Color::Brown, Color::Brown, Color::Yellow, Color::Magenta);
        w.init_empty_bottle();
        w.init_empty_bottle();

        let ref_w = &w;

        let solver = WaterSolver::new(ref_w);

        let result = solver.solve();

        assert!(!result.is_empty());

        for p in result.iter() {
            w.pour(p.from as u8, p.to as u8);
        }

        assert!(w.win());
    }
}

#[cfg(test)]
mod water_sorting_tests {
    use crate::{Bottle, WaterSolver};
    use crate::Color;
    use crate::WaterSorting;

    #[test]
    fn reset_removes_all_the_elements() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);

        w.reset();

        assert_eq!(w.bottles.len(), 0);
    }

    #[test]
    fn pour_works_on_one_level() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_two_colors(Color::Orange, Color::Blue);

        w.pour(1, 0);

        assert_eq!(w.bottles[0].bottom, Some(Color::Blue));
        assert!(w.bottles[1].l1.is_none());
        assert_eq!(w.bottles[1].bottom, Some(Color::Orange));
    }

    #[test]
    fn pour_works_on_empty_bottle() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_empty_bottle();

        w.pour(1, 0);
        assert!(w.bottles[1].bottom.is_none());
        assert_eq!(w.bottles[0].bottom, Some(Color::Blue));
    }

    #[test]
    fn pour_works_on_multiple_levels() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_two_colors(Color::Blue, Color::Blue);

        w.pour(1, 0);

        assert_eq!(w.bottles[0].bottom, Some(Color::Blue));
        assert_eq!(w.bottles[0].l1, Some(Color::Blue));
        assert_eq!(w.bottles[0].l2, Some(Color::Blue));
        assert!(w.bottles[1].l1.is_none());
        assert!(w.bottles[1].bottom.is_none());
    }

    #[test]
    fn pour_works_on_multiple_levels_with_different_bottom_one() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_three_colors(Color::Orange,Color::Blue,Color::Blue);

        w.pour(1, 0);

        assert_eq!(w.bottles[0].bottom, Some(Color::Blue));
        assert_eq!(w.bottles[0].l1, Some(Color::Blue));
        assert_eq!(w.bottles[0].l2, Some(Color::Blue));
        assert!(w.bottles[1].l1.is_none());
        assert_eq!(w.bottles[1].bottom, Some(Color::Orange));
    }

    #[test]
    fn undo_restore_previous_level() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_two_colors(Color::Orange, Color::Blue);

        w.pour(1, 0);

        w.undo();

        assert_eq!(w.bottles[0].bottom, Some(Color::Blue));
        assert!(w.bottles[0].l1.is_none());
        assert_eq!(w.bottles[1].bottom, Some(Color::Orange));
        assert_eq!(w.bottles[1].l1, Some(Color::Blue));
    }

    #[test]
    fn undo_restore_only_the_last_state() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_two_colors(Color::Orange, Color::Blue);
        w.init_empty_bottle();

        w.pour(1, 0);
        w.pour(1,2);

        w.undo();

        assert_eq!(w.bottles[0].bottom, Some(Color::Blue));
        assert_eq!(w.bottles[0].l1, Some(Color::Blue));
        assert_eq!(w.bottles[1].bottom, Some(Color::Orange));
        assert!(w.bottles[2].bottom.is_none());
    }

    #[test]
    fn bottle_is_sorted_if_only_one_color_on_one_level() {
        let b = Bottle::with_one_color(0, Color::Blue);
        assert!(b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_sorted_if_only_one_color_is_one_two_levels() {
        let b = Bottle::with_two_colors(0, Color::Blue, Color::Blue);
        assert!(b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_bottom_levels() {
        let b = Bottle::with_two_colors(0, Color::Blue, Color::Orange);
        assert!(!b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_middle_levels() {
        let b = Bottle::with_three_colors(
            0,
            Color::Blue,
            Color::Orange,
            Color::Blue
        );
        assert!(!b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_top_levels() {
        let b = Bottle::new(
            None,
            Some(Color::Blue),
            Some(Color::Blue),
            Some(Color::Orange),
            Some(Color::Blue),
        );
        assert!(!b.is_empty_or_one_color())
    }

    #[test]
    fn game_is_won_if_all_bottles_are_sorted() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Blue);
        w.init_bottle_with_three_colors(Color::Blue,Color::Blue,Color::Blue);

        w.pour(0, 1);

        assert!(w.win())
    }

    #[test]
    fn game_is_won_if_all_bottles_are_sorted_in_full() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_one_color(Color::Red);
        w.init_bottle_with_one_color(Color::Magenta);
        w.init_bottle_with_three_colors(Color::Red, Color::Red, Color::Red);
        w.init_bottle_with_three_colors(Color::Magenta, Color::Magenta, Color::Magenta);

        assert!(!w.win())
    }

    #[test]
    fn if_there_is_an_empty_bottle_move_is_available() {
        let mut w = WaterSorting::new();
        w.init_empty_bottle();

        assert!(w.move_available())
    }

    #[test]
    fn if_there_are_all_full_bottles_move_is_not_available() {
        let mut w = WaterSorting::new();

        w.init_bottle_with_three_colors(Color::Blue, Color::Blue, Color::Blue);
        w.init_bottle_with_four_colors(Color::Green, Color::Green, Color::Green, Color::Green);

        assert!(!w.move_available())
    }

    #[test]
    fn if_top_colors_do_not_match_if_there_is_room_move_is_not_available() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_two_colors(Color::Blue, Color::Magenta);
        w.init_bottle_with_two_colors(Color::Magenta, Color::Blue);

        assert!(!w.move_available())
    }

    #[test]
    fn if_top_colors_do_not_match_if_there_is_no_room_move_is_not_available() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Blue, Color::Magenta, Color::Red, Color::Green);
        w.init_bottle_with_four_colors(Color::Magenta, Color::Blue, Color::Red, Color::Green);

        assert!(!w.move_available())
    }

    #[test]
    fn if_top_colors_do_match_if_there_is_room_move_is_available() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_two_colors(Color::Blue, Color::Magenta);
        w.init_bottle_with_two_colors(Color::Blue, Color::Magenta);

        assert!(w.move_available())
    }

    #[test]
    fn solve_real_game() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Green,Color::Red, Color::Green, Color::Red);
        w.init_bottle_with_four_colors(Color::Red,Color::Green,Color::Red,Color::Green);
        w.init_empty_bottle();

        w.pour(1, 2);
        w.pour(0, 1);
        w.pour(0, 2);
        w.pour(1, 0);
        w.pour(1, 2);
        w.pour(0, 1);
        w.pour(2, 0);

        assert!(w.win());
    }

    #[test]
    fn pour_as_much_as_possible() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_three_colors(Color::Green, Color::Red, Color::Red);
        w.init_bottle_with_three_colors(Color::Green, Color::Red, Color::Red);

        w.pour(0, 1);

        let b1 = w.bottles.get(0).unwrap();
        let b2 = w.bottles.get(1).unwrap();

        assert_eq!(b1.bottom, Some(Color::Green));
        assert_eq!(b1.l1, Some(Color::Red));
        assert!(b1.l2.is_none());
        assert!(b1.top.is_none());
        assert_eq!(b2.bottom, Some(Color::Green));
        assert_eq!(b2.l1, Some(Color::Red));
        assert_eq!(b2.l2, Some(Color::Red));
        assert_eq!(b2.top, Some(Color::Red));
    }

    #[test]
    fn solve_automatically() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Green,Color::Green, Color::Green, Color::Red);
        w.init_bottle_with_four_colors(Color::Red,Color::Red, Color::Red,Color::Green);
        w.init_empty_bottle();

        let solver = WaterSolver::new(&w);

        let moves = solver.solve();

        for _move in moves.iter() {
            w.pour(_move.from as u8, _move.to as u8)
        }
        assert!(w.win());
    }
}
