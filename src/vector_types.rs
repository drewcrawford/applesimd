use std::mem::MaybeUninit;

#[derive(Copy,Clone,Debug)]
#[repr(C, align(16))]
pub struct Int3 {
    x: i32,
    y: i32,
    z: i32,
    w: MaybeUninit<i32>,
}
impl Int3 {
    #[inline] pub const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,y,z,w: MaybeUninit::uninit()
        }
    }
}

#[repr(C, align(16))]
pub struct Int4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
impl Int4 {
    pub const fn new(x: i32, y: i32, z: i32, w: i32) -> Self {
        Self { x,y,z,w }
    }
}




#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct Float2{
    x: f32,
    y: MaybeUninit<f32>,
}
impl Float2 {

    #[inline] pub const unsafe fn from_undef(x: f32) -> Self {
        Float2 {
            x: x,
            y: MaybeUninit::uninit(),
        }
    }
    #[inline] pub const fn new(x: f32, y: f32) -> Self {
        Float2 {
            x: x,
            y: MaybeUninit::new(y)
        }
    }
    #[inline] pub const fn x(&self) -> f32 {
        self.x
    }
    #[inline] pub const fn y(&self) -> f32 {
        unsafe{self.y.assume_init()}
    }
}

/// simd_float3
#[repr(C,align(16))]
#[derive(Copy,Clone,Debug)]
pub struct Float3 {
    x: f32,
    y: f32,
    z: f32,
    padding: MaybeUninit<f32>, //MSL table 2.3, float3 is 16 bytes
}

impl PartialEq for Float3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Float3 {
    pub const ZERO: Self = Float3::new(0.0, 0.0, 0.0);
    ///Creates a new type.
    #[inline] pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Float3 {
            x,
            y,
            z,
            padding: MaybeUninit::uninit()
        }
    }
    #[inline] pub const fn x(&self) -> f32 {
        self.x
    }
    #[inline] pub const fn y(&self) -> f32 {
        self.y
    }
    #[inline] pub const fn z(&self) -> f32 {
        self.z
    }
    #[inline] pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    #[inline] pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    #[inline] pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }



}

/// simd_float4
#[repr(C)]
#[derive(Copy,Clone,Debug)]
pub struct Float4 {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
impl Float4 {
    ///Creates a new type.
    #[inline] pub const fn new(x: f32, y: f32, z: f32,w: f32) -> Self {
        Float4 {
            x,
            y,
            z,
            w,
        }
    }
    #[inline] pub const fn x(&self) -> f32 {
        self.x
    }
    #[inline] pub const fn y(&self) -> f32 {
        self.y
    }
    #[inline] pub const fn z(&self) -> f32 {
        self.z
    }
    #[inline] pub const fn w(&self) -> f32 {
        self.w
    }

    #[inline] pub fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }
    #[inline] pub fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
    #[inline] pub fn z_mut(&mut self) -> &mut f32 {
        &mut self.z
    }
    #[inline] pub fn w_mut(&mut self) -> &mut f32 {
        &mut self.w
    }
}
