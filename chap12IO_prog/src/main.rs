mod common;
mod modules;
mod services;
mod models;

fn main() {
    common::hello_world::hello_world();
    common::helpers::print_message("This is a categorized project!");

    modules::auth::login("Alice");
    modules::user::create_user("Bob");

    services::database::connect_db();
    services::api::fetch_data();

    let user = models::user::User::new(1, String::from("Charlie"));
    println!("Created user: {} with ID {}", user.name, user.id);
}
