#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

mod volume_data;
pub use volume_data::VolumeData;

mod mesh_builder;
pub use mesh_builder::IMeshBuilder;
pub use mesh_builder::IVolumeData;

mod mesh_data_simple;
pub use mesh_data_simple::MeshDataSimple;

mod marching_cubes;
pub use marching_cubes::MarchingCubesMeshBuilder as MarchingCubesMeshBuilder;

mod marching_tetrahedra;
pub use marching_tetrahedra::MarchingTetrahedraMeshBuilder as MarchingTetrahedraMeshBuilder;
