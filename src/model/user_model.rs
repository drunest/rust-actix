use actix_web::web::Data;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{thing, Array, Object, Value};

use crate::prelude::*;
use crate::repository::surrealdb_repo::{Creatable, Patchable, SurrealDBRepo};
use crate::utils::macros::map;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<String>,
    pub cid: String,
    pub public_key: String,
    pub private_key: String,
    pub name: String,
    pub version: u64,
    pub avatar: String,
    pub email: String,
    pub creation_date: String,
    pub online_state: String,
    pub follow_ids: Array,
    pub is_visible: bool,
    pub is_inactive: bool,
}

impl From<User> for Value {
    fn from(val: User) -> Self {
        match val.id {
            Some(v) => map![
              "id".into() => v.into(),
              "cid".into() => val.cid.into(),
              "publicKey".into() => val.public_key.into(),
              "privateKey".into() => val.private_key.into(),
              "name".into() => val.name.into(),
              "version".into() => val.version.into(),
              "avatar".into() => val.avatar.into(),
              "email".into() => val.email.into(),
              "creationDate".into() => val.creation_date.into(),
              "onlineState".into() => val.online_state.into(),
              "followIds".into() => val.follow_ids.into(),
              "isVisible".into() => val.is_visible.into(),
              "isInactive".into() => val.is_inactive.into(),
            ]
            .into(),
            None => map![
              "cid".into() => val.cid.into(),
              "publicKey".into() => val.public_key.into(),
              "privateKey".into() => val.private_key.into(),
              "name".into() => val.name.into(),
              "version".into() => val.version.into(),
              "avatar".into() => val.avatar.into(),
              "email".into() => val.email.into(),
              "creationDate".into() => val.creation_date.into(),
              "onlineState".into() => val.online_state.into(),
              "followIds".into() => val.follow_ids.into(),
              "isVisible".into() => val.is_visible.into(),
              "isInactive".into() => val.is_inactive.into(),
            ]
            .into(),
        }
    }
}

impl Creatable for User {}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatch {
    pub cid: Option<String>,
    pub public_key: Option<String>,
    pub private_key: Option<String>,
    pub name: Option<String>,
    pub version: Option<u64>,
    pub avatar: Option<String>,
    pub email: Option<String>,
    pub creation_date: Option<String>,
    pub online_state: Option<String>,
    pub follow_ids: Option<Array>,
    pub is_visible: Option<bool>,
    pub is_inactive: Option<bool>,
}

impl From<UserPatch> for Value {
    fn from(val: UserPatch) -> Self {
        let mut value = BTreeMap::new();

        if let Some(v) = val.cid {
            value.insert("cid".into(), v.into());
        }

        if let Some(v) = val.public_key {
            value.insert("publicKey".into(), v.into());
        }

        if let Some(v) = val.private_key {
            value.insert("privateKey".into(), v.into());
        }

        if let Some(v) = val.name {
            value.insert("name".into(), v.into());
        }

        if let Some(v) = val.version {
            value.insert("version".into(), v.into());
        }

        if let Some(v) = val.avatar {
            value.insert("avatar".into(), v.into());
        }

        if let Some(v) = val.email {
            value.insert("email".into(), v.into());
        }

        if let Some(v) = val.creation_date {
            value.insert("creationDate".into(), v.into());
        }

        if let Some(v) = val.online_state {
            value.insert("onlineState".into(), v.into());
        }

        if let Some(v) = val.follow_ids {
            value.insert("followIds".into(), v.into());
        }

        if let Some(v) = val.is_visible {
            value.insert("isVisible".into(), v.into());
        }

        if let Some(v) = val.is_inactive {
            value.insert("isInactive".into(), v.into());
        }
        Value::from(value)
    }
}

impl Patchable for UserPatch {}

pub struct UserBMC;

impl UserBMC {
    pub async fn get_all(db: Data<SurrealDBRepo>) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM user;";

        let res = db.ds.execute(ast, &db.ses, None, true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by_ids(
        db: Data<SurrealDBRepo>,
        ids: Vec<&str>,
    ) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM user WHERE id INSIDE $ids;";

        let ids: Vec<String> = ids.iter().map(|id| format!("user:{}", id)).collect();
        let ids_slice: Vec<&str> = ids.iter().map(|id| id.as_str()).collect();

        let vars: BTreeMap<String, Value> = map!["ids".into() => ids_slice.into()];

        let res = db.ds.execute(ast, &db.ses, Some(vars), true).await?;

        let first_res = res.into_iter().next().expect("Did not get a response");

        let array: Array = W(first_res.result?).try_into()?;

        array.into_iter().map(|value| W(value).try_into()).collect()
    }

    pub async fn search_by(db: Data<SurrealDBRepo>, search: (String, Value)) -> Result<Vec<Object>, Error> {
        let ast = "SELECT * FROM user WHERE $key = $value;";
        println!("search: {:?}", search);

        let vars: BTreeMap<String, Value> = map!["key".into() => search.0.into(), "value".into() => search.1];

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

        let tid = format!("user:{}", tid);

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

        let tid = format!("user:{}", tid);

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

        let tid = format!("user:{}", tid);

        let vars = map!["th".into() => thing(&tid)?.into()];

        let ress = db.ds.execute(sql, &db.ses, Some(vars), false).await?;

        let first_res = ress.into_iter().next().expect("id not returned");

        first_res.result?;

        Ok(tid)
    }
}
