use num_traits::Unsigned;

pub struct VolumeData<T: Unsigned> {
    _marker: std::marker::PhantomData<T>,
}

impl<T: Unsigned> VolumeData<T> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }
}
