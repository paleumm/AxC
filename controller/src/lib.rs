use actix_web::{
    get, post,
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};

mod pod;
mod utils;
mod volume;
use crate::{pod::*, utils::*, volume::*};

#[post("/create")]
pub async fn create(req: HttpRequest, config: Json<Token>) -> impl Responder {
    let params = web::Query::<PileParams>::from_query(req.query_string()).unwrap();
    let name = format!("pile-runtime-{}-{}", params.name, params.id);
    let pvc_name = format!("pile-pvc-{}-{}", params.name, params.id);
    match config.check_token().await {
        Some(_) => {
            create_pod(read_pod_config(), name.clone(), pvc_name.clone()).await.unwrap();
            HttpResponse::Ok()
        }
        None => {
            println!("token invalid");
            HttpResponse::Forbidden()
        }
    }
}

#[post("/delete")]
pub async fn delete(req: HttpRequest, config: Json<Token>) -> impl Responder {
    let params = web::Query::<PileParams>::from_query(req.query_string()).unwrap();
    let name = format!("pile-runtime-{}-{}", params.name, params.id);
    match config.check_token().await {
        Some(_) => {
            delete_pod(&name).await.unwrap();
            HttpResponse::Ok()
        }
        None => {
            println!("token invalid");
            HttpResponse::Forbidden()
        }
    }
}

#[post("/exec")]
pub async fn exec(req: HttpRequest, config: Json<Token>) -> impl Responder {
    let params = web::Query::<PileParams>::from_query(req.query_string()).unwrap();
    let name = format!("pile-runtime-{}-{}", params.name, params.id);
    match config.check_token().await {
        Some(_) => {
            exec_pod(&name).await.unwrap();
            HttpResponse::Ok()
        }
        None => {
            println!("token invalid");
            HttpResponse::Forbidden()
        }
    }
}

#[post("/create/user")]
pub async fn create_user(req: HttpRequest, config: Json<Token>) -> impl Responder {
    let params = web::Query::<PileParams>::from_query(req.query_string()).unwrap();
    let name = format!("pile-pvc-{}-{}", params.name, params.id);
    match config.check_token().await {
        Some(_) => {
            create_pvc(read_pvc_config(), name).await.unwrap();
            HttpResponse::Ok()
        }
        None => {
            println!("token invalid");
            HttpResponse::Forbidden()
        }
    }
}

#[post("/delete/user")]
pub async fn delete_user(req: HttpRequest, config: Json<Token>) -> impl Responder {
    let params = web::Query::<PileParams>::from_query(req.query_string()).unwrap();
    let name = format!("pile-pvc-{}-{}", params.name, params.id);
    match config.check_token().await {
        Some(_) => {
            delete_pvc(&name).await.unwrap();
            HttpResponse::Ok()
        }
        None => {
            println!("token invalid");
            HttpResponse::Forbidden()
        }
    }
}

#[post("/write")]
pub async fn write(req: HttpRequest, info: Json<PileResult>) -> impl Responder {
    let params = web::Query::<PileParams>::from_query(req.query_string()).unwrap();
    let name = format!("pile-pvc-{}-{}", params.name, params.id);
    match info.check_token().await {
        Some(_) => {
            // change to pvc path from params
            let root = "config/pv-user/generator/jgraph.json".to_string();
            write_pvc(&root, &info.result).unwrap();
            HttpResponse::Ok()
        }
        None => {
            println!("token invalid");
            HttpResponse::Forbidden()
        }
    }
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/")]
async fn hello() -> impl Responder {
    "Controller"
}