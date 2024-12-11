use crate::database::models::Id;


#[derive(sqlx::FromRow, Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Id,
    // --------------
    pub email: String,
    pub name: String,
    pub surname: String,
    pub password_hash: String,
    pub password_salt: String,
}

