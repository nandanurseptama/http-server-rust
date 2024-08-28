use sea_orm::DatabaseConnection;

use crate::commons;

mod auth_service;

#[derive(Debug, Clone)]
/// Handle auth logic
pub struct AuthService {
    db_conn: DatabaseConnection,
    jwt: commons::jwt::JWT,
    aes: commons::aes::Aes,
}
