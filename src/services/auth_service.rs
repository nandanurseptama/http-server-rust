use sea_orm::ActiveModelTrait;

use crate::requests;

impl super::AuthService {
    pub fn new(db_conn: sea_orm::DatabaseConnection) -> Self {
        Self { db_conn }
    }

    pub async fn register(&self, request: requests::RegisterRequest) -> super::Result<String> {
        let result = entities::users::ActiveModel {
            email: sea_orm::Set(request.email),
            password: sea_orm::Set(request.password),
            updated_at: sea_orm::NotSet,
            deleted_at: sea_orm::NotSet,
            ..Default::default()
        }
        .insert(&self.db_conn)
        .await;

        return match result {
            Ok(option) => super::Result::ok(Option::Some(option.id.to_string()))
                .set_status(201)
                .set_message(String::from("OK")),
            Err(err) => super::Result::error(err.to_string())
                .set_message(String::from("failed to register user"))
                .set_status(500),
        };
    }
}
