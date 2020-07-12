use actix_web::HttpResponse;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("It works!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http;

    #[actix_rt::test]
    async fn health_check_should_be_works() {
        let resp = health_check().await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
}
