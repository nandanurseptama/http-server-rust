use sea_orm::DatabaseConnection;

use super::UserService;

impl UserService {
    pub fn new(conn: DatabaseConnection) -> Self {
        Self { conn }
    }

    pub fn get_user() {}
}
