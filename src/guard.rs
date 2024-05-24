use std::env;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request,
};

pub mod services {
    tonic::include_proto!("services");
}

use services::{auth_service_client::AuthServiceClient, ClaimsRequest};

use crate::typing::ErrorResponse;

#[derive(Debug, PartialEq)]
pub enum Role {
    Admin,
    User,
}

#[derive(Debug)]
pub struct Claims {
    pub user_id: i32,
    pub role: Role,
}

#[derive(Debug)]
pub struct AuthGuard {
    pub claims: Claims,
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthGuard {
    type Error = ErrorResponse;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ErrorResponse> {
        let auth_header = req.headers().get_one("Authorization");
        if auth_header.is_none() {
            return Outcome::Error((
                Status::Unauthorized,
                ErrorResponse {
                    status_code: rocket::http::Status::Unauthorized,
                    message: "Unauthorized",
                },
            ));
        }
        let token = auth_header.unwrap().split_whitespace().last().unwrap();
        let auth_url = env::var("AUTH_SERVICE_URL").expect("AUTH_SERVICE_URL must be set.");
        let mut client = AuthServiceClient::connect(auth_url).await.unwrap();
        let request = tonic::Request::new(ClaimsRequest {
            token: token.to_string(),
        });
        let response = client.get_claims(request).await.unwrap().into_inner();
        let claims = Claims {
            user_id: response.user_id,
            role: match response.role.as_str() {
                "ADMIN" => Role::Admin,
                "USER" => Role::User,
                _ => Role::User,
            },
        };

        Outcome::Success(AuthGuard {
            claims,
            token: token.to_string(),
        })
    }
}
