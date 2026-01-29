use aws_config::Region;
use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;

#[derive(Clone)]
pub struct S3Manager {
    client: Client,
    bucket: String,
}

impl S3Manager {
    pub async fn new(bucket_name: String, endpoint: String, region: String) -> Self {
        let sdk_config = aws_config::load_from_env().await;

        let s3_config = aws_sdk_s3::config::Builder::from(&sdk_config)
            .endpoint_url(endpoint)
            .region(Region::new(region))
            .force_path_style(true)
            .build();

        let client = Client::from_conf(s3_config);

        Self {
            client,
            bucket: bucket_name,
        }
    }

    pub async fn get_upload_url(
        &self,
        key: &str,
        content_type: &str,
        expires_in: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let expires_in = Duration::from_secs(expires_in);
        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .content_type(content_type)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    pub async fn get_download_url(
        &self,
        key: &str,
        expires_in: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let expires_in = Duration::from_secs(expires_in);
        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    pub async fn delete_file(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .delete_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await?;
        Ok(())
    }

    pub fn generate_proof_key(
        user_id: &str,
        proof_id: &str,
        index: u32,
        extension: &str,
        is_voice: bool,
    ) -> String {
        let folder = if is_voice { "audio" } else { "photos" };
        let prefix = if is_voice { "voice" } else { "photo" };

        format!(
            "users/{}/proofs/{}/{}/{}_{}.{}",
            user_id, proof_id, folder, prefix, index, extension
        )
    }

    pub async fn resolve_urls(&self, keys_json: Option<&serde_json::Value>) -> Vec<String> {
        let mut urls = Vec::new();
        if let Some(keys) = keys_json.and_then(|v| v.as_array()) {
            for key in keys.iter().filter_map(|k| k.as_str()) {
                if let Ok(url) = self.get_download_url(key, 3600).await {
                    urls.push(url);
                }
            }
        }
        urls
    }
}
