use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};

use crate::{
    commons, requests,
    responses::{self},
};

impl super::AuthService {
    pub fn new(
        db_conn: sea_orm::DatabaseConnection,
        jwt: commons::jwt::JWT,
        aes: commons::aes::Aes,
    ) -> Self {
        Self { db_conn, jwt, aes }
    }

    pub async fn register(
        &self,
        request: requests::RegisterRequest,
    ) -> Result<responses::auth::Auth, commons::Error> {
        let cipher_password = self
            .aes
            .clone()
            .encrypt(request.password)
            .map_err(|err| err)?;
        let new_user = entities::users::ActiveModel {
            email: sea_orm::Set(request.email.to_lowercase()),
            password: sea_orm::Set(cipher_password),
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

    pub async fn login(
        &self,
        request: requests::RegisterRequest,
    ) -> Result<responses::auth::Auth, commons::Error> {
        let opt_user = entities::users::Entity::find()
            .filter(
                sea_orm::Condition::all()
                    .add(entities::users::Column::Email.eq(request.email.to_lowercase())),
            )
            .one(&self.db_conn)
            .await
            .map_err(|err| commons::Error {
                status: 500,
                cause: err.to_string(),
                message: String::from("failed to login user"),
            })?;

        if opt_user.is_none() {
            return Err(commons::Error {
                status: 404,
                cause: String::from("user not found"),
                message: String::from("credential not match"),
            });
        }
        let user = opt_user.unwrap();

        let plain_password = self.aes.clone().decrypt(user.password).map_err(|err| err)?;

        if plain_password != request.password {
            return Err(commons::Error {
                status: 400,
                cause: String::from("password not match"),
                message: String::from("credential not match"),
            });
        }

        self.generate_auth(responses::auth::User {
            email: user.email,
            id: user.id,
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
