use crate::components::{Controller, Player, weapon::WeaponManager};
use amethyst::core::{
    math::{Point2, Unit},
    Float, Transform,
};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Axis {
    Horizontal,
    Vertical,
}

#[derive(Debug, Hash, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Action {
    Fire,
}

impl fmt::Display for Axis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self))
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", *self))
    }
}

pub struct GameBindings;
impl BindingTypes for GameBindings {
    type Axis = Axis;
    type Action = Action;
}

#[derive(Default, Debug)]
pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (
        WriteStorage<'a, Player>,
        WriteStorage<'a, Controller>,
        WriteStorage<'a, WeaponManager>,
        Read<'a, InputHandler<GameBindings>>,
    );


    fn run(&mut self, (mut players, mut controllers, mut managers, input): Self::SystemData) {
        // Loop through all players and assign direction
        for (controller, player) in (&mut controllers, &mut players).join() {
            controller.rotation_control = Float::from(input.axis_value(&Axis::Horizontal).unwrap());
            controller.thrust_control = Float::from(input.axis_value(&Axis::Vertical).unwrap());
        }

        // loop through all weapons systems and assign firing states
        for (_, manager) in (&mut players, &mut managers).join() {
            manager.wants_to_fire = input.action_is_down(&Action::Fire).expect("Error reading action");
        }
    }
}
