use crate::prelude::*;

macro_rules! typed_sql_wrapper {
    ($name:ident, $type:ty) => {
        pub type $name = crate::sqlx_extensions::TypedSqlWrapper<$type>;
    };
}
pub(crate) use typed_sql_wrapper;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Type, Eq, PartialEq)]
#[sqlx(transparent)]
#[serde(transparent)]
pub struct TypedSqlWrapper<T>(pub T);

impl<T> From<T> for TypedSqlWrapper<T> {
    fn from(t: T) -> Self {
        Self(t)
    }
}

impl<T> AsRef<TypedSqlWrapper<T>> for TypedSqlWrapper<T> {
    fn as_ref(&self) -> &TypedSqlWrapper<T> {
        self
    }
}

impl<T> AsMut<TypedSqlWrapper<T>> for TypedSqlWrapper<T> {
    fn as_mut(&mut self) -> &mut TypedSqlWrapper<T> {
        self
    }
}