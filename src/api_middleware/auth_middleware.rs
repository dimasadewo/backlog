use std::pin::Pin;
use std::task::{Context, Poll};

use crate::repositories::authorization;
use crate::{actions, ConnectionPools, LoginData};
use actix_service::{Service, Transform};
use actix_web::web::Data;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, web, Error, HttpResponse};
use futures::future::{ok, Either, Ready};
use futures::Future;
use qstring::QString;
use crate::models::User;
use std::sync::Mutex;

// There are two steps in api_middleware processing.
// 1. Middleware initialization, api_middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct AuthMiddleware;

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S> for AuthMiddleware
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthApiMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthApiMiddleware { service })
    }
}

pub struct AuthApiMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthApiMiddleware<S>
    where
        S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
        S::Future: 'static,
        B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        println!("Hi from start. You requested: {}", req.path());
        let path = "./assets/police.txt";
        let pools = req
            .app_data::<web::Data<ConnectionPools>>()
            .unwrap()
            .clone();
        let app_data = req.app_data::<web::Data<Mutex<LoginData>>>().unwrap().clone();

        let query_strings = req.query_string();
        let parse = QString::from(query_strings);
        let api_token = parse.get("api_token").unwrap_or("");
        println!("API Token: {}", api_token);
        let check = check_if_authorized(api_token.to_string(), pools);
        if check.is_some() {
            let mut login_data = app_data.lock().unwrap();
            login_data.user = check;
            Either::Left(self.service.call(req))
        } else {
            let message = std::fs::read_to_string(path).expect("File not found!");
            Either::Right(ok(req.into_response(
                HttpResponse::Unauthorized().body(message).into_body(),
            )))
        }
    }
}

fn check_if_authorized(api_token: String, pools: Data<ConnectionPools>) -> Option<User> {
    if api_token != "" {
        let conn = pools
            .hrm_pool
            .get()
            .expect("Couldn't get any mysql connection from pool!");
        let authorize = actions::get_user_by_token(&conn, &api_token);
        if let Ok(authorize) = authorize {
            if authorize.is_some() {
                authorize
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
