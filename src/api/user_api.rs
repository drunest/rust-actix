// File: src/api/user_api.rs
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::Deserialize;
use serde_json::Value;

use crate::model::user_model::{User, UserBMC, UserPatch};
use crate::repository::surrealdb_repo::SurrealDBRepo;

#[post("/users")]
pub async fn create_user(db: Data<SurrealDBRepo>, new_user: Json<User>) -> HttpResponse {
    let data = User {
        id: None,
        cid: new_user.cid.to_owned(),
        public_key: new_user.public_key.to_owned(),
        private_key: new_user.private_key.to_owned(),
        name: new_user.name.to_owned(),
        version: new_user.version.to_owned(),
        avatar: new_user.avatar.to_owned(),
        email: new_user.email.to_owned(),
        creation_date: new_user.creation_date.to_owned(),
        online_state: new_user.online_state.to_owned(),
        follow_ids: new_user.follow_ids.to_owned(),
        is_visible: new_user.is_visible.to_owned(),
        is_inactive: new_user.is_inactive.to_owned(),
    };

    let user_detail = UserBMC::create(db, "user", data).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users/{id}")]
pub async fn get_user(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let user_detail = UserBMC::get(db, &id).await;

    match user_detail {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/users/{id}")]
pub async fn update_user(
    db: Data<SurrealDBRepo>,
    path: Path<String>,
    user_patch: Json<UserPatch>,
) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = UserPatch {
        cid: user_patch.cid.to_owned(),
        public_key: user_patch.public_key.to_owned(),
        private_key: user_patch.private_key.to_owned(),
        name: user_patch.name.to_owned(),
        version: user_patch.version.to_owned(),
        avatar: user_patch.avatar.to_owned(),
        email: user_patch.email.to_owned(),
        creation_date: user_patch.creation_date.to_owned(),
        online_state: user_patch.online_state.to_owned(),
        follow_ids: user_patch.follow_ids.to_owned(),
        is_visible: user_patch.is_visible.to_owned(),
        is_inactive: user_patch.is_inactive.to_owned(),
    };

    let update_result = UserBMC::update(db, &id, data).await;

    match update_result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/users/{id}")]
pub async fn delete_user(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let result = UserBMC::delete(db, &id).await;

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/users")]
pub async fn get_users(db: Data<SurrealDBRepo>) -> HttpResponse {
    let result = UserBMC::get_all(db).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct SearchUsersByIds {
    ids: Vec<String>,
}

#[get("/usersByIds")]
pub async fn search_users_by_ids(
    db: Data<SurrealDBRepo>,
    search_params: Json<SearchUsersByIds>,
) -> HttpResponse {
    let array_ids = search_params.ids.iter().map(|c| c.as_str()).collect();
    let result = UserBMC::search_by_ids(db, array_ids).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct SearchUsersBy {
    param: (String, Value),
}

#[get("/usersBy")]
pub async fn search_users_by(
    db: Data<SurrealDBRepo>,
    search_params: Json<SearchUsersBy>,
) -> HttpResponse {
    let value = search_params.param.to_owned();

    let result = UserBMC::search_by(db, value).await;

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
