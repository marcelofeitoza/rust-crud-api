use actix_web::{
    delete, error, get, post, put, web, web::Json, App, HttpResponse, HttpServer, Responder,
};
use serde::{Deserialize, Serialize};

mod db;

use db::*;

struct User {
    id: String,
    name: String,
    username: String,
    email: String,
    password: String,
}

// GET: /
#[get("/")]
async fn index() -> impl Responder {
    return HttpResponse::Ok().body("API is running");
}

// GET: /users
#[derive(Deserialize, Serialize)]
struct UserVec {
    id: String,
    name: String,
    username: String,
    email: String,
}

#[get("/users")]
async fn get_users(
    client: web::Data<PrismaClient>,
) -> Result<web::Json<Vec<UserVec>>, error::Error> {
    let users = client
        .user()
        .find_many(vec![])
        .exec()
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    let mut user_vec = Vec::new();

    for user in users {
        user_vec.push(UserVec {
            id: user.id,
            name: user.name,
            username: user.username,
            email: user.email,
        });
    }

    Ok(web::Json(user_vec))
}

// GET: /users/:id
#[derive(Deserialize, Serialize)]
struct GetUser {
    name: String,
    username: String,
    email: String,
}

#[get("/users/{id}")]
async fn get_user(
    client: web::Data<PrismaClient>,
    id: web::Path<String>,
) -> Result<web::Json<GetUser>, error::Error> {
    let user_found = client
        .user()
        .find_unique(user::id::equals(id.clone()))
        .exec()
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    if let Some(user) = user_found {
        Ok(web::Json(GetUser {
            name: user.name,
            username: user.username,
            email: user.email,
        }))
    } else {
        Err(error::ErrorNotFound(Json("User not found".to_string())))
    }
}

// POST: /users/create
#[derive(Deserialize, Serialize)]
struct NewUser {
    name: String,
    username: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Serialize)]
struct ReturnUser {
    name: String,
    username: String,
    email: String,
    message: String,
}

#[post("/users/create")]
async fn create_user(
    client: web::Data<PrismaClient>,
    body: web::Json<NewUser>,
) -> Result<web::Json<ReturnUser>, error::Error> {
    let new_user = client
        .user()
        .create(
            body.name.clone(),
            body.username.clone(),
            body.email.clone(),
            body.password.clone(),
            vec![],
        )
        .exec()
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    Ok(web::Json(ReturnUser {
        name: new_user.name,
        username: new_user.username,
        email: new_user.email,
        message: "User created successfully".to_string(),
    }))
}

// PUT: /users/update/:id
#[derive(Deserialize, Serialize)]
struct UpdateUser {
    name: String,
    username: String,
    email: String,
    password: String,
}

#[put("/users/update/{id}")]
async fn update_user(
    client: web::Data<PrismaClient>,
    body: web::Json<UpdateUser>,
    id: web::Path<String>,
) -> Result<web::Json<UpdateUser>, error::Error> {
    let updated_user = client
        .user()
        .update(
            user::id::equals(id.clone()),
            vec![
                user::name::set(body.name.clone()),
                user::username::set(body.username.clone()),
                user::email::set(body.email.clone()),
                user::password::set(body.password.clone()),
            ]
        )   
        .exec()
        .await
        .map_err(|e| error::ErrorInternalServerError(format!("{}", e)))?;

    Ok(web::Json(UpdateUser {
        name: updated_user.name,
        username: updated_user.username,
        email: updated_user.email,
        password: updated_user.password,
    }))
}

// DELETE: /users/delete/:id
#[derive(Deserialize, Serialize)]
struct DeleteUser {
    username: String,
    message: String,
}

#[delete("/users/delete/{id}")]
async fn delete_user(
    client: web::Data<PrismaClient>,
    id: web::Path<String>,
) -> Result<web::Json<ReturnUser>, error::Error> {
    let deleted_user = client
        .user()
        .delete(user::id::equals(id.clone()))
        .exec()
        .await
        .unwrap();

    Ok(web::Json(ReturnUser {
        name: deleted_user.name,
        username: deleted_user.username,
        email: deleted_user.email,
        message: "User deleted successfully".to_string(),
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = web::Data::new(PrismaClient::_builder().build().await.unwrap());

    #[cfg(debug_assertions)]
    client._db_push().await.unwrap();

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(index)
            .service(get_user)
            .service(get_users)
            .service(create_user)
            .service(update_user)
            .service(delete_user)
    })
    .bind(("0.0.0.0", 5500))?
    .run()
    .await
}
