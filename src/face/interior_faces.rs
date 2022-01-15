use std::collections::BTreeSet;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use usage::Usage;

use super::{FaceDuplicates, FaceId};
use crate::{face::FaceLines, line::LineDuplicates, BrushFaces, Brushes};

pub enum InteriorFacesTag {}

pub type InteriorFaces = Usage<InteriorFacesTag, BTreeSet<FaceId>>;

pub fn interior_faces(
    brushes: &Brushes,
    brush_faces: &BrushFaces,
    face_duplicates: &FaceDuplicates,
    face_lines: &FaceLines,
    line_duplicates: &LineDuplicates,
) -> InteriorFaces {
    // Iterate over the entity's brushes
    brushes
        .par_iter()
        .flat_map(|brush_a| {
            let faces_a = &brush_faces[brush_a];

            // Iterate over LHS brush faces
            faces_a
                .par_iter()
                .filter(|f| !face_duplicates.par_iter().any(|(a, _)| a == *f))
                .map(|face_a| {
                    // Fetch LHS vertex and line data
                    let lines_a = face_lines.get(&face_a).unwrap();

                    // Calculate whether this is an interior face
                    let connected_lines: usize = lines_a
                        .par_iter()
                        .map(|line_id_a| {
                            if line_duplicates.par_iter().any(|(a, _)| a == line_id_a) {
                                1
                            } else {
                                0
                            }
                        })
                        .sum();

                    // If all of this face's lines are connected, add it to the interior set
                    if connected_lines >= lines_a.len() {
                        Some(*face_a)
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect()
}
