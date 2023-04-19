macro_rules! impl_marker_trait {
    ($TRAIT:tt, $($TYPE:ty),+) => {
        $(impl $TRAIT for $TYPE {})+
    };
}

//#region Numeric Helper Traits

macro_rules! impl_max_trait {
    ($($TYPE:ty),+) => {
        $(impl HasMaxValue for $TYPE {
            #[inline(always)]
            fn MAX() -> Self {
                Self::MAX
            }
        })+
    };
}

macro_rules! impl_min_trait {
    ($($TYPE:ty),+) => {
        $(impl HasMinValue for $TYPE {
            #[inline(always)]
            fn MIN() -> Self {
                Self::MIN
            }
        })+
    };
}

macro_rules! impl_add_sub_zero_traits {
    ($ONE:expr, $ZERO:expr, $($TYPE:ty),+) => {
        $(
            impl CanAddOne for $TYPE {
                #[inline(always)]
                fn add_one(self) -> Self {
                    self + $ONE
                }
            }

            impl CanSubOne for $TYPE {
                #[inline(always)]
                fn sub_one(self) -> Self {
                    self - $ONE
                }
            }

            impl CanBeZero for $TYPE {
                #[inline(always)]
                fn zero() -> Self {
                    $ZERO
                }
            }
        )+
    };
}

pub trait IsInteger {}
pub trait IsFloat {}
pub trait IsUnsigned {}
pub trait IsSigned {}

impl_marker_trait!(IsInteger, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_marker_trait!(IsUnsigned, u8, u16, u32, u64, u128, usize);
impl_marker_trait!(IsSigned, i8, i16, i32, i64, i128, isize, f32, f64);
impl_marker_trait!(IsFloat, f32, f64);

pub trait HasMaxValue {
    #[allow(non_snake_case)]
    fn MAX() -> Self;
}

pub trait HasMinValue {
    #[allow(non_snake_case)]
    fn MIN() -> Self;
}

pub trait CanAddOne {
    #[allow(non_snake_case)]
    fn add_one(self) -> Self;
}

pub trait CanSubOne {
    #[allow(non_snake_case)]
    fn sub_one(self) -> Self;
}

pub trait CanBeZero {
    #[allow(non_snake_case)]
    fn zero() -> Self;
}

impl_max_trait!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
impl_min_trait!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
impl_add_sub_zero_traits!(1, 0, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_add_sub_zero_traits!(1.0, 0.0, f32, f64);

const _: () = {
    fn assert_int<T: IsInteger>() {}
    fn assert_float<T: IsFloat>() {}
    fn assert_unsigned<T: IsUnsigned>() {}
    fn assert_signed<T: IsSigned>() {}
    fn assert_max<T: HasMaxValue>() {}
    fn assert_min<T: HasMinValue>() {}

    fn assert_all() {
        assert_int::<u8>();
        assert_int::<isize>();
        assert_float::<f32>();
        assert_float::<f64>();
        assert_unsigned::<u8>();
        assert_unsigned::<usize>();
        assert_signed::<i8>();
        assert_signed::<f64>();
        assert_max::<u8>();
        assert_max::<f64>();
        assert_min::<u8>();
        assert_min::<f64>();
    }
};

//#endregion
