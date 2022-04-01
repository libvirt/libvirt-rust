/*
 * This library is free software; you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public
 * License as published by the Free Software Foundation; either
 * version 2.1 of the License, or (at your option) any later version.
 *
 * This library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with this library.  If not, see
 * <http://www.gnu.org/licenses/>.
 *
 * Sahid Orentino Ferdjaoui <sahid.ferdjaoui@redhat.com>
 */

extern crate libc;

pub mod sys {
    extern crate libc;

    use std;

    #[repr(C)]
    #[derive(Copy)]
    pub struct virTypedParameter {
        pub field: [libc::c_char; 80usize],
        pub typed: libc::c_int,
        pub value: _virTypedParameterValue,
    }

    #[repr(C)]
    #[derive(Copy, Clone)]
    pub union _virTypedParameterValue {
        pub i: libc::c_int,
        pub ui: libc::c_uint,
        pub l: libc::c_longlong,
        pub ul: libc::c_ulonglong,
        pub d: libc::c_double,
        pub b: libc::c_char,
        pub s: *mut libc::c_char,
    }

    impl std::clone::Clone for virTypedParameter {
        fn clone(&self) -> Self {
            *self
        }
    }

    impl std::default::Default for virTypedParameter {
        fn default() -> Self {
            unsafe { std::mem::zeroed() }
        }
    }

    pub type virTypedParameterPtr = *mut virTypedParameter;
}

pub type TypedParameterType = self::libc::c_int;
pub const VIR_TYPED_PARAM_INT: TypedParameterType = 1;
pub const VIR_TYPED_PARAM_UINT: TypedParameterType = 2;
pub const VIR_TYPED_PARAM_LLONG: TypedParameterType = 3;
pub const VIR_TYPED_PARAM_ULLONG: TypedParameterType = 4;
pub const VIR_TYPED_PARAM_DOUBLE: TypedParameterType = 5;
pub const VIR_TYPED_PARAM_BOOLEAN: TypedParameterType = 6;
pub const VIR_TYPED_PARAM_STRING: TypedParameterType = 7;
