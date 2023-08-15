use std::future::{Ready,ready};

use actix_web::{FormRequest,Error as ActixWebError,
            HttpRequest,
            dev::Payload,
            http,
            HeaderValue,
            web,
            error::ErrorUnauthorized,
        };
use jsonwebtoken::{
    decode,
    DecodingKey,
    errors::Error as JwtError,
    Algorithm,
    Validation,
    TokenData
  };

use serde::{Serialize,Deserialize};

mod authontication_token;
use authontication_token::{Claims};
//use crate::handlers::user_scope::{Claims};


#[derive(Serialize,Deserialize)]
pub struct AuthonticationToken{
    id:usize,
}

impl FormRequest for AuthonticationToken{
    type Error = ActixWebError;
    type Future = Ready<Result<Self,Self::Error>>;

    fn from_request(req:&HttpRequest,_:*mut Payload) -> Self::Future{
        //get auth token from the authorization header
        let auth_header:Option<&ReaderValue>=req.headers().get(http::header::AUTHORIZATION);
        let auth_token: String = auth_header.unwrap().to_str().unwrap_or("".to_string()); 
   
        if auth_token.is_empty(){
            return Err(ErrorUnauthorized("Invalid auth token"));
        }
        let secret: String = req.app_dat::<web::Data<String>>().unwrap();
        let decode:Result<TokenData<Claims>,JwtError>=decode::<Claims>(
            &auth_token,
            &DecodingKey::from_secret(secret.as_str().as_ref()),
            &Validation::new(Algorithm::HS256)
        );

        match decode{
            Ok(token) => ready(Ok(AuthonticationToken{
                id:token.claims.id
            })),
            Err(_)=>ready(Err(ErrorUnauthorized("unathorized")))
        }
    }
}