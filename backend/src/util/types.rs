use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UserRow {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub password_hash: String,
    pub is_active: bool,
    pub is_admin: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct PublicUser {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: PublicUser,
    pub expires_at: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct AdWithSeller {
    pub id: String,
    pub seller_id: String,
    pub seller_display_name: String,
    pub title: String,
    pub description: String,
    pub price: i64,
    pub category: String,
    pub location: String,
    pub image_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct ComplaintRow {
    pub id: String,
    pub reporter_id: String,
    pub ad_id: Option<String>,
    pub reason: String,
    pub details: Option<String>,
    pub status: String,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAdRequest {
    pub title: String,
    pub description: String,
    pub price: i64,
    pub category: String,
    pub location: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct UpdateAdRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<i64>,
    pub category: Option<String>,
    pub location: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub display_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub current_password: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateComplaintRequest {
    pub ad_id: String,
    pub reason: String,
    pub details: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateComplaintRequest {
    pub status: String,
}

pub fn user_from_sql(
    id: String,
    email: String,
    display_name: String,
    password_hash: String,
    is_active: i32,
    is_admin: i32,
    created_at: String,
    updated_at: String,
) -> UserRow {
    UserRow {
        id,
        email,
        display_name,
        password_hash,
        is_active: is_active != 0,
        is_admin: is_admin != 0,
        created_at,
        updated_at,
    }
}

pub fn to_public_user(u: &UserRow) -> PublicUser {
    PublicUser {
        id: u.id.clone(),
        email: u.email.clone(),
        display_name: u.display_name.clone(),
        is_admin: u.is_admin,
    }
}

pub fn is_admin_active(u: &UserRow) -> bool {
    u.is_admin && u.is_active
}
