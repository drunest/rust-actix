use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use serde::Deserialize;
use serde_json::Value;

use crate::model::item_model::{Item, ItemBMC, ItemPatch};
use crate::repository::surrealdb_repo::SurrealDBRepo;

#[post("/items")]
pub async fn create_item(db: Data<SurrealDBRepo>, new_item: Json<Item>) -> HttpResponse {
    let data = Item {
        id: None,
        cid: new_item.cid.to_owned(),
        name: new_item.name.to_owned(),
        owner_id: new_item.owner_id.to_owned(),
        version: new_item.version.to_owned(),
        content: new_item.content.to_owned(),
        image_url: new_item.image_url.to_owned(),
        creation_date: new_item.creation_date.to_owned(),
        edition_date: new_item.edition_date.to_owned(),
        tag_ids: new_item.tag_ids.to_owned(),
        follower_ids: new_item.follower_ids.to_owned(),
        is_visible: new_item.is_visible.to_owned(),
        is_archived: new_item.is_archived.to_owned(),
    };

    let item_detail = ItemBMC::create(db, "item", data).await;

    match item_detail {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/items/{id}")]
pub async fn get_item(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }

    let item_detail = ItemBMC::get(db, &id).await;

    match item_detail {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/items/{id}")]
pub async fn update_item(
    db: Data<SurrealDBRepo>,
    path: Path<String>,
    item_patch: Json<ItemPatch>,
) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let data = ItemPatch {
        cid: item_patch.cid.to_owned(),
        name: item_patch.name.to_owned(),
        owner_id: item_patch.owner_id.to_owned(),
        version: item_patch.version.to_owned(),
        content: item_patch.content.to_owned(),
        image_url: item_patch.image_url.to_owned(),
        creation_date: item_patch.creation_date.to_owned(),
        edition_date: item_patch.edition_date.to_owned(),
        tag_ids: item_patch.tag_ids.to_owned(),
        follower_ids: item_patch.follower_ids.to_owned(),
        is_visible: item_patch.is_visible.to_owned(),
        is_archived: item_patch.is_archived.to_owned(),
    };

    let update_result = ItemBMC::update(db, &id, data).await;

    match update_result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
#[delete("/items/{id}")]
pub async fn delete_item(db: Data<SurrealDBRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();

    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let result = ItemBMC::delete(db, &id).await;

    match result {
        Ok(item) => HttpResponse::Ok().json(item),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/items")]
pub async fn get_items(db: Data<SurrealDBRepo>) -> HttpResponse {
    let result = ItemBMC::get_all(db).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct SearchItemsByIds {
    ids: Vec<String>,
}

#[get("/itemsByIds")]
pub async fn search_items_by_ids(
    db: Data<SurrealDBRepo>,
    search_params: Json<SearchItemsByIds>,
) -> HttpResponse {
    let array_ids = search_params.ids.iter().map(|c| c.as_str()).collect();
    let result = ItemBMC::search_by_ids(db, array_ids).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[derive(Deserialize)]
pub struct SearchItemsBy {
    param: (String, Value),
}

#[get("/itemsBy")]
pub async fn search_items_by(
    db: Data<SurrealDBRepo>,
    search_params: Json<SearchItemsBy>,
) -> HttpResponse {
    let key = search_params.param.0.to_owned();
    let value = search_params.param.1.to_owned();

    let result = match key.as_str() {
        "name" => ItemBMC::search_by_name(db, &value.as_str().unwrap_or("")).await,
        "ownerId" => ItemBMC::search_by_owner_id(db, &value.as_str().unwrap_or("")).await,
        "creationDate" => ItemBMC::search_by_creation_date(db, &value.as_str().unwrap_or("")).await,
        "editionDate" => ItemBMC::search_by_edition_date(db, &value.as_str().unwrap_or("")).await,
        "tagIds" => {
            let tag_ids = match &value {
                serde_json::Value::Array(arr) => {
                    arr.iter().map(|c| c.as_str().unwrap_or("")).collect()
                }
                _ => Vec::new(),
            };
            ItemBMC::search_by_tag_ids(db, tag_ids).await
        }
        "followerIds" => {
            let follower_ids = match &value {
                serde_json::Value::Array(arr) => {
                    arr.iter().map(|c| c.as_str().unwrap_or("")).collect()
                }
                _ => Vec::new(),
            };
            ItemBMC::search_by_follower_ids(db, follower_ids).await
        }
        "isVisible" => ItemBMC::search_by_is_visible(db, value.as_bool().unwrap_or(false)).await,
        "isArchived" => ItemBMC::search_by_is_archived(db, value.as_bool().unwrap_or(false)).await,
        _ => panic!("Invalid key"),
    };

    match result {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
