#[macro_use]
extern crate rocket;
use prostgen::prostgen::UserServiceClient;
use prostgen::prostgen::{NewUser, User};
use rocket::serde::json::Json;
mod user_interceptor;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/user", data = "<user>")]
async fn create_user(user: Json<NewUser>) -> Json<User> {
    let mut client = UserServiceClient::connect("http://[::1]:50051")
        .await
        .unwrap();
    let request = tonic::Request::new(user.into_inner());
    let response = client.create_user(request).await.unwrap();
    return Json(response.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, create_user])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::asynchronous::Client;

    #[rocket::async_test]
    async fn test_create_user() {
        // curl -X POST -H "Content-Type: application/json" -d '{"name": "test", "email": "me@gmail.com", "password_hash": "12x"}' http://localhost:8000/user
        let client = Client::tracked(rocket()).await.unwrap();
        let response = client
            .post("/user")
            .body(r#"{"name": "test", "email": "me@gmail.com", "password_hash": "12x"}"#)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_json::<User>().await.is_some());
    }
}
