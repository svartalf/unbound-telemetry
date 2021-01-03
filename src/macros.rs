/// Your usual enum_primitive macro.
#[macro_export]
macro_rules! super_enum {
    (
        enum $name:ident {
            $($variant:ident => ($int:expr, $str:expr),)*
        }
    ) => {
        #[derive(Debug, PartialEq, Eq, Hash)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
//            pub fn as_str(&self) -> &'static str {
//                match self {
//                    $($name::$variant => $str,)*
//                }
//            }
//
//            pub fn as_u64(&self) -> u64 {
//                match self {
//                    $($name::$variant => $int,)*
//                }
//            }
        }

        impl std::str::FromStr for $name {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match () {
                    $(
                        _ if s.eq_ignore_ascii_case(stringify!($variant)) => Ok($name::$variant),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}
