use std::arch::aarch64::{vminvq_u32, vrsqrte_f32, vrsqrts_f32};
use std::mem::MaybeUninit;
use std::ops::{BitAnd, BitOr, Neg};

#[derive(Copy,Clone,Debug)]
#[repr(C, align(16))]
pub struct Int3 {
    x: i32,
    y: i32,
    z: i32,
    w: MaybeUninit<i32>,
}
impl Int3 {
    #[inline] const fn new(x: i32, y: i32, z: i32) -> Self {
        Self {
            x,y,z,w: MaybeUninit::uninit()
        }
    }
    #[inline] fn bitwise_neg(self) -> Self {
        Self::new(self.x.neg(), self.y.neg(), self.z.neg())
    }
    #[inline] fn bitwise_and(self,other: Self) -> Self {
        Self::new(self.x.bitand(other.x), self.y.bitand(other.y), self.z.bitand(other.z))
    }
    #[inline] fn bitwise_or(self, other: Self) -> Self {
        Self::new(self.x.bitor(other.x), self.y.bitor(other.y), self.z.bitor(other.z))
    }
    #[inline] fn bitselect(self, y: Self, mask: Self) -> Self {
        self.bitwise_and(mask.bitwise_neg()).bitwise_or(y.bitwise_and(mask))
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
    #[inline] pub fn fast_rsqrt(self) -> Self {
        unsafe {
            let r = vrsqrte_f32(std::mem::transmute(self));
            let r_rust: Float2 = std::mem::transmute(r);
            let r_squared = r_rust.elementwise_mult(r_rust);
            let t = vrsqrts_f32(std::mem::transmute(self), std::mem::transmute(r_squared));
            let t_rust: Float2 = std::mem::transmute(t);
            r_rust.elementwise_mult(t_rust)
        }
    }
    #[inline] pub fn elementwise_mult(self,other: Self) -> Self {
        Float2 {
            x: self.x * other.x,
            y: MaybeUninit::new(unsafe{self.y.assume_init() * other.y.assume_init()}),
        }
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


    ///The * operator in simd.
    #[inline] pub fn elementwise_mult(self, other: Float3) -> Self {
        Float3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
    #[inline] pub fn elementwise_mult_scalar(self, other: f32) -> Self {
        Float3::new(self.x * other, self.y * other, self.z * other)
    }

    #[inline] pub fn elementwise_sub(self, other: Float3) -> Self {
        Float3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
    #[inline] pub fn elementwise_add(self, other: Float3) -> Self {
        Float3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
    #[inline] pub fn bitselect(self, y: Float3, mask: Int3) -> Float3 {
        unsafe {
            let x: Int3 = std::mem::transmute(self);
            let y: Int3 = std::mem::transmute(y);
            let result = x.bitselect(y,mask);
            let r_f3: Float3 = std::mem::transmute(result);
            r_f3
        }
    }
    #[inline] pub fn abs(self) -> Float3 {
        let zero = Float3::ZERO;
        zero.bitselect(self, Int3::new(0x7fffffff, 0x7fffffff, 0x7fffffff))
    }

    #[inline] pub fn reduce_add(self) -> f32 {
        self.x + self.y + self.z
    }

    ///simd_dot
    #[inline] pub fn dot(self,other: Self) -> f32 {
        self.elementwise_mult(other).reduce_add()
    }

    #[inline] pub fn length_squared(self) -> f32 {
        self.dot(self)
    }

    #[inline] pub fn fast_normalize(self) -> Self {
        let length = self.length_squared();
        let rsqrd = unsafe {
            let extend = Float2::from_undef(length);
            extend.fast_rsqrt().x
        };
        self.elementwise_mult_scalar( rsqrd)
    }
    #[inline] pub fn fast_length(self) -> f32 {
        self.precise_length() //odd, I know!??
    }
    #[inline] pub fn precise_length(self) -> f32 {
        self.length_squared().sqrt()
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


#[test] fn test_rsqrt() {
    let float2 = Float2::new(1.0, 25.0);
    let sqrt = float2.fast_rsqrt();
    assert!((sqrt.x() - 1.0).abs() < 0.1);
    assert!((sqrt.y() - 0.2).abs() < 0.1);
}

#[test] fn test_normalize() {
    let norm = Float3::new(1.0, 2.0, 3.0).fast_normalize();
    assert!((norm.x() - 0.26).abs() < 0.1);
    assert!((norm.y() - 0.53).abs() < 0.1);
    assert!((norm.z() - 0.80).abs() < 0.1);
}

#[test] fn test_abs() {
    let a = Float3::new(0.0, -20.0, 13370000.0);
    let abs = a.abs();
    assert_eq!(a.x.abs(),abs.x);
    assert_eq!(a.y.abs(),abs.y);
    assert_eq!(a.z.abs(),abs.z);

}