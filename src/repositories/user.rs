use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: i32,
    pub email: String,
    pub display_name: String,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

// Only used by /auth/bootstrap-admin, which only works while the
// users table is empty -- there's no role field here deliberately,
// the bootstrapped account is always "admin".
#[derive(Debug, Deserialize)]
pub struct BootstrapAdminRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}