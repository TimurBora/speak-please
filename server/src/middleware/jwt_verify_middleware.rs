use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use shared::utils::jwt::verify_access_token;

pub async fn check_access_token(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = auth.token();
    let verify_result = verify_access_token(token);

    match verify_result {
        Ok(_) => Ok(next.run(request).await),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;
    use axum::{Router, middleware, routing::get};
    use axum_test::TestServer;
    use shared::utils::jwt::create_access_token;

    async fn create_app() -> Router {
        let test_route = get(|| async { "Success" });

        Router::new()
            .route("/protected", test_route)
            .layer(middleware::from_fn(check_access_token))
    }

    #[tokio::test]
    async fn test_check_access_token_valid() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();

        let valid_token =
            create_access_token("user_123".to_string(), "test@example.com".to_string())
                .expect("Cannot create a valid token");

        let response = server
            .get("/protected")
            .add_header(
                axum::http::header::AUTHORIZATION,
                format!("Bearer {}", valid_token),
            )
            .await;

        response.assert_status(StatusCode::OK);
        assert_eq!(response.text(), "Success");
    }

    #[tokio::test]
    async fn test_check_access_token_missing_header() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server.get("/protected").await;

        response.assert_status(StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_check_access_token_invalid_token() {
        let app = create_app().await;
        let server = TestServer::new(app).unwrap();

        let response = server
            .get("/protected")
            .add_header(
                axum::http::header::AUTHORIZATION,
                "Bearer invalid-token-string",
            )
            .await;

        response.assert_status(StatusCode::UNAUTHORIZED);
    }
}
