// use anyhow::Result;

#[derive(Debug, PartialEq)]
pub enum BlockType {
    Solid,
    Transparent,
    Unbreakable,
}

#[derive(Debug, PartialEq)]
pub enum BlockProperty {
    WaterLockable,
}

#[derive(Debug, Default)]
pub struct Block {
    position: [f32; 3],
    transparent: bool,
    unbreakable: bool,
    // block_type: Vec<BlockType>,
    _properties: Option<Vec<BlockProperty>>,
}

// impl Block {
//     pub fn new() -> Self {
//         todo!()
//     }
// }

// impl Sized for Block {}

pub trait BlockTrait {
    fn new(
        position: [f32; 3],
        transparent: bool,
        unbreakable: bool,
        properties: Option<Vec<BlockProperty>>,
    ) -> Self;
    fn pos(&self) -> &[f32; 3];
    fn is_breakable(&self) -> bool;
    fn is_transparent(&self) -> bool;
    fn has_property<'a>(&self, property: BlockProperty) -> &'a bool;
}

impl BlockTrait for Block {
    fn new(
        position: [f32; 3],
        transparent: bool,
        unbreakable: bool,
        properties: Option<Vec<BlockProperty>>,
    ) -> Self {
        Self {
            position,
            transparent,
            unbreakable,
            _properties: properties,
        }
    }

    fn pos(&self) -> &[f32; 3] {
        &self.position
    }

    fn is_breakable(&self) -> bool {
        self.unbreakable
    }

    fn is_transparent(&self) -> bool {
        self.transparent
    }

    fn has_property<'a>(&self, _property: BlockProperty) -> &'a bool {
        // &self.properties.unwrap().contains(&property)
        todo!()
    }
}
