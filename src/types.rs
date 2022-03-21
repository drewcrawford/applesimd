/*! types.h */
use crate::Float3;

///simd_float3x3
#[repr(C)] //see MSL table 2.5
pub struct Float3x3 {
    pub columns: [crate::Float3; 3]
}

impl Float3x3 {
    #[inline] pub const fn from_columns(columns: [crate::Float3; 3]) -> Self {
        Self { columns: columns }
    }
    /**
    Constructs the matrix
    ```text
        abc
        def
        ghi
    ```
    */
    #[inline] pub const fn from_rowmajor(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32) -> Self {
        //Apple simd uses simd::transpose for `from_rows`.  It's my opinion that an inline function that takes an unspecified layout
        //parameter is likely to be faster, as the compiler could simply put the argument into the 'right' layout.
        //Of course whether or not this happens is open to debate, but it should be fast enough for my purposes.
        Self {
            columns: [
                Float3::new(a, d, g),
                Float3::new(b, e, h),
                Float3::new(c, f, i),
            ]
        }
    }
}
