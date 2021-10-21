use crate::{
    mesh_builder::{IMeshBuilder, IMeshData},
    mesh_pattern, IVolumeData,
};
use num_traits::{Float, ToPrimitive, Unsigned};
use std::marker::Copy;

pub struct MarchingCubesMeshBuilder<
    F,
    U,
    TMeshData,
    TDataType,
    const TRESOLUTION: usize,
    TVolumeData,
> where
    F: Float,
    U: Unsigned,
    TMeshData: IMeshData<F, U, TRESOLUTION>,
    TDataType: Unsigned + Copy,
    TVolumeData: IVolumeData<TDataType, TRESOLUTION>,
{
    _threshold: f32,
    _marker: std::marker::PhantomData<(F, U, TMeshData, TDataType, TVolumeData)>,
}

impl<F, U, TMeshData, TDataType, const TRESOLUTION: usize, TVolumeData>
    MarchingCubesMeshBuilder<F, U, TMeshData, TDataType, TRESOLUTION, TVolumeData>
where
    F: Float,
    U: Unsigned,
    TMeshData: IMeshData<F, U, TRESOLUTION>,
    TDataType: Unsigned + Copy,
    TVolumeData: IVolumeData<TDataType, TRESOLUTION>,
{
    pub fn new() -> Self {
        Self {
            _threshold: 0.0,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<F, U, TMeshData, TDataType, const TRESOLUTION: usize, TVolumeData>
    IMeshBuilder<F, U, TMeshData, TDataType, TRESOLUTION, TVolumeData>
    for MarchingCubesMeshBuilder<F, U, TMeshData, TDataType, TRESOLUTION, TVolumeData>
where
    F: Float,
    U: Unsigned + num_traits::NumCast,
    TMeshData: IMeshData<F, U, TRESOLUTION>,
    TDataType: Unsigned + ToPrimitive + Copy,
    TVolumeData: IVolumeData<TDataType, TRESOLUTION>,
{
    fn build(&self, mesh_data: &mut TMeshData, volume_data: &TVolumeData) {
        let mut vertex_count = 0;
        for z in 0..(TRESOLUTION - 1) as u32 {
            for y in 0..(TRESOLUTION - 1) as u32 {
                for x in 0..(TRESOLUTION - 1) as u32 {
                    let value = [
                        volume_data.get(x, y, z),
                        volume_data.get(x + 1, y, z),
                        volume_data.get(x, y + 1, z),
                        volume_data.get(x + 1, y + 1, z),
                        volume_data.get(x, y, z + 1),
                        volume_data.get(x + 1, y, z + 1),
                        volume_data.get(x, y + 1, z + 1),
                        volume_data.get(x + 1, y + 1, z + 1),
                    ];
                    let mut pattern = 0;
                    for index in 0..8 {
                        if self._threshold < (value[index].to_f32().unwrap()) {
                            pattern |= 1 << index;
                        }
                    }

                    let triangles = &mesh_pattern::TRIANGLE_TABLE[pattern];
                    for triangle in triangles {
                        if *triangle == -1 {
                            continue;
                        }

                        let position = &mesh_pattern::CUBE_MID_POINT_TABLE[*triangle as usize];
                        let position_x = F::from(position[0]).unwrap() + F::from(x).unwrap();
                        let position_y = F::from(position[1]).unwrap() + F::from(y).unwrap();
                        let position_z = F::from(position[2]).unwrap() + F::from(z).unwrap();
                        mesh_data.push_vertex(position_x, position_y, position_z);
                        vertex_count += 1;
                    }
                }
            }
        }

        // インデクス
        for i in 0..(vertex_count / 3) {
            let index0 = U::from(3 * i + 0).unwrap();
            let index1 = U::from(3 * i + 1).unwrap();
            let index2 = U::from(3 * i + 2).unwrap();
            mesh_data.push_triangle(index0, index1, index2);
        }
    }

    fn with_threshold(mut self, threshold: f32) -> Self {
        self._threshold = threshold;
        self
    }
}
