use crate::{prelude::*, error::IamResult};
pub trait ScopedResource {
    fn scope(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Scope(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Resource(String);

pub trait UserAccess {
    fn ensure_access_on_resource(&self, scope: Scope, resource: Resource) -> IamResult<&Self>;
}