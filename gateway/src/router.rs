use prostgen::clients::GrowerClient;
use prostgen::clients::UserServiceClient;
use prostgen::grower::{
    CannibanoidScreen, NewCannibanoidScreen, NewTerpenoidScreen, TerpenoidScreen,
};
use prostgen::user::{NewUser, User};
use rocket::serde::json::Json;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

pub struct GrowerServiceAddr(String);
pub struct UserServiceAddr(String);

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

pub mod user {
  use super::*;

  #[post("/", data = "<user>")]
  pub async fn create_user(user: Json<NewUser>) -> Json<User> {
      let servaddr = UserServiceAddr::default().0;
      let mut client = UserServiceClient::connect(servaddr).await.unwrap();
      let request = tonic::Request::new(user.into_inner());
      let response = client.create_user(request).await.unwrap();
      return Json(response.into_inner());
  }
  #[get("/", format = "json")]
  pub async fn get_user_by_id(id_request: prostgen::user::UserByIdRequest) -> Json<User> {
      let servaddr = UserServiceAddr::default().0;
      let mut client = UserServiceClient::connect(servaddr).await.unwrap();
      let request = tonic::Request::new(prostgen::user::UserByIdRequest {
          id: id_request.id.to_string(),
      });
      let response = client.user_by_id(request).await.unwrap();
      return Json(response.into_inner());
  }
}


pub mod grower {
  use super::*;
  
  #[post("/", data = "<c_screen>", rank = 1)]
  pub async fn create_cannibanoid_screen(
      c_screen: Json<NewCannibanoidScreen>,
  ) -> Json<CannibanoidScreen> {
      let servaddr = GrowerServiceAddr::default().0;
      println!("Connecting to {}", servaddr);
      let mut client = GrowerClient::connect(servaddr).await.unwrap();
      let request = tonic::Request::new(c_screen.into_inner());
      let response = client.create_cannibanoid_screen(request).await.unwrap();
      return Json(response.into_inner());
  }

  #[post("/", data = "<t_screen>", rank = 2)]
  pub async fn create_terpenoid_screen(t_screen: Json<NewTerpenoidScreen>) -> Json<TerpenoidScreen> {
      let servaddr = GrowerServiceAddr::default().0;
      println!("Connecting to {}", servaddr);
      let mut client = GrowerClient::connect(servaddr).await.unwrap();
      let request = tonic::Request::new(t_screen.into_inner());
      let response = client.create_terpenoid_screen(request).await.unwrap();
      return Json(response.into_inner());
  }
}


