/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use std::borrow::Cow;

use actix_identity::Identity;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use super::auth::Password;
use super::mcaptcha::get_random;
use crate::errors::*;
use crate::CheckLogin;
use crate::Data;

pub mod routes {

    pub struct Account {
        pub delete: &'static str,
        pub email_exists: &'static str,
        pub update_email: &'static str,
        pub get_secret: &'static str,
        pub update_secret: &'static str,
        pub username_exists: &'static str,
    }

    impl Default for Account {
        fn default() -> Self {
            let get_secret = "/api/v1/account/secret/";
            let update_secret = "/api/v1/account/secret";
            let delete = "/api/v1/account/delete";
            let email_exists = "/api/v1/account/email/exists";
            let username_exists = "/api/v1/account/username/exists";
            let update_email = "/api/v1/account/email";
            Self {
                get_secret,
                update_secret,
                username_exists,
                update_email,
                delete,
                email_exists,
            }
        }
    }
}

#[post("/api/v1/account/delete", wrap = "CheckLogin")]
pub async fn delete_account(
    id: Identity,
    payload: web::Json<Password>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    use argon2_creds::Config;
    use sqlx::Error::RowNotFound;

    let username = id.identity().unwrap();

    let rec = sqlx::query_as!(
        Password,
        r#"SELECT password  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await;

    id.forget();

    match rec {
        Ok(s) => {
            if Config::verify(&s.password, &payload.password)? {
                sqlx::query!("DELETE FROM mcaptcha_users WHERE name = ($1)", &username)
                    .execute(&data.db)
                    .await?;
                Ok(HttpResponse::Ok())
            } else {
                Err(ServiceError::WrongPassword)
            }
        }
        Err(RowNotFound) => return Err(ServiceError::UsernameNotFound),
        Err(_) => return Err(ServiceError::InternalServerError)?,
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckPayload {
    pub val: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountCheckResp {
    pub exists: bool,
}

#[post("/api/v1/account/username/exists")]
pub async fn username_exists(
    payload: web::Json<AccountCheckPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = sqlx::query!(
        "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE name = $1)",
        &payload.val,
    )
    .fetch_one(&data.db)
    .await?;

    let mut resp = AccountCheckResp { exists: false };

    if let Some(x) = res.exists {
        if x {
            resp.exists = true;
        }
    }

    Ok(HttpResponse::Ok().json(resp))
}

#[post("/api/v1/account/email/exists")]
pub async fn email_exists(
    payload: web::Json<AccountCheckPayload>,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let res = sqlx::query!(
        "SELECT EXISTS (SELECT 1 from mcaptcha_users WHERE email = $1)",
        &payload.val,
    )
    .fetch_one(&data.db)
    .await?;

    let mut resp = AccountCheckResp { exists: false };

    if let Some(x) = res.exists {
        if x {
            resp.exists = true;
        }
    }

    Ok(HttpResponse::Ok().json(resp))
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Secret {
    pub secret: String,
}

#[get("/api/v1/account/secret/", wrap = "CheckLogin")]
pub async fn get_secret(id: Identity, data: web::Data<Data>) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let secret = sqlx::query_as!(
        Secret,
        r#"SELECT secret  FROM mcaptcha_users WHERE name = ($1)"#,
        &username,
    )
    .fetch_one(&data.db)
    .await?;

    Ok(HttpResponse::Ok().json(secret))
}

#[post("/api/v1/account/secret/", wrap = "CheckLogin")]
pub async fn update_user_secret(
    id: Identity,
    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    let mut secret;

    loop {
        secret = get_random(32);
        let res = sqlx::query!(
            "UPDATE mcaptcha_users set secret = $1
        WHERE name = $2",
            &secret,
            &username,
        )
        .execute(&data.db)
        .await;
        if res.is_ok() {
            break;
        } else {
            if let Err(sqlx::Error::Database(err)) = res {
                if err.code() == Some(Cow::from("23505"))
                    && err.message().contains("mcaptcha_users_secret_key")
                {
                    continue;
                } else {
                    Err(sqlx::Error::Database(err))?;
                }
            };
        }
    }
    Ok(HttpResponse::Ok())
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Email {
    pub email: String,
}

/// update email
#[post("/api/v1/account/email/", wrap = "CheckLogin")]
pub async fn set_email(
    id: Identity,

    payload: web::Json<Email>,

    data: web::Data<Data>,
) -> ServiceResult<impl Responder> {
    let username = id.identity().unwrap();

    data.creds.email(&payload.email)?;

    let res = sqlx::query!(
        "UPDATE mcaptcha_users set email = $1
        WHERE name = $2",
        &payload.email,
        &username,
    )
    .execute(&data.db)
    .await;
    if !res.is_ok() {
        if let Err(sqlx::Error::Database(err)) = res {
            if err.code() == Some(Cow::from("23505"))
                && err.message().contains("mcaptcha_users_email_key")
            {
                Err(ServiceError::EmailTaken)?
            } else {
                Err(sqlx::Error::Database(err))?
            }
        };
    }
    Ok(HttpResponse::Ok())
}