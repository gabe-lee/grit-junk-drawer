macro_rules! impl_marker_trait {
    ($TRAIT:tt, $($TYPE:ty),+) => {
        $(impl $TRAIT for $TYPE {})+
    };
}

//#region Numeric Helper Traits

macro_rules! impl_max_trait {
    ($TRAIT:tt, $($TYPE:ty),+) => {
        $(impl $TRAIT for $TYPE {
            #[inline(always)]
            fn MAX() -> Self {
                Self::MAX
            }
        })+
    };
}

macro_rules! impl_min_trait {
    ($TRAIT:tt, $($TYPE:ty),+) => {
        $(impl $TRAIT for $TYPE {
            #[inline(always)]
            fn MIN() -> Self {
                Self::MIN
            }
        })+
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

impl_max_trait!(HasMaxValue, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
impl_min_trait!(HasMinValue, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);

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
