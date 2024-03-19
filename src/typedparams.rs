use std::ffi::CStr;
use std::str;

pub enum ParamIn<'a> {
    Int32(&'a mut Option<i32>),
    UInt32(&'a mut Option<u32>),
    Int64(&'a mut Option<i64>),
    UInt64(&'a mut Option<u64>),
    #[allow(dead_code)]
    Float64(&'a mut Option<f64>),
    #[allow(dead_code)]
    Bool(&'a mut Option<bool>),
    String(&'a mut Option<String>),
}

pub enum ParamOut<'a> {
    Int32(&'a Option<i32>),
    UInt32(&'a Option<u32>),
    Int64(&'a Option<i64>),
    UInt64(&'a Option<u64>),
    #[allow(dead_code)]
    Float64(&'a Option<f64>),
    #[allow(dead_code)]
    Bool(&'a Option<bool>),
    String(&'a Option<String>),
}

pub struct FieldIn<'a> {
    pub name: String,
    pub value: ParamIn<'a>,
}

pub struct FieldOut<'a> {
    pub name: String,
    pub value: ParamOut<'a>,
}

#[macro_export]
macro_rules! param_field_in {
    ($name:expr, $type:ident, $field:expr) => {
        $crate::typedparams::FieldIn {
            name: unsafe { c_chars_to_string!($name.as_ptr() as *const libc::c_char, nofree) }
                .to_string(),
            value: $crate::typedparams::ParamIn::$type(&mut $field),
        }
    };
}

#[macro_export]
macro_rules! param_field_out {
    ($name:expr, $type:ident, $field:expr) => {
        $crate::typedparams::FieldOut {
            name: unsafe { c_chars_to_string!($name.as_ptr() as *const libc::c_char, nofree) }
                .to_string(),
            value: $crate::typedparams::ParamOut::$type(&$field),
        }
    };
}

macro_rules! valid_type {
    ($got:expr, $want:expr, $name:expr) => {
        if $got != $want {
            panic!(
                "Expected typed param type {} not {} for {}",
                $got, $want, $name
            );
        }
    };
}

pub fn from_params(mut params: Vec<sys::virTypedParameter>, mut fields: Vec<FieldIn>) {
    for param in params.iter_mut() {
        let param_name =
            unsafe { str::from_utf8(CStr::from_ptr(param.field.as_ptr()).to_bytes()).unwrap() };
        for field in fields.iter_mut() {
            if field.name == param_name {
                match &mut field.value {
                    ParamIn::Int32(i) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_INT, param_name);
                        **i = unsafe { Some(param.value.i) }
                    }
                    ParamIn::UInt32(i) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_UINT, param_name);
                        **i = unsafe { Some(param.value.ui) }
                    }
                    ParamIn::Int64(i) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_LLONG, param_name);
                        **i = unsafe { Some(param.value.l) }
                    }
                    ParamIn::UInt64(i) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_ULLONG, param_name);
                        **i = unsafe { Some(param.value.ul) }
                    }
                    ParamIn::Float64(d) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_DOUBLE, param_name);
                        **d = unsafe { Some(param.value.d) }
                    }
                    ParamIn::Bool(b) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_BOOLEAN, param_name);
                        **b = unsafe { Some(param.value.b != 0) }
                    }
                    ParamIn::String(s) => {
                        valid_type!(param.type_ as u32, sys::VIR_TYPED_PARAM_STRING, param_name);
                        **s = unsafe { Some(c_chars_to_string!(param.value.s, nofree)) }
                    }
                }
                break;
            }
        }
    }
}

pub fn to_params(mut fields: Vec<FieldOut>) -> Vec<sys::virTypedParameter> {
    let mut params: Vec<sys::virTypedParameter> = Vec::new();

    for field in fields.iter_mut() {
        let param = match &mut field.value {
            ParamOut::Int32(i) => i.map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_INT as i32,
                value: sys::_virTypedParameterValue { i: v },
            }),
            ParamOut::UInt32(i) => i.map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_UINT as i32,
                value: sys::_virTypedParameterValue { ui: v },
            }),
            ParamOut::Int64(i) => i.map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_LLONG as i32,
                value: sys::_virTypedParameterValue { l: v },
            }),
            ParamOut::UInt64(i) => i.map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_ULLONG as i32,
                value: sys::_virTypedParameterValue { ul: v },
            }),
            ParamOut::Float64(i) => i.map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_DOUBLE as i32,
                value: sys::_virTypedParameterValue { d: v },
            }),
            ParamOut::Bool(i) => i.map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_BOOLEAN as i32,
                value: sys::_virTypedParameterValue {
                    b: v as libc::c_char,
                },
            }),
            ParamOut::String(i) => i.clone().map(|v| sys::virTypedParameter {
                field: to_arr(&field.name),
                type_: sys::VIR_TYPED_PARAM_STRING as i32,
                value: sys::_virTypedParameterValue {
                    s: string_to_mut_c_chars!(v),
                },
            }),
        };
        if let Some(p) = param {
            params.push(p)
        };
    }
    params
}

fn to_arr(name: &str) -> [libc::c_char; 80] {
    let mut field: [libc::c_char; 80] = [0; 80];
    for (a, c) in field.iter_mut().zip(name.as_bytes()) {
        *a = *c as libc::c_char
    }
    field
}

#[cfg(test)]
mod test {

    use crate::typedparams::{from_params, to_params};

    #[derive(PartialEq, Debug)]
    struct Demo {
        vi32: Option<i32>,
        vu32: Option<u32>,
        vi64: Option<i64>,
        vu64: Option<u64>,
        vf64: Option<f64>,
        vbool: Option<bool>,
        vstring: Option<String>,
    }

    macro_rules! fields {
        ($dir:ident, $var:ident) => {
            vec![
                $dir!(b"int32\0", Int32, $var.vi32),
                $dir!(b"uint32\0", UInt32, $var.vu32),
                $dir!(b"int64\0", Int64, $var.vi64),
                $dir!(b"uint64\0", UInt64, $var.vu64),
                $dir!(b"float64\0", Float64, $var.vf64),
                $dir!(b"bool\0", Bool, $var.vbool),
                $dir!(b"string\0", String, $var.vstring),
            ]
        };
    }

    fn roundtrip(demoout: Demo) {
        let fieldsout = fields!(param_field_out, demoout);
        let params: Vec<sys::virTypedParameter> = to_params(fieldsout);

        let mut demoin: Demo = Demo {
            vi32: None,
            vu32: None,
            vi64: None,
            vu64: None,
            vf64: None,
            vbool: None,
            vstring: None,
        };
        let fieldsin = fields!(param_field_in, demoin);
        from_params(params, fieldsin);

        assert!(demoin == demoout);
    }

    #[test]
    fn test_roundtrip_full() {
        let demo: Demo = Demo {
            vi32: Some(-1729),
            vu32: Some(1729),
            vi64: Some(-87539319),
            vu64: Some(87539319),
            vf64: Some(87539319.0),
            vbool: Some(true),
            vstring: Some("it is a very interesting number".to_string()),
        };
        roundtrip(demo);
    }

    #[test]
    fn test_roundtrip_partial() {
        let demo: Demo = Demo {
            vi32: Some(-1729),
            vu32: None,
            vi64: Some(-87539319),
            vu64: Some(87539319),
            vf64: None,
            vbool: None,
            vstring: Some("it is a very interesting number".to_string()),
        };
        roundtrip(demo);
    }
}
