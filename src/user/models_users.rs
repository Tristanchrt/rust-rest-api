use diesel::prelude::*;

#[derive(serde::Serialize, Queryable, Selectable)]
#[diesel(table_name = crate::user::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = crate::user::schema::users)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}
