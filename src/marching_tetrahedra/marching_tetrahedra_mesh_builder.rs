use num_traits::{Float, Unsigned};

use crate::{IMeshBuilder, IVolumeData, mesh_builder::IMeshData};

pub struct MarchingTetrahedraMeshBuilder<
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
	IMeshBuilder<F, U, TMeshData, TDataType, TRESOLUTION, TVolumeData>
	for MarchingTetrahedraMeshBuilder<F, U, TMeshData, TDataType, TRESOLUTION, TVolumeData>
where F: Float,
	  U: Unsigned,
	  TMeshData: IMeshData<F, U, TRESOLUTION>,
	  TDataType: Unsigned + Copy,
	  TVolumeData: IVolumeData<TDataType, TRESOLUTION>,
{
    fn build(&self, mesh_data: &mut TMeshData, volume_data: &TVolumeData) {
        todo!()
    }

    fn with_threshold(mut self, threshold: f32) -> Self {
		self._threshold = threshold;
        self
    }
}
