use std::fmt;

#[derive(Debug)]
pub(super) enum ScrollDirection {
    None,
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for ScrollDirection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ScrollDirection::None => write!(f, "none"),
            ScrollDirection::Up => write!(f, "up"),
            ScrollDirection::Down => write!(f, "down"),
            ScrollDirection::Left => write!(f, "left"),
            ScrollDirection::Right => write!(f, "right"),
        }
    }
}
