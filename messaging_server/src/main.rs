use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use email::send_bulk_email_sesv2;
use email::sesv2::model::SendBulkEmailRequestDTO;

#[get("/")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

async fn sesv2_sedbulk(data: web::Json<SendBulkEmailRequestDTO>) -> HttpResponse {
    let data = data.into_inner();
    let result = send_bulk_email_sesv2(
        data.to_email_and_content,
        data.from_email,
        data.content_subject,
        data.content_html,
    )
    .await;

    let response = match result {
        Ok(success_message) => HttpResponse::Ok().body(success_message),
        Err(error_message) => {
            HttpResponse::BadRequest().body(format!("AWS SES SDKV2 ERROR: {}", error_message))
        }
    };
    return response;
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
            .route("/email/sesv2/send_bulk", web::post().to(sesv2_sedbulk))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
