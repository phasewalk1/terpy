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
