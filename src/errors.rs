// Copyright (C) 2022  Aravinth Manivannan <realaravinth@batsense.net>
// SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
//
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::convert::From;

use actix::MailboxError;
use actix_web::{
    error::ResponseError,
    http::{header, StatusCode},
    HttpResponse, HttpResponseBuilder,
};
use argon2_creds::errors::CredsError;
use db_core::errors::DBError;
use derive_more::{Display, Error};
use lettre::transport::smtp::Error as SmtpError;
use libmcaptcha::errors::CaptchaError;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot::error::RecvError;
use url::ParseError;
use validator::ValidationErrors;

#[derive(Debug, Display, Error)]
pub struct SmtpErrorWrapper(SmtpError);

#[derive(Debug, Display, Error)]
pub struct DBErrorWrapper(DBError);

impl std::cmp::PartialEq for DBErrorWrapper {
    fn eq(&self, other: &Self) -> bool {
        format!("{}", self.0) == format!("{}", other.0)
    }
}

impl std::cmp::PartialEq for SmtpErrorWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0.status() == other.0.status()
    }
}

#[derive(Debug, Display, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum ServiceError {
    #[display(fmt = "internal server error")]
    InternalServerError,

    #[display(
        fmt = "This server is is closed for registration. Contact admin if this is unexpecter"
    )]
    ClosedForRegistration,

    #[display(fmt = "The value you entered for email is not an email")] //405j
    NotAnEmail,
    #[display(fmt = "The value you entered for URL is not a URL")] //405j
    NotAUrl,

    #[display(fmt = "Wrong password")]
    WrongPassword,
    #[display(fmt = "Username not found")]
    UsernameNotFound,
    #[display(fmt = "Account not found")]
    AccountNotFound,

    /// when the value passed contains profainity
    #[display(fmt = "Can't allow profanity in usernames")]
    ProfainityError,
    /// when the value passed contains blacklisted words
    /// see [blacklist](https://github.com/shuttlecraft/The-Big-Username-Blacklist)
    #[display(fmt = "Username contains blacklisted words")]
    BlacklistError,
    /// when the value passed contains characters not present
    /// in [UsernameCaseMapped](https://tools.ietf.org/html/rfc8265#page-7)
    /// profile
    #[display(fmt = "username_case_mapped violation")]
    UsernameCaseMappedError,

    #[display(fmt = "Passsword too short")]
    PasswordTooShort,
    #[display(fmt = "Password too long")]
    PasswordTooLong,
    #[display(fmt = "Passwords don't match")]
    PasswordsDontMatch,

    /// when the a username is already taken
    #[display(fmt = "Username not available")]
    UsernameTaken,

    /// email is already taken
    #[display(fmt = "Email not available")]
    EmailTaken,

    /// Unable to send email
    #[display(fmt = "Unable to send email, contact admin")]
    UnableToSendEmail(SmtpErrorWrapper),

    /// token not found
    #[display(fmt = "Token not found. Is token registered?")]
    TokenNotFound,

    #[display(fmt = "{}", _0)]
    CaptchaError(CaptchaError),

    #[display(fmt = "{}", _0)]
    DBError(DBErrorWrapper),

    /// captcha not found
    #[display(fmt = "Captcha not found.")]
    CaptchaNotFound,

    /// Traffic pattern not found
    #[display(fmt = "Traffic pattern not found")]
    TrafficPatternNotFound,
}

#[derive(Serialize, Deserialize)]
#[cfg(not(tarpaulin_include))]
pub struct ErrorToResponse {
    pub error: String,
}

#[cfg(not(tarpaulin_include))]
impl ResponseError for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .append_header((header::CONTENT_TYPE, "application/json; charset=UTF-8"))
            .body(
                serde_json::to_string(&ErrorToResponse {
                    error: self.to_string(),
                })
                .unwrap(),
            )
    }

    #[cfg(not(tarpaulin_include))]
    fn status_code(&self) -> StatusCode {
        match self {
            ServiceError::ClosedForRegistration => StatusCode::FORBIDDEN,
            ServiceError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::NotAnEmail => StatusCode::BAD_REQUEST,
            ServiceError::NotAUrl => StatusCode::BAD_REQUEST,
            ServiceError::WrongPassword => StatusCode::UNAUTHORIZED,
            ServiceError::UsernameNotFound => StatusCode::NOT_FOUND,
            ServiceError::AccountNotFound => StatusCode::NOT_FOUND,

            ServiceError::ProfainityError => StatusCode::BAD_REQUEST,
            ServiceError::BlacklistError => StatusCode::BAD_REQUEST,
            ServiceError::UsernameCaseMappedError => StatusCode::BAD_REQUEST,

            ServiceError::PasswordTooShort => StatusCode::BAD_REQUEST,
            ServiceError::PasswordTooLong => StatusCode::BAD_REQUEST,
            ServiceError::PasswordsDontMatch => StatusCode::BAD_REQUEST,

            ServiceError::UsernameTaken => StatusCode::BAD_REQUEST,
            ServiceError::EmailTaken => StatusCode::BAD_REQUEST,

            ServiceError::TokenNotFound => StatusCode::NOT_FOUND,
            ServiceError::CaptchaError(e) => {
                log::error!("{}", e);
                match e {
                    CaptchaError::MailboxError => StatusCode::INTERNAL_SERVER_ERROR,
                    _ => StatusCode::BAD_REQUEST,
                }
            }

            ServiceError::UnableToSendEmail(e) => {
                log::error!("{}", e.0);
                StatusCode::INTERNAL_SERVER_ERROR
            }

            ServiceError::DBError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceError::CaptchaNotFound => StatusCode::NOT_FOUND,
            ServiceError::TrafficPatternNotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl From<CredsError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: CredsError) -> ServiceError {
        match e {
            CredsError::UsernameCaseMappedError => ServiceError::UsernameCaseMappedError,
            CredsError::ProfainityError => ServiceError::ProfainityError,
            CredsError::BlacklistError => ServiceError::BlacklistError,
            CredsError::NotAnEmail => ServiceError::NotAnEmail,
            CredsError::Argon2Error(_) => ServiceError::InternalServerError,
            CredsError::PasswordTooLong => ServiceError::PasswordTooLong,
            CredsError::PasswordTooShort => ServiceError::PasswordTooShort,
        }
    }
}

