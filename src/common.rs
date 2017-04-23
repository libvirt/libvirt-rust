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

#![allow(improper_ctypes)]

extern crate libc;

pub mod sys {
    extern crate libc;

    #[allow(non_camel_case_types)]
    #[allow(non_snake_case)]
    #[repr(C)]
    pub struct virTypedParameter {
        pub field: [libc::c_char; 80],
        pub typed: libc::c_int,
        pub value: libc::c_void,
    }

    #[allow(non_camel_case_types)]
    pub type virTypedParameterPtr = *mut virTypedParameter;
}

pub type TypedParameterType = self::libc::c_uint;
pub const VIR_TYPED_PARAM_INT: TypedParameterType = 1;
pub const VIR_TYPED_PARAM_UINT: TypedParameterType = 2;
pub const VIR_TYPED_PARAM_LLONG: TypedParameterType = 3;
pub const VIR_TYPED_PARAM_ULLONG: TypedParameterType = 4;
pub const VIR_TYPED_PARAM_DOUBLE: TypedParameterType = 5;
pub const VIR_TYPED_PARAM_BOOLEAN: TypedParameterType = 6;
pub const VIR_TYPED_PARAM_STRING: TypedParameterType = 7;
