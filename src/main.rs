use actix_web::{error, get, web::Query, App, HttpResponse, Responder};
use bytes::Bytes;
use std::{future::Future, pin::Pin};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| App::new().service(speech))
        .bind("0.0.0.0:1234")?
        .run()
        .await
}
#[derive(serde::Deserialize)]
struct Q {
    pub q: String,
}
#[get("/")]
async fn speech(word: Query<Q>) -> impl Responder {
    reqwest::get(&google_translate_tts::url(&word.q, "tr"))
        .await
        .unwrap()
        .bytes()
        .await
        .map_err(|_| error::ErrorImATeapot("error"))
        .map(AudioResponder)
}
struct AudioResponder(Bytes);
impl Responder for AudioResponder {
    type Error = error::Error;
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Self::Error>>>>;
    fn respond_to(self, _: &actix_web::HttpRequest) -> Self::Future {
        Box::pin(async move { Ok(HttpResponse::Ok().content_type("audio/mpeg").body(self.0)) })
    }
}
