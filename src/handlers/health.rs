use actix_web::web::Json;
use crate::errors::ApiError;
use crate::helpers::respond_json;

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct HealthResponse {
    pub status: String,
}

// Handler to get liveliness of the service
pub async fn get_health() -> Result<Json<HealthResponse>, ApiError>{
    respond_json(HealthResponse{
        status: "ok".into(),
    })
}