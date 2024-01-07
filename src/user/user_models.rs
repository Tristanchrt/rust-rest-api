#[derive(serde::Deserialize, Insertable)]
#[diesel(table_name = user_db)]
struct UserDB {
    name: String,
    email: String,
    password: String,
}
