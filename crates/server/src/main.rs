use db::{create_user, establish_connection, models::User};
use diesel::prelude::*;

fn main() {
    use db::schema::users::dsl::*;

    let connection = &mut establish_connection();

    create_user(
        connection,
        "L-Nafaryus",
        "asdasd",
        "L-Nafaryus",
        "l.nafaryus@elnafo.ru",
        true,
    );

    let results = users
        .select(User::as_select())
        .load(connection)
        .expect("Error loading users");

    println!("Found {} users", results.len());
}
