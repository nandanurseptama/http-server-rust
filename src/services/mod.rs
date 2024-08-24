use sea_orm::DatabaseConnection;

use crate::commons;

mod auth_service;

/// Handle auth logic
pub struct AuthService {
    db_conn: DatabaseConnection,
    jwt: commons::jwt::JWT,
}