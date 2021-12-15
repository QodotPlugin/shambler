use std::collections::BTreeMap;

use super::EntityId;
use crate::{Vector3, brush::{BrushCenters, BrushId}};

#[derive(Debug, Clone)]
pub struct EntityCenters(BTreeMap<EntityId, Vector3>);

impl EntityCenters {
    // Calculate entity centers
    pub fn new(
        entity_brushes: &BTreeMap<EntityId, Vec<BrushId>>,
        brush_centers: &BrushCenters,
    ) -> Self {
        let mut entity_centers = BTreeMap::<EntityId, Vector3>::default();
        for (entity_id, brush_ids) in entity_brushes {
            let mut center = Vector3::zeros();

            for plane_id in brush_ids {
                center += brush_centers[plane_id];
            }
            center /= brush_ids.len() as f32;

            entity_centers.insert(*entity_id, center);
        }
        EntityCenters(entity_centers)
    }

    pub fn get(&self, entity_id: &EntityId) -> Option<&Vector3> {
        self.0.get(entity_id)
    }
}

impl std::ops::Index<&EntityId> for EntityCenters {
    type Output = Vector3;

    fn index(&self, index: &EntityId) -> &Self::Output {
        &self.0[index]
    }
}
