/*! types.h */
use crate::{Float3, Float4};

///simd_float3x3
#[repr(C)] //see MSL table 2.5
#[derive(Debug)]
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

    ///Access the internal columns
    #[inline] pub fn columns_mut(&mut self) -> &mut [crate::Float3; 3] {
        &mut self.columns
    }
}


///simd_float3x3
#[repr(C)] //see MSL table 2.5
#[derive(Debug)]
pub struct Float4x4 {
    pub columns: [crate::Float4; 4]
}
impl Float4x4 {
    #[inline] pub const fn from_columns(columns: [crate::Float4; 4]) -> Self {
        Self { columns: columns }
    }

    /**
    Constructs the matrix
    ```text
        abcd
        efgh
        ijkl
        mnop
    ```
     */
    #[inline] pub const fn from_rowmajor(a: f32, b: f32, c: f32, d: f32, e: f32, f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, l: f32,m:f32,n:f32,o:f32,p:f32 ) -> Self {
        //Apple simd uses simd::transpose for `from_rows`.  It's my opinion that an inline function that takes an unspecified layout
        //parameter is likely to be faster, as the compiler could simply put the argument into the 'right' layout.
        //Of course whether or not this happens is open to debate, but it should be fast enough for my purposes.
        Self {
            columns: [
                Float4::new(a,e,i,m),
                Float4::new(b,f,j,n),
                Float4::new(c,g,k,o),
                Float4::new(d,h,l,p),
            ]
        }
    }
    ///Access the internal columns
    #[inline] pub fn columns_mut(&mut self) -> &mut [crate::Float4; 4] {
        &mut self.columns
    }
}