/// This thing again, typed string wrappers

#[macro_export]
macro_rules! def_name_type {
    {$name:tt} => {

        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct $name(String);

        impl std::ops::Deref for $name {
            type Target = String;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }
        }

        // and some application-specific part:
        impl From<&str> for $name {
            fn from(s: &str) -> Self {
                let mut s1 = String::with_capacity(s.len());
                for c in s.chars() {
                    if c != '™' {
                        s1.push(c);
                    }
                }
                Self(s1)
            }
        }

    }
}

