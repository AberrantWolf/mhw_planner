#[derive(Debug)]
pub enum ItemType {
    Armor,
    Weapon,
}

impl Default for ItemType {
    fn default() -> Self {
        ItemType::Armor
    }
}

pub trait Gui {
    fn layout(&mut self);
}
