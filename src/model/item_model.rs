use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: Option<String>,
    pub cid: String,
    pub name: String,
    pub owner_id: String,
    pub version: u64,
    pub content: Array,
    pub image_url: String,
    pub creation_date: String,
    pub edition_date: String,
    pub tag_ids: Array,
    pub follower_ids: Array,
    pub is_visible: bool,
    pub is_archived: bool,
}

impl From<Item> for Value {
    fn from(val: Item) -> Self {
        match val.id {
            Some(v) => map![
              "id".into() => v.into(),
              "cid".into() => val.cid.into(),
              "name".into() => val.name.into(),
              "ownerId".into() => val.owner_id.into(),
              "version".into() => val.version.into(),
              "content".into() => val.content.into(),
              "imageUrl".into() => val.image_url.into(),
              "creationDate".into() => val.creation_date.into(),
              "editionDate".into() => val.edition_date.into(),
              "tagIds".into() => val.tag_ids.into(),
              "followerIds".into() => val.follower_ids.into(),
              "isVisible".into() => val.is_visible.into(),
              "isArchived".into() => val.is_archived.into(),
            ]
            .into(),
            None => map![
              "cid".into() => val.cid.into(),
              "name".into() => val.name.into(),
              "ownerId".into() => val.owner_id.into(),
              "version".into() => val.version.into(),
              "content".into() => val.content.into(),
              "imageUrl".into() => val.image_url.into(),
              "creationDate".into() => val.creation_date.into(),
              "editionDate".into() => val.edition_date.into(),
              "tagIds".into() => val.tag_ids.into(),
              "followerIds".into() => val.follower_ids.into(),
              "isVisible".into() => val.is_visible.into(),
              "isArchived".into() => val.is_archived.into(),
            ]
            .into(),
        }
    }
}

impl Creatable for Item {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ItemPatch {
    pub cid: Option<String>,
    pub name: Option<String>,
    pub owner_id: Option<String>,
    pub version: Option<u64>,
    pub content: Option<Array>,
    pub image_url: Option<String>,
    pub creation_date: Option<String>,
    pub edition_date: Option<String>,
    pub tag_ids: Option<Array>,
    pub follower_ids: Option<Array>,
    pub is_visible: Option<bool>,
    pub is_archived: Option<bool>,
}

impl From<ItemPatch> for Value {
    fn from(val: ItemPatch) -> Self {
        let mut value = BTreeMap::new();

        if let Some(v) = val.cid {
            value.insert("cid".into(), v.into());
        }

        if let Some(v) = val.name {
            value.insert("name".into(), v.into());
        }

        if let Some(v) = val.owner_id {
            value.insert("ownerId".into(), v.into());
        }

        if let Some(v) = val.version {
            value.insert("version".into(), v.into());
        }

        if let Some(v) = val.content {
            value.insert("content".into(), v.into());
        }

        if let Some(v) = val.image_url {
            value.insert("imageUrl".into(), v.into());
        }

        if let Some(v) = val.creation_date {
            value.insert("creationDate".into(), v.into());
        }

        if let Some(v) = val.edition_date {
            value.insert("editionDate".into(), v.into());
        }

        if let Some(v) = val.tag_ids {
            value.insert("tagIds".into(), v.into());
        }

        if let Some(v) = val.follower_ids {
            value.insert("followerIds".into(), v.into());
        }

        if let Some(v) = val.is_visible {
            value.insert("isVisible".into(), v.into());
        }

        if let Some(v) = val.is_archived {
            value.insert("isArchived".into(), v.into());
        }
        Value::from(value)
    }
}

impl Patchable for ItemPatch {}

pub struct ItemBMC;

impl ItemBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item;";