impl From<DBError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: DBError) -> ServiceError {
        println!("from conversin: {}", e);
        match e {
            DBError::UsernameTaken => ServiceError::UsernameTaken,
            DBError::SecretTaken => ServiceError::InternalServerError,
            DBError::EmailTaken => ServiceError::EmailTaken,
            DBError::AccountNotFound => ServiceError::AccountNotFound,
            DBError::CaptchaNotFound => ServiceError::CaptchaNotFound,
            DBError::TrafficPatternNotFound => ServiceError::TrafficPatternNotFound,
            _ => ServiceError::DBError(DBErrorWrapper(e)),
        }
    }
}

impl From<ValidationErrors> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(_: ValidationErrors) -> ServiceError {
        ServiceError::NotAnEmail
    }
}

impl From<ParseError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(_: ParseError) -> ServiceError {
        ServiceError::NotAUrl
    }
}

#[cfg(not(tarpaulin_include))]
impl From<CaptchaError> for ServiceError {
    fn from(e: CaptchaError) -> ServiceError {
        ServiceError::CaptchaError(e)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<SmtpError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: SmtpError) -> Self {
        ServiceError::UnableToSendEmail(SmtpErrorWrapper(e))
    }
}

#[cfg(not(tarpaulin_include))]
impl From<RecvError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: RecvError) -> Self {
        log::error!("{:?}", e);
        ServiceError::InternalServerError
    }
}

#[cfg(not(tarpaulin_include))]
impl From<MailboxError> for ServiceError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: MailboxError) -> Self {
        log::error!("{:?}", e);
        ServiceError::InternalServerError
    }
}

#[cfg(not(tarpaulin_include))]
pub type ServiceResult<V> = std::result::Result<V, ServiceError>;

#[derive(Debug, Display, PartialEq, Error)]
#[cfg(not(tarpaulin_include))]
pub enum PageError {
    #[display(fmt = "Something weng wrong: Internal server error")]
    InternalServerError,

    #[display(fmt = "{}", _0)]
    ServiceError(ServiceError),
}

#[cfg(not(tarpaulin_include))]
impl From<ServiceError> for PageError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: ServiceError) -> Self {
        PageError::ServiceError(e)
    }
}

#[cfg(not(tarpaulin_include))]
impl From<DBError> for PageError {
    #[cfg(not(tarpaulin_include))]
    fn from(e: DBError) -> Self {
        let se: ServiceError = e.into();
        se.into()
    }
}

impl ResponseError for PageError {
    fn error_response(&self) -> HttpResponse {
        use crate::PAGES;
        match self.status_code() {
            StatusCode::INTERNAL_SERVER_ERROR => HttpResponse::Found()
                .append_header((header::LOCATION, PAGES.errors.internal_server_error))
                .finish(),
            _ => HttpResponse::Found()
                .append_header((header::LOCATION, PAGES.errors.unknown_error))
                .finish(),
        }
    }

    #[cfg(not(tarpaulin_include))]
    fn status_code(&self) -> StatusCode {
        match self {
            PageError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            PageError::ServiceError(e) => e.status_code(),
        }
    }
}

#[cfg(not(tarpaulin_include))]
pub type PageResult<V> = std::result::Result<V, PageError>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PAGES;

    #[test]
    fn error_works() {
        let resp: HttpResponse = PageError::InternalServerError.error_response();
        assert_eq!(resp.status(), StatusCode::FOUND);
        let headers = resp.headers();
        assert_eq!(
            headers.get(header::LOCATION).unwrap(),
            PAGES.errors.internal_server_error
        );
    }
}
