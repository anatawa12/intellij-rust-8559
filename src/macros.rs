
macro_rules! str_enum {
    (
        $enum_access: vis enum $enum_name: ident {
            $error_access: vis type Err = $error_name: ident;
            $( $variant_name: ident ( $variant_str: literal ) ),* $(,)?
        }
    ) => {
        #[derive(::core::cmp::Eq, ::core::cmp::PartialEq)]
        $enum_access enum $enum_name {
            $($variant_name,)*
        }

        impl ::core::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                match self {
                    $(Self::$variant_name => f.write_str($variant_str),)*
                }
            }
        }

        impl ::core::fmt::Debug for $enum_name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                ::core::fmt::Display::fmt(self, f)
            }
        }

        impl ::core::str::FromStr for $enum_name {
            type Err = $error_name;

            fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
                match s {
                    $($variant_str => Ok(Self::$variant_name),)*
                    other => Err($error_name(other.to_owned()))
                }
            }
        }

        #[derive(::core::fmt::Debug)]
        $error_access struct $error_name(String);
    };
}