        let res = db.ds.execute(ast, &db.ses, None, true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_ids(
        db: Data<SurrealDBRepo>,
        ids: Vec<&str>,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE id INSIDE $ids;";

        let ids: Vec<String> = ids.iter().map(|id| format!("item:{}", id)).collect();
        let ids_slice: Vec<&str> = ids.iter().map(|id| id.as_str()).collect();

        let vars: BTreeMap<String, Value> = map!["ids".into() => ids_slice.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_name(db: Data<SurrealDBRepo>, name: &str) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE name CONTAINS $name;";

        let vars: BTreeMap<String, Value> = map!["name".into() => name.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_owner_id(
        db: Data<SurrealDBRepo>,
        owner_id: &str,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE ownerId = $owner_id;";

        let vars: BTreeMap<String, Value> = map!["owner_id".into() => owner_id.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_creation_date(
        db: Data<SurrealDBRepo>,
        creation_date: &str,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE creationDate = $creation_date;";

        let vars: BTreeMap<String, Value> = map!["creation_date".into() => creation_date.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_edition_date(
        db: Data<SurrealDBRepo>,
        edition_date: &str,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE editionDate = $edition_date;";

        let vars: BTreeMap<String, Value> = map!["edition_date".into() => edition_date.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_tag_ids(
        db: Data<SurrealDBRepo>,
        tag_ids: Vec<&str>,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE tagIds INSIDE $ids;";

        let ids: Vec<String> = tag_ids.iter().map(|id| format!("item:{}", id)).collect();
        let ids_slice: Vec<&str> = ids.iter().map(|id| id.as_str()).collect();

        let vars: BTreeMap<String, Value> = map!["ids".into() => ids_slice.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_follower_ids(
        db: Data<SurrealDBRepo>,
        follower_ids: Vec<&str>,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE followerIds INSIDE $ids;";

        let ids: Vec<String> = follower_ids.iter().map(|id| format!("item:{}", id)).collect();
        let ids_slice: Vec<&str> = ids.iter().map(|id| id.as_str()).collect();

        let vars: BTreeMap<String, Value> = map!["ids".into() => ids_slice.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_is_visible(
        db: Data<SurrealDBRepo>,
        is_visible: bool,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE isVisible = $is_visible;";

        let vars: BTreeMap<String, Value> = map!["is_visible".into() => is_visible.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_is_archived(
        db: Data<SurrealDBRepo>,
        is_archived: bool,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM item WHERE isArchived = $is_archived;";

        let vars: BTreeMap<String, Value> = map!["is_archived".into() => is_archived.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");
        let array: Array = W(first_res.result?).try_into()?;
        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn create<T: Creatable>(
        db: Data<SurrealDBRepo>,
        tb: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "CREATE type::table($tb) CONTENT $data RETURN *";

        let data: Object = W(data.into()).try_into()?;

        let vars: BTreeMap<String, Value> = map![
    "tb".into() => tb.into(),
    "data".into() => Value::from(data)];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_val = ress
            .into_iter()
            .next()
            .map(|r| r.result)
            .expect("id not returned")?;

        W(first_val.first()).try_into()
    }

    pub async fn get(db: Data<SurrealDBRepo>, tid: &str) -> Result<Object, Error> {
        let sql = "SELECT * FROM $th";

        let tid = format!("item:{}", tid);

        let vars: BTreeMap<String, Value> = map!["th".into() => thing(&tid)?.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("Did not get a response");

        W(first_res.result?.first()).try_into()
    }

    pub async fn update<T: Patchable>(
        db: Data<SurrealDBRepo>,
        tid: &str,
        data: T,
    ) -> Result<Object, Error> {
        let sql = "UPDATE $th MERGE $data RETURN *";

        let tid = format!("item:{}", tid);

        let vars = map![
    "th".into() => thing(&tid)?.into(),
    "data".into() => data.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), true).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        let result = first_res.result?;

        W(result.first()).try_into()
    }

    pub async fn delete(db: Data<SurrealDBRepo>, tid: &str) -> Result<String, Error> {
        let sql = "DELETE $th RETURN *";

        let tid = format!("item:{}", tid);

        let vars = map!["th".into() => thing(&tid)?.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        first_res.result?;

        Ok(tid)
    }
}
