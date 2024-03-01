use actix_web::{http::header::ContentType, HttpResponse, ResponseError};
use derive_more::Display;
use reqwest::StatusCode;

#[derive(Debug, Display)]
pub enum PizzaError {
    NoPizzasFound,
    PizzaCreationFailure,
    NoSuchPizzaFound,
}

impl ResponseError for PizzaError {
    fn status_code(&self) -> StatusCode {
        match self {
            PizzaError::NoPizzasFound => StatusCode::NO_CONTENT,
            PizzaError::PizzaCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            PizzaError::NoSuchPizzaFound => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}
