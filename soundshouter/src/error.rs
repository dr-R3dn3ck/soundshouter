use rocket::http::Status;
use rocket::response::Responder;
use rocket::{response, Request};

pub enum AppError {
    DBError(String),
    QueueError(String),
    NotFoundError(String),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        match self {
            AppError::DBError(_) | AppError::QueueError(_) => Status::InternalServerError.respond_to(req),
            AppError::NotFoundError(_) => Status::NotFound.respond_to(req),
        }
    }
}