#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
// Direction is not an enum because an entity can move in 2 axes at once
pub struct Direction {
    pub vertical: Option<Vertical>,
    pub horizontal: Option<Horizontal>,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Vertical {
    Up,
    Down,
}
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum Horizontal {
    Left,
    Right,
}

pub trait CanMove {
    fn get_move_direction(&self) -> Option<Direction>;
}

#[macro_export]
macro_rules! impl_can_move {
    ($ty:ty) => {
        impl $crate::common::movement::CanMove for $ty {
            fn get_move_direction(&self) -> Option<Direction> {
                if let Self::Moving(direction) = *self {
                    Some(direction)
                } else {
                    None
                }
            }
        }
    };
}
