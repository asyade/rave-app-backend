use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetOptions {
    #[cfg(feature = "aws-s3-data-provider")]
    pub aws_s3_bucket_name: String,
    #[cfg(feature = "aws-s3-data-provider")]
    pub aws_s3_bucket_region: String,
}
