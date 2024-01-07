use crate::prelude::*;

macro_rules! typed_sql_wrapper {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Type, Eq, PartialEq)]
        #[sqlx(transparent)]
        #[serde(transparent)]
        pub struct $name(pub $type);

        impl AsRef<$type> for $name {
            fn as_ref(&self) -> &$type {
                &self.0
            }
        }

        impl From<$type> for $name {
            fn from(value: $type) -> Self {
                Self(value)
            }
        }
    };
}
pub(crate) use typed_sql_wrapper;

