macro_rules! impl_enum {
    (enum: $type:ty, raw: $raw:ty, match: { $($match_arms:tt)* }) => {
        impl std::fmt::Display for $type {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                $crate::enumutil::impl_enum_display!(self, f, $($match_arms)*)
            }
        }

        impl $type {
            /// Converts libvirt C enum constant to Rust enum.
            pub fn from_raw(raw: $raw) -> Self {
                $crate::enumutil::impl_enum_from!(raw, $($match_arms)*)
            }

            /// Converts Rust enum to libvirt C enum constant.
            pub fn to_raw(self) -> $raw {
                $crate::enumutil::impl_enum_to!(self, $($match_arms)*)
            }
        }
    };
}

macro_rules! impl_enum_display {
    (@acc ($e:expr, $f:expr, _ => $type:ident => $_raw:path,) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_display!(@final ($e) -> ($($body)* Self::$type => write!($f, "{}", stringify!($type).to_lowercase()),))
    };
    (@acc ($e:expr, $f:expr, _ => $type:ident,) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_display!(@final ($e) -> ($($body)* ))
    };
    (@acc ($e:expr, $f:expr, $(#[$attr:meta])* $raw:path => $type:ident, $($match_arms:tt)*) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_display!(@acc ($e, $f, $($match_arms)*) -> ($($body)* $(#[$attr])* Self::$type => write!($f, "{}", stringify!($type).to_lowercase()),))
    };
    (@final ($e:expr) -> ($($body:tt)*)) => {
        match $e { $($body)* }
    };
    ($e:expr, $f:expr, $($match_arms:tt)*) => {
        $crate::enumutil::impl_enum_display!(@acc ($e, $f, $($match_arms)*) -> ())
    };
}

macro_rules! impl_enum_from {
    (@acc ($e:expr, _ => $type:ident => $_raw:path,) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_from!(@final ($e) -> ($($body)* _ => Self::$type,))
    };
    (@acc ($e:expr, _ => $type:ident,) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_from!(@final ($e) -> ($($body)* _ => Self::$type,))
    };
    (@acc ($e:expr, $(#[$attr:meta])* $raw:path => $type:ident, $($match_arms:tt)*) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_from!(@acc ($e, $($match_arms)*) -> ($($body)* $(#[$attr])* $raw => Self::$type,))
    };
    (@final ($e:expr) -> ($($body:tt)*)) => {
        match $e { $($body)* }
    };
    ($e:expr, $($match_arms:tt)*) => {
        $crate::enumutil::impl_enum_from!(@acc ($e, $($match_arms)*) -> ())
    };
}

macro_rules! impl_enum_to {
    (@acc ($e:expr, _ => $type:ident => $raw:path,) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_to!(@final ($e) -> ($($body)* Self::$type => $raw,))
    };
    (@acc ($e:expr, _ => $_type:ident,) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_to!(@final ($e) -> ($($body)*))
    };
    (@acc ($e:expr, $(#[$attr:meta])* $raw:path => $type:ident, $($match_arms:tt)*) -> ($($body:tt)*)) => {
        $crate::enumutil::impl_enum_to!(@acc ($e, $($match_arms)*) -> ($($body)* $(#[$attr])* Self::$type => $raw,))
    };
    (@final ($e:expr) -> ($($body:tt)*)) => {
        match $e { $($body)* }
    };
    ($e:expr, $($match_arms:tt)*) => {
        $crate::enumutil::impl_enum_to!(@acc ($e, $($match_arms)*) -> ())
    };
}

pub(crate) use impl_enum;
pub(crate) use impl_enum_display;
pub(crate) use impl_enum_from;
pub(crate) use impl_enum_to;

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
            FOO => Foo,
            BAR => Bar,
            BAZ => Baz,
            _ => Foo,
        }
    }

    impl_enum! {
        enum: WithLast,
        raw: u32,
        match: {
            FOO => Foo,
            BAR => Bar,
            BAZ => Baz,
            _ => Last => FOO,
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
            (WithoutLast::Foo, FOO, "foo"),
            (WithoutLast::Bar, BAR, "bar"),
            (WithoutLast::Baz, BAZ, "baz"),
        ];

        for &(variant, expected, estr) in inputs.iter() {
            assert_eq!(variant.to_raw(), expected);
            assert_eq!(variant.to_string(), estr);
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
            (WithLast::Foo, FOO, "foo"),
            (WithLast::Bar, BAR, "bar"),
            (WithLast::Baz, BAZ, "baz"),
            (WithLast::Last, FOO, "last"),
        ];

        for &(variant, expected, estr) in inputs.iter() {
            assert_eq!(variant.to_raw(), expected);
            assert_eq!(variant.to_string(), estr);
        }
    }
}
