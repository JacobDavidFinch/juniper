//! Helper traits and macros for compile-time reflection.

use std::{rc::Rc, sync::Arc};

use futures::future::BoxFuture;

use crate::{Arguments, DefaultScalarValue, ExecutionResult, Executor, Nullable, ScalarValue};

pub type Type = &'static str;
pub type Types = &'static [Type];
pub type Name = &'static str;
pub type Names = &'static [Name];
pub type FieldName = u128;
pub type WrappedValue = u128;

/// TODO
pub trait BaseType<S = DefaultScalarValue> {
    const NAME: Type;
}

impl<'a, S, T: BaseType<S> + ?Sized> BaseType<S> for &'a T {
    const NAME: Type = T::NAME;
}

// TODO: Reconsider
impl<S, T: BaseType<S>, Ctx> BaseType<S> for (Ctx, T) {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S>> BaseType<S> for Option<T> {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S>> BaseType<S> for Nullable<T> {
    const NAME: Type = T::NAME;
}

// TODO: Should Err be trait bounded somehow?
impl<S, T: BaseType<S>, E> BaseType<S> for Result<T, E> {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S>> BaseType<S> for Vec<T> {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S>> BaseType<S> for [T] {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S>, const N: usize> BaseType<S> for [T; N] {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S> + ?Sized> BaseType<S> for Box<T> {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S> + ?Sized> BaseType<S> for Arc<T> {
    const NAME: Type = T::NAME;
}

impl<S, T: BaseType<S> + ?Sized> BaseType<S> for Rc<T> {
    const NAME: Type = T::NAME;
}

/// TODO
pub trait BaseSubTypes<S = DefaultScalarValue> {
    const NAMES: Types;
}

impl<'a, S, T: BaseSubTypes<S> + ?Sized> BaseSubTypes<S> for &'a T {
    const NAMES: Types = T::NAMES;
}

// TODO: Reconsider
impl<S, T: BaseSubTypes<S>, Ctx> BaseSubTypes<S> for (Ctx, T) {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S>> BaseSubTypes<S> for Option<T> {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S>> BaseSubTypes<S> for Nullable<T> {
    const NAMES: Types = T::NAMES;
}

// TODO: Should Err be trait bounded somehow?
impl<S, T: BaseSubTypes<S>, E> BaseSubTypes<S> for Result<T, E> {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S>> BaseSubTypes<S> for Vec<T> {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S>> BaseSubTypes<S> for [T] {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S>, const N: usize> BaseSubTypes<S> for [T; N] {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S> + ?Sized> BaseSubTypes<S> for Box<T> {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S> + ?Sized> BaseSubTypes<S> for Arc<T> {
    const NAMES: Types = T::NAMES;
}

impl<S, T: BaseSubTypes<S> + ?Sized> BaseSubTypes<S> for Rc<T> {
    const NAMES: Types = T::NAMES;
}

/// TODO
pub trait WrappedType<S = DefaultScalarValue> {
    /// NonNull  - 1
    /// Nullable - 2
    /// List     - 3
    ///
    /// `[[Int]!] - <Option<Vec<Vec<Option<i32>>>> as WrappedType>::N = 12332`
    const VALUE: u128;
}

impl<'a, S, T: WrappedType<S>> WrappedType<S> for (&'a T::Context, T)
where
    S: ScalarValue,
    T: crate::GraphQLValue<S>,
{
    const VALUE: u128 = T::VALUE;
}

impl<S, T: WrappedType<S>> WrappedType<S> for Option<T> {
    const VALUE: u128 = T::VALUE * 10 + 2;
}

impl<S, T: WrappedType<S>> WrappedType<S> for Nullable<T> {
    const VALUE: u128 = T::VALUE * 10 + 2;
}

// TODO: Should Err be trait bounded somehow?
//       And should `VALUE` be `T::VALUE` or `T::VALUE * 10 + 2`?
impl<S, T: WrappedType<S>, E> WrappedType<S> for Result<T, E> {
    const VALUE: u128 = T::VALUE;
}

impl<S, T: WrappedType<S>> WrappedType<S> for Vec<T> {
    const VALUE: u128 = T::VALUE * 10 + 3;
}

impl<S, T: WrappedType<S>> WrappedType<S> for [T] {
    const VALUE: u128 = T::VALUE * 10 + 3;
}

impl<S, T: WrappedType<S>, const N: usize> WrappedType<S> for [T; N] {
    const VALUE: u128 = T::VALUE * 10 + 3;
}

impl<'a, S, T: WrappedType<S> + ?Sized> WrappedType<S> for &'a T {
    const VALUE: u128 = T::VALUE;
}

impl<S, T: WrappedType<S> + ?Sized> WrappedType<S> for Box<T> {
    const VALUE: u128 = T::VALUE;
}

impl<S, T: WrappedType<S> + ?Sized> WrappedType<S> for Arc<T> {
    const VALUE: u128 = T::VALUE;
}

impl<S, T: WrappedType<S> + ?Sized> WrappedType<S> for Rc<T> {
    const VALUE: u128 = T::VALUE;
}

pub trait Fields<S = DefaultScalarValue> {
    const NAMES: Names;
}

pub trait FieldMeta<S, const N: FieldName> {
    type Context;
    type TypeInfo;
    const TYPE: Type;
    const SUB_TYPES: Types;
    const WRAPPED_VALUE: WrappedValue;
    const ARGUMENTS: &'static [(Name, Type, WrappedValue)];
}

pub trait Field<S, const N: FieldName>: FieldMeta<S, N> {
    fn call(
        &self,
        info: &Self::TypeInfo,
        args: &Arguments<S>,
        executor: &Executor<Self::Context, S>,
    ) -> ExecutionResult<S>;
}

pub trait AsyncField<S, const N: FieldName>: FieldMeta<S, N> {
    fn call<'b>(
        &'b self,
        info: &'b Self::TypeInfo,
        args: &'b Arguments<S>,
        executor: &'b Executor<Self::Context, S>,
    ) -> BoxFuture<'b, ExecutionResult<S>>;
}

/// Non-cryptographic hash with good dispersion to use [`str`](prim@str) in
/// const generics. See [spec] for more info.
///
/// [spec]: https://datatracker.ietf.org/doc/html/draft-eastlake-fnv-17.html
pub const fn fnv1a128(str: Name) -> u128 {
    const FNV_OFFSET_BASIS: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013b;

    let bytes = str.as_bytes();
    let mut hash = FNV_OFFSET_BASIS;
    let mut i = 0;
    while i < bytes.len() {
        hash ^= bytes[i] as u128;
        hash = hash.wrapping_mul(FNV_PRIME);
        i += 1;
    }
    hash
}

pub const fn str_eq(l: &str, r: &str) -> bool {
    let (l, r) = (l.as_bytes(), r.as_bytes());

    if l.len() != r.len() {
        return false;
    }

    let mut i = 0;
    while i < l.len() {
        if l[i] != r[i] {
            return false;
        }
        i += 1;
    }

    true
}

pub const fn str_len_from_wrapped_val(ty: Type, v: WrappedValue) -> usize {
    let mut len = ty.as_bytes().len() + "!".as_bytes().len(); // Type!

    let mut current_wrap_val = v;
    while current_wrap_val % 10 != 0 {
        match current_wrap_val % 10 {
            2 => len -= "!".as_bytes().len(),   // remove !
            3 => len += "[]!".as_bytes().len(), // [Type]!
            _ => {}
        }

        current_wrap_val /= 10;
    }

    len
}

pub const fn can_be_subtype(ty: WrappedValue, subtype: WrappedValue) -> bool {
    let ty_current = ty % 10;
    let subtype_current = subtype % 10;

    if ty_current == subtype_current {
        if ty_current == 1 {
            true
        } else {
            can_be_subtype(ty / 10, subtype / 10)
        }
    } else if ty_current == 2 {
        can_be_subtype(ty / 10, subtype)
    } else {
        false
    }
}

pub const fn exists(val: Type, arr: Types) -> bool {
    let mut i = 0;
    while i < arr.len() {
        if str_eq(val, arr[i]) {
            return true;
        }
        i += 1;
    }
    false
}

#[macro_export]
macro_rules! assert_field {
    (
        $base_ty: ty,
        $impl_ty: ty,
        $scalar: ty,
        $field_name: expr $(,)?
    ) => {
        $crate::assert_field_args!($base_ty, $impl_ty, $scalar, $field_name);
        $crate::assert_subtype!($base_ty, $impl_ty, $scalar, $field_name);
    };
}

#[macro_export]
macro_rules! assert_subtype {
    (
        $base_ty: ty,
        $impl_ty: ty,
        $scalar: ty,
        $field_name: expr $(,)?
    ) => {
        const _: () = {
            const BASE_TY: $crate::macros::reflection::Type =
                <$base_ty as $crate::macros::reflection::BaseType<$scalar>>::NAME;
            const IMPL_TY: $crate::macros::reflection::Type =
                <$impl_ty as $crate::macros::reflection::BaseType<$scalar>>::NAME;
            const ERR_PREFIX: &str = $crate::const_concat!(
                "Failed to implement interface `",
                BASE_TY,
                "` on `",
                IMPL_TY,
                "`: ",
            );

            const FIELD_NAME: $crate::macros::reflection::Name =
                $field_name;
            const BASE_RETURN_WRAPPED_VAL: $crate::macros::reflection::WrappedValue =
                <$base_ty as $crate::macros::reflection::FieldMeta<
                    $scalar,
                    { $crate::hash!(FIELD_NAME, $base_ty, $scalar, ERR_PREFIX) },
                >>::WRAPPED_VALUE;
            const IMPL_RETURN_WRAPPED_VAL: $crate::macros::reflection::WrappedValue =
                <$impl_ty as $crate::macros::reflection::FieldMeta<
                    $scalar,
                    { $crate::hash!(FIELD_NAME, $impl_ty, $scalar, ERR_PREFIX) },
                >>::WRAPPED_VALUE;

            const BASE_RETURN_SUB_TYPES: $crate::macros::reflection::Types =
                <$base_ty as $crate::macros::reflection::FieldMeta<
                    $scalar,
                    { $crate::hash!(FIELD_NAME, $base_ty, $scalar, ERR_PREFIX) },
                >>::SUB_TYPES;

            const BASE_RETURN_TY: $crate::macros::reflection::Type =
                <$base_ty as $crate::macros::reflection::FieldMeta<
                    $scalar,
                    { $crate::hash!(FIELD_NAME, $base_ty, $scalar, ERR_PREFIX) },
                >>::TYPE;
            const IMPL_RETURN_TY: $crate::macros::reflection::Type =
                <$impl_ty as $crate::macros::reflection::FieldMeta<
                    $scalar,
                    { $crate::hash!(FIELD_NAME, $impl_ty, $scalar, ERR_PREFIX) },
                >>::TYPE;

            let is_subtype = $crate::macros::reflection::exists(IMPL_RETURN_TY, BASE_RETURN_SUB_TYPES)
                && $crate::macros::reflection::can_be_subtype(BASE_RETURN_WRAPPED_VAL, IMPL_RETURN_WRAPPED_VAL);
            if !is_subtype {
                const MSG: &str = $crate::const_concat!(
                    ERR_PREFIX,
                    "Field `",
                    FIELD_NAME,
                    "`: implementor is expected to return a subtype of interface's return object: `",
                    $crate::format_type!(IMPL_RETURN_TY, IMPL_RETURN_WRAPPED_VAL),
                    "` is not a subtype of `",
                    $crate::format_type!(BASE_RETURN_TY, BASE_RETURN_WRAPPED_VAL),
                    "`.",
                );
                ::std::panic!("{}", MSG);
            }
        };
    };
}

#[macro_export]
macro_rules! assert_field_args {
    (
        $base_ty: ty,
        $impl_ty: ty,
        $scalar: ty,
        $field_name: expr $(,)?
    ) => {
        const _: () = {
            type FullArg = (
                $crate::macros::reflection::Name,
                $crate::macros::reflection::Type,
                $crate::macros::reflection::WrappedValue,
            );

            const BASE_NAME: &str =
                <$base_ty as $crate::macros::reflection::BaseType<$scalar>>::NAME;
            const IMPL_NAME: &str =
                <$impl_ty as $crate::macros::reflection::BaseType<$scalar>>::NAME;
            const ERR_PREFIX: &str = $crate::const_concat!(
                "Failed to implement interface `",
                BASE_NAME,
                "` on `",
                IMPL_NAME,
                "`: ",
            );

            const FIELD_NAME: &str = $field_name;
            const BASE_ARGS: &[FullArg] = <$base_ty as $crate::macros::reflection::FieldMeta<
                $scalar,
                { $crate::hash!(FIELD_NAME, $base_ty, $scalar, ERR_PREFIX) },
            >>::ARGUMENTS;
            const IMPL_ARGS: &[FullArg] = <$impl_ty as $crate::macros::reflection::FieldMeta<
                $scalar,
                { $crate::hash!(FIELD_NAME, $impl_ty, $scalar, ERR_PREFIX) },
            >>::ARGUMENTS;

            struct Error {
                cause: Cause,
                base: FullArg,
                implementation: FullArg,
            }

            enum Cause {
                RequiredField,
                AdditionalNonNullableField,
                TypeMismatch,
            }

            const fn unwrap_error(v: ::std::result::Result<(), Error>) -> Error {
                match v {
                    Ok(()) => Error {
                        cause: Cause::RequiredField,
                        base: ("unreachable", "unreachable", 1),
                        implementation: ("unreachable", "unreachable", 1),
                    },
                    Err(err) => err,
                }
            }

            const fn check() -> Result<(), Error> {
                let mut base_i = 0;
                while base_i < BASE_ARGS.len() {
                    let (base_name, base_type, base_wrap_val) = BASE_ARGS[base_i];

                    let mut impl_i = 0;
                    let mut was_found = false;
                    while impl_i < IMPL_ARGS.len() {
                        let (impl_name, impl_type, impl_wrap_val) = IMPL_ARGS[impl_i];

                        if $crate::macros::reflection::str_eq(base_name, impl_name) {
                            if $crate::macros::reflection::str_eq(base_type, impl_type)
                                && base_wrap_val == impl_wrap_val
                            {
                                was_found = true;
                                break;
                            } else {
                                return Err(Error {
                                    cause: Cause::TypeMismatch,
                                    base: (base_name, base_type, base_wrap_val),
                                    implementation: (impl_name, impl_type, impl_wrap_val),
                                });
                            }
                        }

                        impl_i += 1;
                    }

                    if !was_found {
                        return Err(Error {
                            cause: Cause::RequiredField,
                            base: (base_name, base_type, base_wrap_val),
                            implementation: (base_name, base_type, base_wrap_val),
                        });
                    }

                    base_i += 1;
                }

                let mut impl_i = 0;
                while impl_i < IMPL_ARGS.len() {
                    let (impl_name, impl_type, impl_wrapped_val) = IMPL_ARGS[impl_i];
                    impl_i += 1;

                    if impl_wrapped_val % 10 == 2 {
                        continue;
                    }

                    let mut base_i = 0;
                    let mut was_found = false;
                    while base_i < BASE_ARGS.len() {
                        let (base_name, _, _) = BASE_ARGS[base_i];
                        if $crate::macros::reflection::str_eq(base_name, impl_name) {
                            was_found = true;
                            break;
                        }
                        base_i += 1;
                    }
                    if !was_found {
                        return Err(Error {
                            cause: Cause::AdditionalNonNullableField,
                            base: (impl_name, impl_type, impl_wrapped_val),
                            implementation: (impl_name, impl_type, impl_wrapped_val),
                        });
                    }
                }

                Ok(())
            }

            const RES: ::std::result::Result<(), Error> = check();
            if RES.is_err() {
                const ERROR: Error = unwrap_error(RES);

                const BASE_ARG_NAME: &str = ERROR.base.0;
                const IMPL_ARG_NAME: &str = ERROR.implementation.0;

                const BASE_TYPE_FORMATTED: &str = $crate::format_type!(ERROR.base.1, ERROR.base.2);
                const IMPL_TYPE_FORMATTED: &str =
                    $crate::format_type!(ERROR.implementation.1, ERROR.implementation.2);

                const MSG: &str = match ERROR.cause {
                    Cause::TypeMismatch => {
                        $crate::const_concat!(
                            "Argument `",
                            BASE_ARG_NAME,
                            "`: expected type `",
                            BASE_TYPE_FORMATTED,
                            "`, found: `",
                            IMPL_TYPE_FORMATTED,
                            "`.",
                        )
                    }
                    Cause::RequiredField => {
                        $crate::const_concat!(
                            "Argument `",
                            BASE_ARG_NAME,
                            "` of type `",
                            BASE_TYPE_FORMATTED,
                            "` was expected, but not found."
                        )
                    }
                    Cause::AdditionalNonNullableField => {
                        $crate::const_concat!(
                            "Argument `",
                            IMPL_ARG_NAME,
                            "` of type `",
                            IMPL_TYPE_FORMATTED,
                            "` not present on the interface and so has to be nullable."
                        )
                    }
                };
                const ERROR_MSG: &str =
                    $crate::const_concat!(ERR_PREFIX, "Field `", FIELD_NAME, "`: ", MSG);
                ::std::panic!("{}", ERROR_MSG);
            }
        };
    };
}

#[macro_export]
macro_rules! const_concat {
    ($($s:expr),* $(,)?) => {{
        const LEN: usize = 0 $(+ $s.len())*;
        const CNT: usize = [$($s),*].len();
        const fn concat(input: [&str; CNT]) -> [u8; LEN] {
            let mut bytes = [0; LEN];
            let (mut i, mut byte) = (0, 0);
            while i < CNT {
                let mut b = 0;
                while b < input[i].len() {
                    bytes[byte] = input[i].as_bytes()[b];
                    byte += 1;
                    b += 1;
                }
                i += 1;
            }
            bytes
        }
        const CON: [u8; LEN] = concat([$($s),*]);
        unsafe { std::str::from_utf8_unchecked(&CON) }
    }};
}

#[macro_export]
macro_rules! hash {
    ($field_name: expr, $impl_ty: ty, $scalar: ty $(, $prefix: expr)? $(,)?) => {{
        let exists = $crate::macros::reflection::exists(
            $field_name,
            <$impl_ty as $crate::macros::reflection::Fields<$scalar>>::NAMES,
        );
        if exists {
            $crate::macros::reflection::fnv1a128(FIELD_NAME)
        } else {
            ::std::panic!(
                "{}",
                $crate::const_concat!(
                    $($prefix,)?
                    "Field `",
                    $field_name,
                    "` isn't implemented on `",
                    <$impl_ty as $crate::macros::reflection::BaseType<$scalar>>::NAME,
                    "`."
                )
            )
        }
    }};
}

#[macro_export]
macro_rules! format_type {
    ($ty: expr, $wrapped_value: expr $(,)?) => {{
        const TYPE: (
            $crate::macros::reflection::Type,
            $crate::macros::reflection::WrappedValue,
        ) = ($ty, $wrapped_value);
        const RES_LEN: usize = $crate::macros::reflection::str_len_from_wrapped_val(TYPE.0, TYPE.1);

        const OPENING_BRACKET: &str = "[";
        const CLOSING_BRACKET: &str = "]";
        const BANG: &str = "!";

        const fn format_type_arr() -> [u8; RES_LEN] {
            let (ty, wrap_val) = TYPE;
            let mut type_arr: [u8; RES_LEN] = [0; RES_LEN];

            let mut current_start = 0;
            let mut current_end = RES_LEN - 1;
            let mut current_wrap_val = wrap_val;
            let mut is_null = false;
            while current_wrap_val % 10 != 0 {
                match current_wrap_val % 10 {
                    2 => is_null = true,
                    3 => {
                        let mut i = 0;
                        while i < OPENING_BRACKET.as_bytes().len() {
                            type_arr[current_start + i] = OPENING_BRACKET.as_bytes()[i];
                            i += 1;
                        }
                        current_start += i;
                        if !is_null {
                            i = 0;
                            while i < BANG.as_bytes().len() {
                                type_arr[current_end - BANG.as_bytes().len() + i + 1] =
                                    BANG.as_bytes()[i];
                                i += 1;
                            }
                            current_end -= i;
                        }
                        i = 0;
                        while i < CLOSING_BRACKET.as_bytes().len() {
                            type_arr[current_end - CLOSING_BRACKET.as_bytes().len() + i + 1] =
                                CLOSING_BRACKET.as_bytes()[i];
                            i += 1;
                        }
                        current_end -= i;
                        is_null = false;
                    }
                    _ => {}
                }

                current_wrap_val /= 10;
            }

            let mut i = 0;
            while i < ty.as_bytes().len() {
                type_arr[current_start + i] = ty.as_bytes()[i];
                i += 1;
            }
            i = 0;
            if !is_null {
                while i < BANG.as_bytes().len() {
                    type_arr[current_end - BANG.as_bytes().len() + i + 1] = BANG.as_bytes()[i];
                    i += 1;
                }
            }

            type_arr
        }

        const TYPE_ARR: [u8; RES_LEN] = format_type_arr();
        // SAFETY: This is safe, as `TYPE_ARR` was formatted from `Type`, `[`, `]` and `!`.
        const TYPE_FORMATTED: &str = unsafe { ::std::mem::transmute(TYPE_ARR.as_slice()) };
        TYPE_FORMATTED
    }};
}