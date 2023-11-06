pub fn create_user(email: String, password: String) -> () {
    use crate::utils::hash_password::hash_password;
    use crate::database::models::model_user::User;

    let mut new_password: String = String::new();
    match hash_password(password) {
        Ok(hash) => {
            new_password = String::from(&hash);
        },
        Err(err) =>  {
            println!("Error: {:?}", err);
        }
    };

    let user = User::new(1, String::from(email), new_password, true);
    println!("User: {:?}", user);
}
