#[macro_use]
extern crate rocket;
use prostgen::clients::UserServiceClient;
use prostgen::clients::GrowerClient;
use prostgen::user::{NewUser, User};
use prostgen::grower::{NewCannibanoidScreen, CannibanoidScreen};
use rocket::serde::json::Json;
mod user_interceptor;
use std::env::var as env_var;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

struct GrowerServiceAddr(String);
struct UserServiceAddr(String);
impl Default for GrowerServiceAddr {
    fn default() -> Self {
        return Self(String::from("http://[::1]:8077"));
    }
}
impl Default for UserServiceAddr {
    fn default() -> Self {
        return Self(String::from("http://[::1]:8076"));
    }
}

#[post("/", data = "<user>")]
async fn create_user(user: Json<NewUser>) -> Json<User> {
    let servaddr = UserServiceAddr::default().0;
    let mut client = UserServiceClient::connect(servaddr)
        .await
        .unwrap();
    let request = tonic::Request::new(user.into_inner());
    let response = client.create_user(request).await.unwrap();
    return Json(response.into_inner());
}

#[post("/", data = "<c_screen>")]
async fn create_cannibanoid_screen(c_screen: Json<NewCannibanoidScreen>) -> Json<CannibanoidScreen> {
    let servaddr = GrowerServiceAddr::default().0;
    println!("Connecting to {}", servaddr);
    let mut client = GrowerClient::connect(servaddr)
        .await
        .unwrap();
    let request = tonic::Request::new(c_screen.into_inner());
    let response = client.create_cannibanoid_screen(request).await.unwrap();
    return Json(response.into_inner());
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/user", routes![create_user])
        .mount("/grower", routes![create_cannibanoid_screen])
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
            .body(r#"{"name": "test", "email": "me@gmail.com", "password_hash": "12x"}"#)
            .dispatch()
            .await;
        assert_eq!(response.status(), Status::Ok);
        assert!(response.into_json::<User>().await.is_some());
    }
}
