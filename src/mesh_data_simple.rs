use num_traits::{Float, Unsigned};

use crate::mesh_builder::IMeshData;

pub struct MeshDataSimple<F: Float, U: Unsigned> {
    _vertex_data: Vec<F>,
    _index_data: Vec<U>,
}

impl<F: Float, U: Unsigned> MeshDataSimple<F, U> {
    pub fn new() -> Self {
        Self {
            _vertex_data: Vec::new(),
            _index_data: Vec::new(),
        }
    }

    pub fn get_vertex_data(&self) -> &[F] {
        &self._vertex_data
    }

    pub fn get_index_data(&self) -> &[U] {
        &self._index_data
    }
}

impl<F: Float, U: Unsigned, const TRESOLUTION: usize> IMeshData<F, U, TRESOLUTION>
    for MeshDataSimple<F, U>
{
    fn push_vertex(&mut self, x: F, y: F, z: F) {
        self._vertex_data.push(x);
        self._vertex_data.push(y);
        self._vertex_data.push(z);
    }

    fn push_triangle(&mut self, x: U, y: U, z: U) {
        self._index_data.push(x);
        self._index_data.push(y);
        self._index_data.push(z);
    }
}
