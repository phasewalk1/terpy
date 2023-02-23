use crate::limiter::RateLimitGuard;
use prostgen::clients::GrowerClient;
use prostgen::clients::UserServiceClient;
use prostgen::grower::{
    CannibanoidScreen, NewCannibanoidScreen, NewTerpenoidScreen, TerpenoidScreen,
};
use prostgen::user::{NewUser, User};
use rocket::serde::json::Json;
use rocket_governor::RocketGovernor;
use terpy_orm::db::pooling::rocket_wrapper::RocketPoolGuard;
use rocket_auth::{prelude::Error, *};
use rocket::form::*;
use rocket::response::Redirect;

#[post("/login", data = "<form>")]
pub async fn post_login(auth: Auth<'_>, form: Form<Login>) -> std::result::Result<Redirect, Error> {
    let result = auth.login(&form).await;
    println!("login attempt: {:?}", result);
    result?;
    Ok(Redirect::to("/"))
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
    pub async fn create_user(
        _limitguard: RocketGovernor<'_, RateLimitGuard>,
        user: Json<NewUser>,
        _conn: RocketPoolGuard,
    ) -> Json<User> {
        let servaddr = UserServiceAddr::default().0;
        let mut client = UserServiceClient::connect(servaddr).await.unwrap();
        let request = tonic::Request::new(user.into_inner());
        let response = client.create_user(request).await.unwrap();
        return Json(response.into_inner());
    }
    #[get("/", format = "json")]
    pub async fn get_user_by_id(
        _limitguard: RocketGovernor<'_, RateLimitGuard>,
        id_request: prostgen::user::UserByIdRequest,
        _conn: RocketPoolGuard,
    ) -> Json<User> {
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
        _limitguard: RocketGovernor<'_, RateLimitGuard>,
        c_screen: Json<NewCannibanoidScreen>,
        _conn: RocketPoolGuard,
    ) -> Json<CannibanoidScreen> {
        let servaddr = GrowerServiceAddr::default().0;
        println!("Connecting to {}", servaddr);
        let mut client = GrowerClient::connect(servaddr).await.unwrap();
        let request = tonic::Request::new(c_screen.into_inner());
        let response = client.create_cannibanoid_screen(request).await.unwrap();
        return Json(response.into_inner());
    }

    #[get("/cannibanoid", format = "json")]
    pub async fn get_cannibanoid_screen(
        _limitguard: RocketGovernor<'_, RateLimitGuard>,
        id: prostgen::grower::CannibanoidScreenById,
        _conn: RocketPoolGuard,
    ) -> Json<CannibanoidScreen> {
        let servaddr = GrowerServiceAddr::default().0;
        println!("Connecting to {}", servaddr);
        let mut client = GrowerClient::connect(servaddr).await.unwrap();
        let request = tonic::Request::new(id);
        let response = client.get_cannibanoid_screen(request).await.unwrap();
        return Json(response.into_inner());
    }

    #[post("/", data = "<t_screen>", rank = 2)]
    pub async fn create_terpenoid_screen(
        _limitguard: RocketGovernor<'_, RateLimitGuard>,
        t_screen: Json<NewTerpenoidScreen>,
        _conn: RocketPoolGuard,
    ) -> Json<TerpenoidScreen> {
        let servaddr = GrowerServiceAddr::default().0;
        println!("Connecting to {}", servaddr);
        let mut client = GrowerClient::connect(servaddr).await.unwrap();
        let request = tonic::Request::new(t_screen.into_inner());
        let response = client.create_terpenoid_screen(request).await.unwrap();
        return Json(response.into_inner());
    }
    #[get("/terpenoid", format = "json")]
    pub async fn get_terpenoid_screen(
        _limitguard: RocketGovernor<'_, RateLimitGuard>,
        id: prostgen::grower::TerpenoidScreenById,
        _conn: RocketPoolGuard,
    ) -> Json<TerpenoidScreen> {
        let servaddr = GrowerServiceAddr::default().0;
        println!("Connecting to {}", servaddr);
        let mut client = GrowerClient::connect(servaddr).await.unwrap();
        let request = tonic::Request::new(id);
        let response = client.get_terpenoid_screen(request).await.unwrap();
        return Json(response.into_inner());
    }
}
