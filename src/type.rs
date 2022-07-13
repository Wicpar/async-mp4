use uuid::Uuid;
use std::mem;
use std::fmt::{Display, Formatter};
use crate::id::BoxId;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum BoxType {
    Id(BoxId),
    UUID(Uuid)
}

impl Display for BoxType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BoxType::Id(id) => Display::fmt(id, f),
            BoxType::UUID(uuid) => Display::fmt(uuid, f)
        }
    }
}

impl PartialEq<BoxId> for BoxType {
    fn eq(&self, other: &BoxId) -> bool {
        match self {
            BoxType::Id(id) => id == other,
            BoxType::UUID(_) => false
        }
    }
}

impl PartialEq<Uuid> for BoxType {
    fn eq(&self, other: &Uuid) -> bool {
        match self {
            BoxType::Id(_) => false,
            BoxType::UUID(id) => id == other
        }
    }
}

impl PartialEq<&[u8;4]> for BoxType {
    fn eq(&self, other: &&[u8;4]) -> bool {
        match self {
            BoxType::Id(id) => id == other,
            BoxType::UUID(_) => false
        }
    }
}

impl From<BoxId> for BoxType {
    fn from(id: BoxId) -> Self {
        Self::Id(id)
    }
}

impl From<[u8;4]> for BoxType {
    fn from(id: [u8; 4]) -> Self {
        Self::Id(id.into())
    }
}

impl From<&[u8;4]> for BoxType {
    fn from(id: &[u8; 4]) -> Self {
        Self::Id((*id).into())
    }
}

impl From<Uuid> for BoxType {
    fn from(id: Uuid) -> Self {
        Self::UUID(id)
    }
}

impl PartialEq<uuid::Bytes> for BoxType {
    fn eq(&self, other: &uuid::Bytes) -> bool {
        match self {
            BoxType::Id(_) => false,
            BoxType::UUID(id) => id.as_bytes() == other
        }
    }
}

impl BoxType {
    pub fn byte_size(&self) -> usize {
        match self {
            BoxType::Id(_) => BoxId::size(),
            BoxType::UUID(_) => BoxId::size() + mem::size_of::<uuid::Bytes>()
        }
    }
}
