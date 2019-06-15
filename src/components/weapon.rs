use amethyst::{
    assets::PrefabData,
    core::{math::Vector3, Float},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    error::Error,
};
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone, Deserialize, Serialize, PrefabData, Debug, PartialEq)]
#[serde(default)]
#[prefab(Component)]
pub struct Weapon {
    name: String,
    /// Speed the projectile will travel
    speed: Float,
    /// Time taken between consecutive shots
    pub recoil: Duration,
    /// Absolute time that weapon was last fired
    pub last_fired: Duration,
    /// Component that will be attached to this weapon
    pub projectile: Projectile,
}

impl Component for Weapon {
    type Storage = DenseVecStorage<Self>;
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            name: String::from("Weapon"),
            speed: Float::from(10.),
            recoil: Duration::from_millis(250),
            last_fired: Duration::from_secs(0),
            projectile: Projectile::default(),
        }
    }
}

impl Weapon {
    /// Useful for testing equality without needing to build an entire `Weapon`.
    fn set_name(mut self, name: String) -> Self {
        self.name = name;
        self
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Projectile {
    pub max_speed: Float,
    pub traction: Float,
}

impl Projectile {}

impl Default for Projectile {
    fn default() -> Self {
        Self {
            max_speed: Float::from(10.0),
            traction: Float::from(10.0),
        }
    }
}

impl Component for Projectile {
    type Storage = DenseVecStorage<Self>;
}

/// A weapon manager references weapons that are currently in the inventory. It switches active weapon and also
/// equips/unequips weapons.
#[derive(Clone, Deserialize, Serialize, PrefabData, Default)]
#[serde(default)]
#[prefab(Component)]
pub struct WeaponManager {
    weapons: Vec<Weapon>,
    active_index: usize,
    pub wants_to_fire: bool,
}

impl WeaponManager {
    pub fn weapons(&self) -> &Vec<Weapon> {
        &self.weapons
    }

    pub fn current_weapon(&self) -> Option<&Weapon> {
        if self.weapons.len() != 0 {
            Some(&self.weapons[self.active_index])
        } else {
            None
        }
    }

    pub fn current_weapon_mut(&mut self) -> Option<&mut Weapon> {
        if self.weapons.len() != 0 {
            Some(&mut self.weapons[self.active_index])
        } else {
            None
        }
    }

    // cycle through equipped weapons
    pub fn cycle(&mut self, direction: Direction) -> Option<&Weapon> {
        // Do nothing if no weapons are in our manager
        if self.weapons().is_empty() {
            return None;
        }
        // Cycle in the right direction
        match direction {
            // Make sure we are always in bounds
            Direction::Forward => {
                self.active_index += 1;
                let next = self.weapons.get(self.active_index);
                // Return weapon reference and
                if next.is_some() {
                    next
                } else {
                    self.active_index = 0;
                    self.weapons.get(0)
                }
            }
            Direction::Backward => {
                if self.active_index == 0 {
                    self.active_index = self.weapons().len() - 1;
                } else {
                    self.active_index -= 1;
                }
                self.weapons.get(self.active_index)
            }
        }
    }

    fn index(&self) -> usize {
        self.active_index
    }

    /// If the user wants to select their weapon from an array of weapons, they can set the index manually
    pub fn set_index(&mut self, new_index: usize) {
        self.active_index = new_index;
    }
}

impl Component for WeaponManager {
    type Storage = DenseVecStorage<Self>;
}

pub enum Direction {
    Forward,
    Backward,
}

#[cfg(test)]
mod tests {
    use super::*;
    fn create_manager(index: usize) -> WeaponManager {
        WeaponManager {
            weapons: vec![
                Weapon::default().set_name("0".to_string()),
                Weapon::default().set_name("1".to_string()),
                Weapon::default().set_name("2".to_string()),
            ],
            active_index: index,
            wants_to_fire: false,
        }
    }

    #[test]
    fn test_increment() {
        let mut wm = create_manager(0);
        assert_eq!(
            wm.cycle(Direction::Forward).unwrap(),
            &Weapon::default().set_name("1".to_string())
        );
        assert_eq!(wm.index(), 1);
    }

    #[test]
    fn decrement() {
        let mut wm = create_manager(1);
        assert_eq!(
            wm.cycle(Direction::Backward).unwrap(),
            &Weapon::default().set_name("0".to_string())
        );
        assert_eq!(wm.index(), 0);
    }

    #[test]
    fn decrement_wrap() {
        let mut wm = create_manager(0);
        assert_eq!(
            wm.cycle(Direction::Backward).unwrap(),
            &Weapon::default().set_name("2".to_string())
        );
        assert_eq!(wm.index(), 2);
    }

    #[test]
    fn increment_wrap() {
        let mut wm = create_manager(2);
        assert_eq!(
            wm.cycle(Direction::Forward).unwrap(),
            &Weapon::default().set_name("0".to_string())
        );
        assert_eq!(wm.index(), 0);
    }
}
