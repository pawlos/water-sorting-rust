use core::fmt::Debug;
use std::fmt::{Display, Formatter};
use wasm_bindgen::prelude::wasm_bindgen;

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
            Self::Empty => write!(f, ""),
        }
    }
}

#[wasm_bindgen]
#[derive(PartialEq)]
struct Bottle {
    bottom: Option<Color>,
    l1: Option<Color>,
    l2: Option<Color>,
    top: Option<Color>,
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
        bottom: Option<Color>,
        l1: Option<Color>,
        l2: Option<Color>,
        top: Option<Color>,
    ) -> Self {
        Bottle {
            bottom,
            l1,
            l2,
            top,
        }
    }

    pub fn empty() -> Self {
        Bottle { bottom: None, l1: None, l2: None, top: None }
    }

    pub fn with_one_color(c: Color) -> Self {
        Bottle { bottom: Some(c), l1: None, l2: None, top: None }
    }

    pub fn with_two_colors(b: Color, l1: Color) -> Self {
        Bottle { bottom: Some(b), l1: Some(l1), l2: None, top: None}
    }

    pub fn with_three_colors(b: Color, l1: Color, l2: Color) -> Self {
        Bottle { bottom: Some(b), l1: Some(l1), l2: Some(l2), top: None}
    }

    pub fn with_four_colors(b: Color, l1: Color, l2: Color, t: Color) -> Self {
        Bottle { bottom: Some(b), l1: Some(l1), l2: Some(l2), top: Some(t) }
    }

    pub fn is_empty(&self) -> bool {
        matches!(
            (self.bottom, self.l1, self.l2, self.top),
            (None, None, None, None)
        )
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
struct WaterSorting {
    bottles: Vec<Bottle>,
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
        self.bottles.iter().for_each(|x|{
            f.write_fmt(format_args!("{:?}\n", x)).expect("");
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
        }
    }

    pub fn pour(&mut self, from_index: u8, to_index: u8) {
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

    pub fn win(self) -> bool {
        self.bottles.into_iter().all(|b| b.is_empty_or_one_color())
    }

    pub fn init_empty_bottle(&mut self) {
        self.bottles.push(Bottle::empty())
    }

    pub fn init_bottle_with_one_color(&mut self, c: Color) {
        self.bottles.push(Bottle::with_one_color(c))
    }

    pub fn init_bottle_with_two_colors(&mut self, b: Color, l1: Color) {
        self.bottles.push(Bottle::with_two_colors(b, l1))
    }

    pub fn init_bottle_with_three_colors(&mut self, b: Color, l1: Color, l2: Color) {
        self.bottles.push(Bottle::with_three_colors(b, l1, l2))
    }

    pub fn init_bottle_with_four_colors(&mut self, b: Color, l1: Color, l2: Color, t: Color) {
        self.bottles.push(Bottle::with_four_colors(b, l1, l2, t))
    }

    pub fn move_available(self) -> bool {
        if self.bottles.iter().any(|b| b.is_empty()) {
            return true;
        }
        if self.bottles.iter().all(|b| !b.is_empty()) {
            return false;
        }
        true
    }

    pub fn bottles(&self) -> *const Bottle {
        self.bottles.as_ptr()
    }

    pub fn bottle_index(self, b: &Bottle) -> Option<usize> {
        self.bottles.iter().position(|pb| pb == b)
    }

    pub fn solve(&mut self) -> bool {
        for _b in self.bottles.as_slice() {}
        true
    }

    pub fn render(&self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::Bottle;
    use crate::Color;
    use crate::WaterSorting;

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
    fn bottle_is_sorted_if_only_one_color_on_one_level() {
        let b = Bottle::with_one_color(Color::Blue);
        assert!(b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_sorted_if_only_one_color_is_one_two_levels() {
        let b = Bottle::new(Some(Color::Blue), Some(Color::Blue), None, None);
        assert!(b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_bottom_levels() {
        let b = Bottle::new(Some(Color::Blue), Some(Color::Orange), None, None);
        assert!(!b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_middle_levels() {
        let b = Bottle::new(
            Some(Color::Blue),
            Some(Color::Orange),
            Some(Color::Blue),
            None,
        );
        assert!(!b.is_empty_or_one_color())
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_top_levels() {
        let b = Bottle::new(
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
        w.init_bottle_with_three_colors(Color::Orange,Color::Blue,Color::Blue);

        w.pour(1, 0);

        assert!(w.win())
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
    #[ignore = "Not yet finished"]
    fn solve_automatically() {
        let mut w = WaterSorting::new();
        w.init_bottle_with_four_colors(Color::Green,Color::Red, Color::Green, Color::Red);
        w.init_bottle_with_four_colors(Color::Red,Color::Green, Color::Red,Color::Green);
        w.init_empty_bottle();

        w.solve();

        assert!(w.win());
    }
}
