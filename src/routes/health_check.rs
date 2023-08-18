use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod tests {
    // use crate::health_check;
    use crate::routes::health_check;

    #[tokio::test]
    async fn health_check_succeeds() {
        let res = health_check().await;
        assert!(res.status().is_success())
    }
}