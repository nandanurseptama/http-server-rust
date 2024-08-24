use sea_orm::DatabaseConnection;
use serde::Serialize;

mod auth_service;

/// Handle auth logic
pub struct AuthService {
    db_conn: DatabaseConnection,
}

#[derive(Debug, Serialize)]
pub struct Result<T: Serialize> {
    data: Option<T>,
    status: u16,
    error: Option<String>,
    message: String,
}

impl<T: Serialize> Result<T> {
    pub fn ok(data: Option<T>) -> Self {
        Self {
            data,
            status: 200,
            error: Option::None,
            message: String::from(""),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            status: 500,
            data: Option::None,
            error: Option::Some(error),
            message: String::from("error"),
        }
    }

    pub fn set_status(self, status: u16) -> Self {
        Self {
            status,
            data: self.data,
            error: self.error,
            message: self.message,
        }
    }

    pub fn set_message(self, message: String) -> Self {
        Self {
            message,
            status: self.status,
            data: self.data,
            error: self.error,
        }
    }

    pub fn get_status(self) -> u16 {
        return self.status;
    }
}
