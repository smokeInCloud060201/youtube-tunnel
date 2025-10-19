use actix_web::{get, web, HttpResponse};
use service::video_search::VideoSearchService;
use shared::model::video_search;
use std::sync::Arc;
use tracing::debug;

#[get("/v1/search")]
async fn get_video(
    query: web::Query<video_search::QueryParams>,
    service: web::Data<Arc<VideoSearchService>>,
) -> actix_web::Result<HttpResponse> {
    let query_params = query.into_inner();

    debug!("Query: {:?}", query_params);

    let response = service
        .search(query_params)
        .await
        .expect("Error searching video");

    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(response))
}
