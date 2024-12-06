use crate::claims::Claims;
/// Authentication functionalities
use rocket::http::Status;
use rocket::post;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

/// Login request object
#[derive(Deserialize)]
pub struct LoginRequest {
    user: String,
    password: String,
}

/// Successful response
#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

/// User authentication, Successful authentication returns a JWT
#[post("/login", data = "<req>")]
pub fn login(req: Json<LoginRequest>) -> Result<Json<LoginResponse>, Custom<String>> {
    //TODO: use the database
    if req.user != "test" || req.password != "prueba123" {
        return Err(Custom(Status::Unauthorized, "Missing account".to_string()));
    }

    let claim = Claims::from_name(&req.user);
    let response = LoginResponse {
        token: claim.into_token()?,
    };

    Ok(Json(response))
}
