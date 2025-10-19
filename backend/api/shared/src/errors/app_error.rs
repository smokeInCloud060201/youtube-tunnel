use actix_web::http::StatusCode;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error_code: u16,
    pub error_message: String,
    pub time_stamp: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("The requested item was not found.")]
    NotFound,
    #[error("You are forbidden to access this resource.")]
    Forbidden,
    // This allows wrapping other errors
    #[error("An unexpected error occurred.")]
    InternalError(String),
}

impl AppError {
    // Helper function to get the status code
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    // Helper function to get a machine-readable error type
    pub fn error_type(&self) -> String {
        match self {
            AppError::NotFound => "not_found".to_string(),
            AppError::Forbidden => "forbidden".to_string(),
            AppError::InternalError(_) => "internal_server_error".to_string(),
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::InternalError(err.to_string())
    }
}

// pub struct ErrorHandlingMiddleware;
//
// // `Transform` trait implementation for the middleware
// impl<S, B> Transform<S, ServiceRequest> for ErrorHandlingMiddleware
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type InitError = ();
//     type Transform = ErrorHandlingMiddlewareService<S>;
//     type Future = Ready<Result<Self::Transform, Self::InitError>>;
//
//     fn new_transform(&self, service: S) -> Self::Future {
//         ready(Ok(ErrorHandlingMiddlewareService { service }))
//     }
// }
//
// // The actual middleware service
// pub struct ErrorHandlingMiddlewareService<S> {
//     service: S,
// }
//
// impl<S, B> Service<ServiceRequest> for ErrorHandlingMiddlewareService<S>
// where
//     S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
//     S::Future: 'static,
//     B: 'static,
// {
//     type Response = ServiceResponse<B>;
//     type Error = Error;
//     type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;
//
//     dev::forward_ready!(service);
//
//     fn call(&self, req: ServiceRequest) -> Self::Future {
//         let fut = self.service.call(req);
//
//         Box::pin(async move {
//             // Await the response from the inner service
//             let res = fut.await;
//
//             match res {
//                 // If the handler was successful, just pass the response through
//                 Ok(response) => Ok(response),
//
//                 // If the handler returned an error, intercept it
//                 Err(error) => {
//                     // Log the original error for debugging purposes
//                     error!("An error occurred: {:?}", error);
//
//                     // Attempt to downcast the error to our specific AppError type
//                     // This allows us to handle our custom errors differently if needed
//                     if let Some(app_error) = error.as_error::<AppError>() {
//                         // This is one of our defined application errors
//                         let status_code = app_error.status_code();
//                         let error_response = ErrorResponse {
//                             error_code: status_code.as_u16(),
//                             time_stamp: app_error.error_type(),
//                             error_message: app_error.to_string(),
//                         };
//                         // Create a new HttpResponse with our custom format
//                         let http_response = HttpResponse::build(status_code).json(error_response);
//                         // Convert it into a ServiceResponse
//                         Ok(req.into_response(http_response))
//                     } else {
//                         // This is an unexpected error (e.g., from a library, or a panic)
//                         // Return a generic 500 internal server error
//                         let error_response = ErrorResponse {
//                             error_code: 500,
//                             time_stamp: "internal_server_error".to_string(),
//                             error_message: "An unexpected internal server error occurred.".to_string(),
//                         };
//                         let http_response = HttpResponse::InternalServerError().json(error_response);
//                         Ok(req.into_response(http_response))
//                     }
//                 }
//             }
//         })
//     }
// }
//
