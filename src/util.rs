macro_rules! impl_enum {
    (enum: $type:ty, raw: $raw:ty, match: { $($match_arms:tt)* }) => {
        impl $type {
            /// Converts libvirt C enum constant to Rust enum.
            pub fn from_raw(raw: $raw) -> Self {
                $crate::util::impl_enum_from!(raw, $($match_arms)*)
            }

            /// Converts Rust enum to libvirt C enum constant.
            pub fn to_raw(self) -> $raw {
                $crate::util::impl_enum_to!(self, $($match_arms)*)
            }
        }
    };
}

macro_rules! impl_enum_from {
    (@acc ($e:expr, _ => $type:path => $_raw:path,) -> ($($body:tt)*)) => {
        $crate::util::impl_enum_from!(@final ($e) -> ($($body)* _ => $type,))
    };
    (@acc ($e:expr, _ => $type:path,) -> ($($body:tt)*)) => {
        $crate::util::impl_enum_from!(@final ($e) -> ($($body)* _ => $type,))
    };
    (@acc ($e:expr, $(#[$attr:meta])* $raw:path => $type:path, $($match_arms:tt)*) -> ($($body:tt)*)) => {
        $crate::util::impl_enum_from!(@acc ($e, $($match_arms)*) -> ($($body)* $(#[$attr])* $raw => $type,))
    };
    (@final ($e:expr) -> ($($body:tt)*)) => {
        match $e { $($body)* }
    };
    ($e:expr, $($match_arms:tt)*) => {
        $crate::util::impl_enum_from!(@acc ($e, $($match_arms)*) -> ())
    };
}

macro_rules! impl_enum_to {
    (@acc ($e:expr, _ => $type:path => $raw:path,) -> ($($body:tt)*)) => {
        $crate::util::impl_enum_to!(@final ($e) -> ($($body)* $type => $raw,))
    };
    (@acc ($e:expr, _ => $_type:path,) -> ($($body:tt)*)) => {
        $crate::util::impl_enum_to!(@final ($e) -> ($($body)*))
    };
    (@acc ($e:expr, $(#[$attr:meta])* $raw:path => $type:path, $($match_arms:tt)*) -> ($($body:tt)*)) => {
        $crate::util::impl_enum_to!(@acc ($e, $($match_arms)*) -> ($($body)* $(#[$attr])* $type => $raw,))
    };
    (@final ($e:expr) -> ($($body:tt)*)) => {
        match $e { $($body)* }
    };
    ($e:expr, $($match_arms:tt)*) => {
        $crate::util::impl_enum_to!(@acc ($e, $($match_arms)*) -> ())
    };
}

pub(crate) use impl_enum;
pub(crate) use impl_enum_from;
pub(crate) use impl_enum_to;

#[cfg(all(target_pointer_width = "64", not(windows)))]
pub fn c_ulong_to_u64(val: ::libc::c_ulong) -> u64 {
    val
}

#[cfg(not(all(target_pointer_width = "64", not(windows))))]
pub fn c_ulong_to_u64(val: ::libc::c_ulong) -> u64 {
    val as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const FOO: u32 = 0;
    const BAR: u32 = 1;
    const BAZ: u32 = 2;

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum WithoutLast {
        Foo,
        Bar,
        Baz,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    enum WithLast {
        Foo,
        Bar,
        Baz,
        Last,
    }

    impl_enum! {
        enum: WithoutLast,
        raw: u32,
        match: {
            FOO => WithoutLast::Foo,
            BAR => WithoutLast::Bar,
            BAZ => WithoutLast::Baz,
            _ => WithoutLast::Foo,
        }
    }

    impl_enum! {
        enum: WithLast,
        raw: u32,
        match: {
            FOO => WithLast::Foo,
            BAR => WithLast::Bar,
            BAZ => WithLast::Baz,
            _ => WithLast::Last => FOO,
        }
    }

    #[test]
    fn test_enum_without_last_from_raw() {
        let inputs = [
            (FOO, WithoutLast::Foo),
            (BAR, WithoutLast::Bar),
            (BAZ, WithoutLast::Baz),
            (10, WithoutLast::Foo),
        ];

        for &(raw, expected) in inputs.iter() {
            assert_eq!(WithoutLast::from_raw(raw), expected);
        }
    }

    #[test]
    fn test_enum_without_last_to_raw() {
        let inputs = [
            (WithoutLast::Foo, FOO),
            (WithoutLast::Bar, BAR),
            (WithoutLast::Baz, BAZ),
        ];

        for &(variant, expected) in inputs.iter() {
            assert_eq!(variant.to_raw(), expected);
        }
    }

    #[test]
    fn test_enum_with_last_from_raw() {
        let inputs = [
            (FOO, WithLast::Foo),
            (BAR, WithLast::Bar),
            (BAZ, WithLast::Baz),
            (10, WithLast::Last),
        ];

        for &(raw, expected) in inputs.iter() {
            assert_eq!(WithLast::from_raw(raw), expected);
        }
    }

    #[test]
    fn test_enum_with_last_to_raw() {
        let inputs = [
            (WithLast::Foo, FOO),
            (WithLast::Bar, BAR),
            (WithLast::Baz, BAZ),
            (WithLast::Last, FOO),
        ];

        for &(variant, expected) in inputs.iter() {
            assert_eq!(variant.to_raw(), expected);
        }
    }
}
