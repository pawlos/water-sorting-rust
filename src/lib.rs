use core::fmt::Debug;

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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

struct Bottle {
    bottom: Option<Color>,
    l1: Option<Color>,
    l2: Option<Color>,
    top: Option<Color>,
}

impl Debug for Bottle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl Bottle {
    pub fn new(
        bottom: Option<Color>,
        l1: Option<Color>,
        l2: Option<Color>,
        top: Option<Color>,
    ) -> Self {
        Bottle {
            bottom: bottom,
            l1: l1,
            l2: l2,
            top: top,
        }
    }

    pub fn top_color(self: &Self) -> Option<Color> {
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
        let w = match (self.top, self.l2, self.l1, self.bottom) {
            (None, None, None, None) => {
                self.bottom = Option::Some(c);
                true
            }
            (None, None, None, Some(s)) if s == c => {
                self.l1 = Option::Some(c);
                true
            }
            (None, None, Some(s), Some(_)) if s == c => {
                self.l2 = Option::Some(c);
                true
            }
            (None, Some(s), Some(_), Some(_)) if s == c => {
                self.top = Option::Some(c);
                true
            }
            _ => false,
        };

        w
    }

    pub fn empty_or_one_color(self) -> bool {
        match (self.bottom, self.l1, self.l2, self.top) {
            (None, None, None, None) => true,
            (Some(t), Some(l2), Some(l1), Some(bottom)) if t == l2 && l2 == l1 && l1 == bottom => {
                true
            }
            (Some(t), Some(l2), Some(l1), None) if t == l2 && l2 == l1 => true,
            (Some(t), Some(l2), None, None) if t == l2 => true,
            (Some(_), None, None, None) => true,
            _ => false,
        }
    }
}

struct WaterSorting {
    bottles: Vec<Bottle>,
}

impl Debug for WaterSorting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WaterSorting")
            .field("bottles", &self.bottles)
            .finish()
    }
}

impl WaterSorting {
    pub fn new(count: usize) -> Self {
        WaterSorting {
            bottles: Vec::with_capacity(count),
        }
    }

    pub fn pour(&mut self, from: u8, to: u8) {
        loop {
            let from_b = self.bottles[from as usize].top_color();
            if from_b == None {
                break;
            }
            let to_b = &mut self.bottles[to as usize];
            let r = to_b.pour(from_b.unwrap());
            if !r {
                break;
            }
            self.bottles[from as usize].pop();
        }
    }

    pub fn win(self) -> bool {
        self.bottles.into_iter().all(|b| b.empty_or_one_color())
    }

    pub fn init_bottle(&mut self, c: Option<Color>, c1: Option<Color>, c2: Option<Color>) {
        self.bottles.push(Bottle::new(c, c1, c2, None));
    }
}

#[cfg(test)]
mod tests {
    use crate::Bottle;
    use crate::Color;
    use crate::WaterSorting;

    #[test]
    fn pour_works_on_one_level() {
        let mut w = WaterSorting::new(2);
        w.init_bottle(Some(Color::Blue), None, None);
        w.init_bottle(Some(Color::Orange), Some(Color::Blue), None);

        w.pour(1, 0);

        assert!(w.bottles[0].bottom.unwrap() == Color::Blue);
        assert!(w.bottles[1].l1 == None);
        assert!(w.bottles[1].bottom.unwrap() == Color::Orange);
    }

    #[test]
    fn pour_works_on_multiple_levels() {
        let mut w = WaterSorting::new(2);
        w.init_bottle(Some(Color::Blue), None, None);
        w.init_bottle(Some(Color::Blue), Some(Color::Blue), None);

        w.pour(1, 0);

        assert!(w.bottles[0].bottom.unwrap() == Color::Blue);
        assert!(w.bottles[0].l1.unwrap() == Color::Blue);
        assert!(w.bottles[0].l2.unwrap() == Color::Blue);
        assert!(w.bottles[1].l1 == None);
        assert!(w.bottles[1].bottom == None);
    }

    #[test]
    fn pour_works_on_multiple_levels_with_different_bottom_one() {
        let mut w = WaterSorting::new(2);
        w.init_bottle(Some(Color::Blue), None, None);
        w.init_bottle(Some(Color::Orange), Some(Color::Blue), Some(Color::Blue));

        w.pour(1, 0);

        assert!(w.bottles[0].bottom.unwrap() == Color::Blue);
        assert!(w.bottles[0].l1.unwrap() == Color::Blue);
        assert!(w.bottles[0].l2.unwrap() == Color::Blue);
        assert!(w.bottles[1].l1 == None);
        assert!(w.bottles[1].bottom.unwrap() == Color::Orange);
    }

    #[test]
    fn bottle_is_sorted_if_only_one_color_on_one_level() {
        let b = Bottle::new(Some(Color::Blue), None, None, None);
        assert!(b.empty_or_one_color() == true)
    }

    #[test]
    fn bottle_is_sorted_if_only_one_color_is_one_two_levels() {
        let b = Bottle::new(Some(Color::Blue), Some(Color::Blue), None, None);
        assert!(b.empty_or_one_color() == true)
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_bottom_levels() {
        let b = Bottle::new(Some(Color::Blue), Some(Color::Orange), None, None);
        assert!(b.empty_or_one_color() == false)
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_middle_levels() {
        let b = Bottle::new(
            Some(Color::Blue),
            Some(Color::Orange),
            Some(Color::Blue),
            None,
        );
        assert!(b.empty_or_one_color() == false)
    }

    #[test]
    fn bottle_is_not_sorted_if_different_colors_on_two_top_levels() {
        let b = Bottle::new(
            Some(Color::Blue),
            Some(Color::Blue),
            Some(Color::Orange),
            Some(Color::Blue),
        );
        assert!(b.empty_or_one_color() == false)
    }

    #[test]
    fn game_is_won_if_all_bottles_are_sorted() {
        let mut w = WaterSorting::new(2);
        w.init_bottle(Some(Color::Blue), None, None);
        w.init_bottle(Some(Color::Orange), Some(Color::Blue), Some(Color::Blue));

        w.pour(1, 0);

        assert!(w.win() == true)
    }
}
