use sqlx::types::Uuid;

pub struct PrivateUserViewRow {
    pub uid: Uuid,
    pub email: String,
    pub name: String,
    pub nickname: String,
    pub email_verified: bool,
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}