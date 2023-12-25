use serde::Serialize;
use std::collections::HashMap;
use actix_files::NamedFile;
use actix_web::{ 
    web::{self}, 
    App, 
    HttpResponse, 
    HttpServer, 
    error,
    Result,
};
use ohm::Operation;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    result: bool,
    err: String,
}

#[derive(Debug, Serialize)]
struct IndexResponse {
    result: bool,
    routes: HashMap<String, String>,
}

async fn index() -> HttpResponse{
    
    let mut routes: HashMap<String, String> = HashMap::new();
    routes.insert("help for requests".to_string(), "/help".to_string());
    routes.insert("calculation of ohms".to_string(), "/ohm".to_string());

    let idx = IndexResponse{
        result: true,
        routes: routes,
    };
    HttpResponse::Ok().json(idx)
}


async fn help() -> Result<NamedFile>  {    
    Ok(NamedFile::open("src/pdf/ohms.pdf")?)
}

async fn ohm(item: web::Json<Operation>) -> HttpResponse {

    let opt = item.operation_type.clone().to_uppercase();
    let uk1 = item.unknown_1.clone().to_uppercase();
    let uk2 = item.unknown_2.clone().to_uppercase();

    let resp = Operation{
            operation_type: opt, 
            unknown_1: uk1, 
            unknown_2: uk2, 
            value_unknown_1: item.value_unknown_1, 
            value_unknown_2: item.value_unknown_2
        }.new();
    
    match resp {
        
        Ok(resp) => {
            HttpResponse::Ok().json(resp)
        }

        Err(er) => {

            let r = ErrorResponse{
                result: false,
                err: er.to_string(),
            };
            
            HttpResponse::BadRequest().json(r)
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req|{
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });
        App::new()
            .service(
                web::resource("/ohm")
                    .app_data(json_config)
                    .route(web::get().to(ohm)
                )
            )
            .service(
                web::resource("/help")
                    .route(web::get().to(help)
                )
            )
            .service(
                web::resource("/")
                    .route(web::get().to(index)
                )
            )
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
