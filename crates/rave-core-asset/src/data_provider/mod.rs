use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::sync::Arc;
use tokio::sync::RwLock;

use rave_core_database::tables::asset::AssetKind;
use crate::prelude::*;

#[cfg(feature = "aws-s3-data-provider")]
pub mod aws_s3;

pub struct DataProviderManager {
    providers: HashMap<DataProviderUid, DataProviderHandle>,
}

#[derive(Debug, Clone)]
pub struct DataProviderHandle {
    supported_kinds: HashSet<AssetKind>,
    provider: Arc<RwLock<Box<dyn DataProvider>>>,
}

/// Represent a public URL to access an asset data from the frontend.
/// The url must be callable with an http `GET` query before `expires_at`.
pub struct GetUrl {
    /// The http url on which to perform an http `GET` query.
    pub url: String,
    /// The custom headers to add to the request.
    pub custom_headers: HashMap<String, String>,
    /// Expiration date of the url.
    pub expires_at: Option<DateTime>,
}

/// Represent a public URL to upload an asset data from the frontend.
/// The url must be callable with an http `PUT` query before `expires_at`.
pub struct PutUrl {
    /// The http url on which to perform an http `PUT` query.
    pub url: String,
    /// The custom headers to add to the request.
    pub custom_headers: HashMap<String, String>,
    /// Expiration date of the url.
    pub expires_at: Option<DateTime>,
}

pub type DataProviderResultFuture<T> =
    Box<dyn 'static + Future<Output = AssetResult<T>>>;

pub trait DataProvider: Send + Sync + Debug + 'static {
    fn uid(&self) -> DataProviderUid;

    fn supported_kinds(&self) -> Vec<AssetKind>;

    fn prepare_get(&self, identifier: AssetUid) -> DataProviderResultFuture<GetUrl>;

    fn prepare_put(&self, identifier: AssetUid) -> DataProviderResultFuture<PutUrl>;

    fn perform_del(&self, identifier: AssetUid) -> DataProviderResultFuture<()>;
}

impl DataProviderManager {
    pub fn new() -> Self {
        Self {
            providers: HashMap::new(),
        }
    }

    pub fn register_provider<T: DataProvider>(&mut self, provider: T) {
        let uid = provider.uid();
        let supported_kinds = provider
            .supported_kinds()
            .into_iter()
            .collect::<HashSet<_>>();
        self.providers.insert(
            uid,
            DataProviderHandle {
                supported_kinds,
                provider: Arc::new(RwLock::new(Box::new(provider))),
            },
        );
    }

    pub fn find_provider_for_kind(&self, kind: AssetKind) -> AssetResult<&DataProviderHandle> {
        self.providers
            .values()
            .find(|provider| provider.supported_kinds.contains(&kind))
            .ok_or(AssetError::NoProviderFoundForAssetKind(kind))
    }
}
