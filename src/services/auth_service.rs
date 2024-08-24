use sea_orm::ActiveModelTrait;

use crate::{
    commons, requests,
    responses::{self},
};

impl super::AuthService {
    pub fn new(db_conn: sea_orm::DatabaseConnection, jwt: commons::jwt::JWT) -> Self {
        Self { db_conn, jwt }
    }

    pub async fn register(
        &self,
        request: requests::RegisterRequest,
    ) -> Result<responses::auth::Auth, commons::Error> {
        let new_user = entities::users::ActiveModel {
            email: sea_orm::Set(request.email),
            password: sea_orm::Set(request.password),
            updated_at: sea_orm::NotSet,
            deleted_at: sea_orm::NotSet,
            ..Default::default()
        }
        .insert(&self.db_conn)
        .await
        .map_err(|err| commons::Error {
            status: 500,
            cause: err.to_string(),
            message: String::from("failed to register user"),
        })?;

        self.generate_auth(responses::auth::User {
            email: new_user.email,
            id: new_user.id,
        })
    }

    fn generate_auth(
        &self,
        user: responses::auth::User,
    ) -> Result<responses::auth::Auth, commons::Error> {
        let token = self
            .jwt
            .clone()
            .generate_auth_token(user.clone())
            .map_err(|err| commons::Error {
                cause: err.to_string(),
                message: String::from("failed generate auth token"),
                status: 500,
            })?;

        let refresh_token = self
            .jwt
            .clone()
            .generate_refresh_token(user.clone())
            .map_err(|err| commons::Error {
                cause: err.to_string(),
                message: String::from("failed generate refresh token"),
                status: 500,
            })?;

        Ok(responses::auth::Auth {
            refresh_token,
            token,
        })
    }
}
