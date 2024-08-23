use sea_orm::DatabaseConnection;

mod user_service;
pub struct UserService {
    conn: DatabaseConnection,
}
