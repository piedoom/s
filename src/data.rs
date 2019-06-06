//! Pure data, non-component shared definitions such as movement enumerations
use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self))
    }
}

pub struct GameBindings;
impl BindingTypes for GameBindings {
    type Axis = Axis;
    type Action = String;
}
