use std::collections::BTreeMap;

use super::{BrushHulls, BrushId};
use crate::{
    face::{FaceId, FaceVertices},
};

#[derive(Debug, Clone)]
pub struct BrushFaceContainment(BTreeMap<BrushId, Vec<FaceId>>);

impl BrushFaceContainment {
    // Find contained faces
    pub fn new(
        brushes: &Vec<BrushId>,
        faces: &Vec<FaceId>,
        brush_faces: &BTreeMap<BrushId, Vec<FaceId>>,
        brush_hulls: &BrushHulls,
        face_vertices: &FaceVertices,
    ) -> Self {
        let mut contained_faces = BTreeMap::<BrushId, Vec<FaceId>>::default();
        for brush_id in brushes {
            let brush_faces = &brush_faces[brush_id];
            let brush_hull = &brush_hulls[brush_id];

            for face_id in faces {
                // Skip checking own vertices
                if brush_faces.contains(face_id) {
                    continue;
                }

                let face_verts = face_vertices.vertices(&face_id).unwrap();

                let mut contained = true;
                for vertex in face_verts {
                    if !brush_hull.contains(*vertex) {
                        contained = false;
                        break;
                    }
                }

                if !contained {
                    continue;
                }

                contained_faces.entry(*brush_id).or_default().push(*face_id);
            }
        }
        BrushFaceContainment(contained_faces)
    }

    pub fn get_contained_faces(&self, brush_id: &BrushId) -> Option<&Vec<FaceId>> {
        self.0.get(brush_id)
    }

    pub fn is_contained(&self, face_id: &FaceId) -> bool {
        self.0.values().flatten().any(|id| id == face_id)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&BrushId, &Vec<FaceId>)> {
        self.0.iter()
    }
}
