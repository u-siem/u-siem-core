#[macro_export]
macro_rules! impl_parse_str {
    ($S:ident) => {
        impl From<&str> for $S {
            fn from(value: &str) -> Self {
                Self(value.to_string())
            }
        }
        
        impl From<&String> for $S {
            fn from(value: &String) -> Self {
                Self(value.clone())
            }
        }
        impl From<String> for $S {
            fn from(value: String) -> Self {
                Self(value)
            }
        }
        impl From<$S> for $crate::prelude::LogString {
            fn from(v: $S) -> Self {
                $crate::prelude::LogString::Owned(v.0)
            }
        }
        impl From<&$S> for $crate::prelude::LogString {
            fn from(v: &$S) -> Self {
                $crate::prelude::LogString::Owned(v.0.to_string())
            }
        }
        impl<'a> Into<&'a str> for &'a $S {
            fn into(self) -> &'a str {
                &self.0[..]
            }
        }
        
        impl PartialEq<$S> for str {
            fn eq(&self, other: &$S) -> bool {
                self == &other.0[..]
            }
        }
        
        impl $S {
            pub fn contains(&self, txt : &str) -> bool {
                self.0.contains(txt)
            }
            pub fn contains_str(&self, txt : &str) -> bool {
                self.0.contains(txt)
            }
            pub fn eq_ignore_ascii_case(&self, txt : &str) -> bool {
                self.0.eq_ignore_ascii_case(txt)
            }
        }
    };
}