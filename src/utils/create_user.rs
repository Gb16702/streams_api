use sqlx::{PgPool, query};


pub async fn create_user(email: String, password: String, pool: &PgPool) -> () {
    use crate::utils::hash_password::hash_password;
    use crate::database::models::model_user::User;

    let mut hashed_password: String = String::new();
    match hash_password(password) {
        Ok(hash) => {
            hashed_password = String::from(&hash);
        },
        Err(err) =>  {
            println!("Error: {:?}", err);
        }
    };

    let user = User::new( String::from(email), hashed_password, true);

    let is_user_existing = query!(
        "SELECT email FROM utilisateur WHERE email = $1",
        user.get_email()
    ).fetch_optional(pool).await.unwrap();

    if let None = is_user_existing {
        query!(
        "INSERT INTO utilisateur (email, password, is_admin) VALUES ($1, $2, $3)",
        user.get_email(),
        user.get_password(),
        true
        ).execute(pool).await.unwrap();
    }
}
