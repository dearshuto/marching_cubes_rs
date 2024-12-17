use num_traits::{Float, Unsigned};
use std::marker::Copy;

pub trait IVolumeData<T: Unsigned + Copy, const TRESOLUTION: usize> {
    fn get(&self, x: u32, y: u32, z: u32) -> T;

    fn set(&mut self, x: u32, y: u32, z: u32, value: T);
}

pub trait IMeshData<F, U>
where
    F: Float,
    U: Unsigned,
{
    fn push_vertex(&mut self, x: F, y: F, z: F);

    fn push_triangle(&mut self, x: U, y: U, z: U);
}

pub trait IMeshBuilder<F, U, TMeshData, TDataType, const TRESOLUTION: usize, TVolumeData>
where
    F: Float,
    U: Unsigned,
    TMeshData: IMeshData<F, U>,
    TDataType: Unsigned + Copy,
    TVolumeData: IVolumeData<TDataType, TRESOLUTION>,
{
    fn build(&self, mesh_data: &mut TMeshData, volume_data: &TVolumeData);

    fn with_threshold(self, threshold: f32) -> Self;
}
