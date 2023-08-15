use actix_web::{ web, Scope, HttpResponse};
use serde::{
    Serialize,
    Deserialize,
};
use chrono::{Utc,Duration};
use jsonwebtoken::{
    encode,
    decode,
    EncodingKey,
    Header,
    DecodingKey,
    Validation,
    Algorithm,
    TokenData,
    errors::Error as JwtError
};


#[derive(Serialize,Deserialize)]
struct Claims{
    id:usize,
    exp:usize,
}

#[derive(Serialize,Deserialize)]
struct Response{
    message:String,
}
#[derive(Serialize,Deserialize)]
struct EncodeResponse{
    message:String,
    token:String,
}
pub fn user_scope()->Scope{
    web::scope("/user_scope")
        .route("/encode-token/{id}",web::get().to(encode_token))
        .route("/decode-token",web::get().to(decode_token))
        .route("/protected",web::get().to(protected))
}

async fn encode_token(path:web::Path<usize>,secret:web::Data<String>)->HttpResponse{
    let id: usize= path.into_inner();
    
    let exp:usize =(Utc::now() + Duration::days(365)).timestamp() as usize;
    let claims:Claims = Claims{
        id,
        exp
    };
    let token: String =encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_str().as_ref())
    ).unwrap();
    HttpResponse::Ok().json(EncodeResponse{
                                message: "encode_token".to_owned(),
                                token
                            })
}

#[derive(Serialize,Deserialize)]
struct DecodeBody{
    token:String
}
#[derive(Serialize,Deserialize)]
struct DecodeResponse{
    message:String,
    id:usize,
}

async fn decode_token(body: web::Json<DecodeBody>,secret:web::Data<String>)->HttpResponse{
    let decoded:Result<TokenData<Claims>,JwtError> = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(secret.as_str().as_ref()),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(token)=>HttpResponse::Ok().json(DecodeResponse{
            message:"Authorized".to_string(),
            id:token.claims.id,

        }),
        Err(e)=>HttpResponse::BadRequest().json(Response{message:e.to_string()})
    }

}

async fn protected()->HttpResponse{
    HttpResponse::Ok().json(Response{message:"protected".to_owned()})
}