use std::fmt::Display;
use actix_web::{App, HttpServer, ResponseError};
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::Json;
use apistos::actix::CreatedJson;
use apistos::api_operation;
use apistos::ApiComponent;
use apistos::ApiErrorComponent;
use apistos::app::OpenApiWrapper;
use apistos::spec::Spec;
use apistos::web::{post, resource, scope};
use apistos_models::info::Info;
use core::fmt::Formatter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, JsonSchema, Debug, Clone)]
enum Xenum {
    Axenum,
    Bxenum { name: String, age: u64 },
}

#[derive(Serialize, Deserialize, Debug, Clone, JsonSchema, ApiComponent)]
pub struct Test {
  pub test: String,
  x: Xenum,
}

#[derive(Serialize, Deserialize, Debug, Clone, ApiErrorComponent)]
#[openapi_error(
  status(code = 403),
  status(code = 404),
  status(code = 405, description = "Invalid input"),
  status(code = 409)
)]
pub enum ErrorResponse {
  MethodNotAllowed(String),
  NotFound(String),
  Conflict(String),
  Unauthorized(String),
}

impl Display for ErrorResponse {
  fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
    todo!()
  }
}

impl ResponseError for ErrorResponse {
  fn status_code(&self) -> StatusCode {
    todo!()
  }
}

#[api_operation(
  tag = "pet",
  summary = "Add a new pet to the store",
  description = r###"Add a new pet to the store
    Plop"###,
  error_code = 405
)]
pub(crate) async fn test(
  body: Json<Test>,
) -> Result<CreatedJson<Test>, ErrorResponse> {
  Ok(CreatedJson(body.0))
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  HttpServer::new(move || {
    let spec = Spec {
      info: Info {
        title: "An API".to_string(),
        version: "1.0.0".to_string(),
        ..Default::default()
      },
      ..Default::default()
    };

    App::new()
        .document(spec)
        .wrap(Logger::default())
        .service(scope("/test")
            .service(
              resource("")
                  .route(post().to(test))
            )
        )
        .build("/openapi.json")
  })
      .bind((Ipv4Addr::UNSPECIFIED, 8080))?
      .run()
      .await
}