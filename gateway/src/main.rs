#[macro_use]
extern crate rocket;

mod router;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![router::index])
        .mount(
            "/user",
            routes![router::user::create_user, router::user::get_user_by_id],
        )
        .mount(
            "/grower",
            routes![
                router::grower::create_cannibanoid_screen,
                router::grower::create_terpenoid_screen
            ],
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    // DOCKER must be running
    #[rocket::async_test]
    async fn test_create_user() {
        // curl -X POST -H "Content-Type: application/json" -d '{"name": "test", "email": "me@gmail.com", "password_hash": "12x"}' http://localhost:8000/user
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .post("/user")
            .body(r#"{"name": "test", "is_grower": false, "email": "me@gmail.com", "password_hash": "12x"}"#)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_json::<prostgen::user::User>().await.is_some());
    }
}
