use rocket::{*, form::Form};
use rocket_auth::{Auth, Signup, Login};

#[post("/signup", data="<form>")]
pub async fn signup(form: Form<Signup>, auth: Auth<'_>) {
  auth.signup(&form).await.unwrap();
  auth.login(&form.into()).await.unwrap();
}

#[post("/login", data="<form>")]
pub async fn login(form: Form<Login>, auth: Auth<'_>) {
  auth.login(&form).await.unwrap();
}

#[get("/logout")]
pub async fn logout(auth: Auth<'_>) {
  auth.logout().unwrap();
}