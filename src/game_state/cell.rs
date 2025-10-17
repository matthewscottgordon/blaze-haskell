use crate::api_types;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cell(pub i8, pub i8);

impl From<&api_types::Coordinates> for Cell {
    fn from(value: &api_types::Coordinates) -> Self {
        Self(value.x as i8, value.y as i8)
    }
}
