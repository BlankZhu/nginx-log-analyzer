use super::{
    enum_item::StrItem,
    noop_item::NoopItem,
    num_item::{F64Item, IsizeItem},
    Item,
};

pub enum ItemType {
    F64ItemType,
    IsizeItemType,
    NoopItemType,
    StrItemType,
}

pub struct ItemFactory {
}

impl ItemFactory {
    pub fn create_item(typ: ItemType, title: &str) -> Box<dyn Item> {
        match typ {
            ItemType::F64ItemType => Box::new(F64Item::new(title)),
            ItemType::IsizeItemType => Box::new(IsizeItem::new(title)),
            ItemType::NoopItemType => Box::new(NoopItem::new(title)),
            ItemType::StrItemType => Box::new(StrItem::new(title)),
            _ => Box::new(NoopItem::new(title)),
        }
    }
}