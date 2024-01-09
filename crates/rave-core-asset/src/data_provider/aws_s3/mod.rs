use super::{DataProvider, DataProviderResultFuture, GetUrl};
use crate::data_provider::PutUrl;
use crate::prelude::*;
use aws_config::{meta::credentials::CredentialsProviderChain, Region};
use aws_sdk_s3::{presigning::PresigningConfig, Client};
use std::time::{Duration, SystemTime};

pub const AWS_S3_DATA_PROVIDER: super::DataProviderUid = DataProviderUid(0x0000_0001);

#[derive(Debug)]
pub struct AwsS3DataProvider {
    bucket: String,
    region: Region,
    client: Client,
    put_url_expiration: Duration,
    get_url_expiration: Duration,
}

impl AwsS3DataProvider {
    #[instrument(ret)]
    pub async fn init_from_environ(bucket: &str, region: &str) -> Self {
        let client: aws_sdk_s3::Client = aws_sdk_s3::Client::new(
            &aws_config::from_env()
                .credentials_provider(
                    CredentialsProviderChain::first_try(
                        "Environment",
                        aws_config::environment::credentials::EnvironmentVariableCredentialsProvider::new(),
                    )
                    .or_else(
                        "Profile",
                        aws_config::profile::ProfileFileCredentialsProvider::builder().build(),
                    ),
                )
                .load()
                .await
        );
        Self {
            put_url_expiration: Duration::from_secs(60),
            get_url_expiration: Duration::from_secs(60 * 60),
            bucket: bucket.to_string(),
            region: Region::new(region.to_owned()),
            client,
        }
    }
}

impl DataProvider for AwsS3DataProvider {
    fn uid(&self) -> super::DataProviderUid {
        AWS_S3_DATA_PROVIDER
    }

    fn supported_kinds(&self) -> Vec<AssetKind> {
        vec![AssetKind::Image, AssetKind::Video, AssetKind::Audio]
    }

    fn prepare_get(&self, identifier: AssetUid) -> DataProviderResultFuture<GetUrl> {
        let request = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(identifier.0.to_string())
            .presigned(
                PresigningConfig::builder()
                    .expires_in(self.put_url_expiration)
                    .build()
                    .unwrap(),
            );
        Box::new(async move {
            let result = request
                .await
                .map_err(|e| AssetError::ProviderSpecific(Box::new(e)))?;
            Ok(GetUrl {
                url: result.uri().to_owned(),
                custom_headers: result
                    .headers()
                    .map(|(a, b)| (a.to_owned(), b.to_owned()))
                    .collect(),
                expires_at: Some(DateTime::from(SystemTime::now() + self.get_url_expiration)),
            })
        })
    }

    fn prepare_put(&self, identifier: AssetUid) -> DataProviderResultFuture<PutUrl> {
        let request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(identifier.0.to_string())
            .presigned(
                PresigningConfig::builder()
                    .expires_in(self.put_url_expiration)
                    .build()
                    .unwrap(),
            );
        Box::new(async move {
            let result = request
                .await
                .map_err(|e| AssetError::ProviderSpecific(Box::new(e)))?;
            Ok(PutUrl {
                url: result.uri().to_owned(),
                custom_headers: result
                    .headers()
                    .map(|(a, b)| (a.to_owned(), b.to_owned()))
                    .collect(),
                expires_at: Some(DateTime::from(SystemTime::now() + self.put_url_expiration)),
            })
        })
    }

    fn perform_del(
        &self,
        identifier: rave_core_database::prelude::AssetUid,
    ) -> DataProviderResultFuture<()> {
        let request = self
            .client
            .delete_object()
            .bucket(&self.bucket)
            .key(identifier.0.to_string())
            .send();
        Box::new(async move {
            let _ = request
                .await
                .map_err(|e| AssetError::ProviderSpecific(Box::new(e)))?;
            Ok(())
        })
    }
}
