use std::collections::BTreeMap;

use crate::{
    brush::{BrushHulls, BrushId},
    face::FaceId,
    ConvexHull, FacePlanes, Plane3d, Vector3, EPSILON,
};

/// Per-face vertex lists
#[derive(Debug, Clone)]
pub struct FaceVertices {
    vertices: BTreeMap<FaceId, Vec<Vector3>>,
    vertex_planes: BTreeMap<FaceId, Vec<(FaceId, FaceId, FaceId)>>,
}

impl FaceVertices {
    /// Calculate face vertices via triplanar intersection
    pub fn new(
        brush_planes: &BTreeMap<BrushId, Vec<FaceId>>,
        face_planes: &FacePlanes,
        brush_hulls: &BrushHulls,
    ) -> Self {
        let mut vertices = BTreeMap::<FaceId, Vec<Vector3>>::default();
        let mut vertex_planes = BTreeMap::<FaceId, Vec<(FaceId, FaceId, FaceId)>>::default();
        for (brush_id, plane_ids) in brush_planes {
            let hull = &brush_hulls[brush_id];

            let plane_iter = face_planes.iter().filter_map(|(k, v)| {
                if plane_ids.contains(k) {
                    Some((*k, *v))
                } else {
                    None
                }
            });

            for plane_id in plane_ids.iter() {
                let plane = &face_planes[plane_id];
                let mut verts = intersect_planes(*plane_id, *plane, plane_iter.clone(), hull);
                verts.dedup_by(|(_, lhs), (_, rhs)| lhs == rhs);
                let (vert_planes, verts) = verts.into_iter().unzip();
                vertices.insert(*plane_id, verts);
                vertex_planes.insert(*plane_id, vert_planes);
            }
        }
        FaceVertices {
            vertices,
            vertex_planes,
        }
    }

    pub fn len(&self) -> usize {
        let len = self.vertices.len();
        assert!(len == self.vertex_planes.len());
        len
    }

    /// Get the vertices corresponding to the provided face ID, if they exist
    pub fn vertices(&self, face_id: &FaceId) -> Option<&Vec<Vector3>> {
        self.vertices.get(face_id)
    }

    pub fn vertex_planes(&self, face_id: &FaceId) -> Option<&Vec<(FaceId, FaceId, FaceId)>> {
        self.vertex_planes.get(face_id)
    }

    pub fn iter_vertices(&self) -> impl Iterator<Item = (&FaceId, &Vec<Vector3>)> {
        self.vertices.iter()
    }

    pub fn iter_vertex_planes(
        &self,
    ) -> impl Iterator<Item = (&FaceId, &Vec<(FaceId, FaceId, FaceId)>)> {
        self.vertex_planes.iter()
    }
}

// Create world vertices via triplanar intersection
pub fn intersect_planes<'a, I: Iterator<Item = (FaceId, Plane3d)> + Clone>(
    p0_id: FaceId,
    p0: Plane3d,
    planes: I,
    hull: &ConvexHull,
) -> Vec<((FaceId, FaceId, FaceId), Vector3)> {
    let mut plane_world_vertices: Vec<((FaceId, FaceId, FaceId), Vector3)> = vec![];

    for (p1_id, p1) in planes.clone() {
        for (p2_id, p2) in planes.clone() {
            if let Some(position) = triplanar_intersection(&p0, &p1, &p2) {
                if hull.contains(position) {
                    plane_world_vertices.push(((p0_id, p1_id, p2_id), position));
                }
            }
        }
    }
    plane_world_vertices
}

pub fn triplanar_intersection(p0: &Plane3d, p1: &Plane3d, p2: &Plane3d) -> Option<Vector3> {
    let n0 = p0.normal();
    let n1 = p1.normal();
    let n2 = p2.normal();

    let denom = n0.cross(n1).dot(n2);

    if denom < EPSILON {
        return None;
    }

    Some(
        (n1.cross(n2) * p0.distance()
            + n2.cross(n0) * p1.distance()
            + n0.cross(n1) * p2.distance())
            / denom,
    )
}
