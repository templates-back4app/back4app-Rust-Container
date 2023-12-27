use actix_web::{web, App, HttpServer, HttpResponse, Error};
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    pub id: i32,
    pub username: String,
    pub email: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/person", web::post().to(create_person))
            .route("/people", web::get().to(get_people))
            .route("/person/{id}", web::put().to(update_person))
            .route("/person/{id}", web::delete().to(delete_person))
    })
        .bind("0.0.0.0:8000")?
        .run()
        .await
}

extern crate lazy_static;
use lazy_static::lazy_static;
use serde_derive::{Deserialize, Serialize};

lazy_static! {
    static ref DATA_STORE: Mutex<Vec<Person>> = Mutex::new(Vec::new());
}

// READ ALL
pub async fn get_people() -> Result<HttpResponse, Error> {
    let data_store = DATA_STORE.lock().unwrap();
    let people: Vec<Person> = data_store.clone();

    Ok(HttpResponse::Ok().json(people))
}

// CREATE
pub async fn create_person(new_person: web::Json<Person>) -> Result<HttpResponse, Error> {
    let mut data_store = DATA_STORE.lock().unwrap();
    let new_id = data_store.len() as i32 + 1;
    let mut person = new_person.into_inner();
    person.id = new_id;
    data_store.push(person.clone());

    Ok(HttpResponse::Ok().json(person))
}

// UPDATE
pub async fn update_person(
    id: web::Path<i32>,
    person_update: web::Json<Person>,
) -> Result<HttpResponse, Error> {
    let mut data_store = DATA_STORE.lock().unwrap();

    if let Some(person) = data_store.iter_mut().find(|p| p.id == *id) {
        *person = person_update.into_inner();
        Ok(HttpResponse::Ok().json("Person updated successfully"))
    } else {
        Ok(HttpResponse::NotFound().json("Person not found"))
    }
}

// DELETE
pub async fn delete_person(id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut data_store = DATA_STORE.lock().unwrap();

    if let Some(index) = data_store.iter().position(|p| p.id == *id) {
        data_store.remove(index);
        Ok(HttpResponse::Ok().json("Deleted successfully"))
    } else {
        Ok(HttpResponse::NotFound().json("Person not found"))
    }
}
