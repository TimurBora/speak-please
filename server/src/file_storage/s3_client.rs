use aws_sdk_s3::Client;
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;

#[derive(Clone)]
pub struct S3Manager {
    client: Client,
    bucket: String,
}

impl S3Manager {
    pub async fn new(bucket_name: String) -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self {
            client,
            bucket: bucket_name,
        }
    }

    /// Генерирует Presigned URL для загрузки файла (PUT)
    /// Позволяет фронтенду загружать файл напрямую в S3, минуя твой сервер
    pub async fn get_upload_url(
        &self,
        key: &str,
        expires_in: u64,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let expires_in = Duration::from_secs(expires_in);
        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .presigned(PresigningConfig::expires_in(expires_in)?)
            .await?;

        Ok(presigned_request.uri().to_string())
    }

    /// Генерирует Presigned URL для скачивания/просмотра (GET)
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

    /// Удаление объекта
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
        file_name: &str,
        is_voice: bool,
    ) -> String {
        let folder = if is_voice { "audio" } else { "photos" };
        format!(
            "users/{}/proofs/{}/{}/{}",
            user_id, proof_id, folder, file_name
        )
    }
}
