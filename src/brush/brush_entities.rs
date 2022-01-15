use std::collections::BTreeMap;

use crate::{brush::BrushId, entity::EntityId, EntityBrushes};

#[derive(Debug, Default, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BrushEntities(pub BTreeMap<BrushId, EntityId>);

impl BrushEntities {
    pub fn new(entity_brushes: &EntityBrushes) -> Self {
        BrushEntities(
            entity_brushes
                .iter()
                .flat_map(|(entity, brushes)| brushes.iter().map(move |brush| (*brush, *entity)))
                .collect(),
        )
    }
}
