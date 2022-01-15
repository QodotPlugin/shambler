use std::collections::BTreeMap;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use usage::Usage;

use crate::{
    brush::{BrushHulls, BrushId},
    face::FaceId,
    ConvexHull, FacePlanes, Plane3d, Vector3, EPSILON,
};

pub enum FaceVerticesTag {}
pub enum FaceVertexPlanesTag {}

pub type FaceVertices = Usage<FaceVerticesTag, BTreeMap<FaceId, Vec<Vector3>>>;
pub type FaceVertexPlanes =
    Usage<FaceVertexPlanesTag, BTreeMap<FaceId, Vec<(FaceId, FaceId, FaceId)>>>;

pub fn face_vertices(
    brush_planes: &BTreeMap<BrushId, Vec<FaceId>>,
    face_planes: &FacePlanes,
    brush_hulls: &BrushHulls,
) -> (FaceVertices, FaceVertexPlanes) {
    brush_planes
        .par_iter()
        .flat_map(|(brush_id, face_ids)| {
            let hull = &brush_hulls[brush_id];

            let plane_iter = face_planes.par_iter().filter_map(move |(k, v)| {
                if face_ids.contains(k) {
                    Some((*k, *v))
                } else {
                    None
                }
            });

            face_ids.par_iter().map(move |face_id| {
                let plane = &face_planes[face_id];
                let mut verts = intersect_planes(*face_id, *plane, plane_iter.clone(), hull);
                verts.dedup_by(|(_, lhs), (_, rhs)| lhs == rhs);
                let (vert_planes, verts) = verts.into_iter().unzip();
                ((*face_id, verts), (*face_id, vert_planes))
            })
        })
        .unzip()
}

// Create world vertices via triplanar intersection
pub fn intersect_planes<'a, I: ParallelIterator<Item = (FaceId, Plane3d)> + Clone + Sync>(
    p0_id: FaceId,
    p0: Plane3d,
    planes: I,
    hull: &ConvexHull,
) -> Vec<((FaceId, FaceId, FaceId), Vector3)> {
    planes
        .clone()
        .flat_map(move |(p1_id, p1)| {
            planes.clone().map(move |(p2_id, p2)| {
                if let Some(position) = triplanar_intersection(&p0, &p1, &p2) {
                    if hull.contains(&position) {
                        return Some(((p0_id, p1_id, p2_id), position));
                    }
                }

                None
            })
        })
        .flatten()
        .collect()
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
