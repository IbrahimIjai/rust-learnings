use std::sync::{Arc, Mutex};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post, put},
};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;

#[tokio::main]
async fn main() {
    let list = Arc::new(Mutex::new(PersonList { list: Vec::new() }));
    let app = app(list);

    let listner = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listner, app).await.unwrap();
}

fn app(list: Arc<Mutex<PersonList>>) -> axum::Router {
    Router::new()
        .route("/person", get(|| async { "Hello, World!" }))
        .route("/list", get(get_list))
        .route("/person/{id}", get(get_single_person))
        .route("/add_new_person", post(add_new_person))
        .route("/remove_person/{id}", delete(remove_person))
        .route("/update_person/{id}", put(update_person))
        .with_state(list)
}

async fn get_list(State(list): State<Arc<Mutex<PersonList>>>) -> impl IntoResponse {
    let person_list = list.lock().unwrap().clone();
    let json_data = serde_json::to_string_pretty(&person_list.list).unwrap();
    (StatusCode::OK, json_data)
}

async fn get_single_person(
    State(list): State<Arc<Mutex<PersonList>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let person_list = list.lock().unwrap().clone();
    match person_list.list.iter().find(|person| person.id == id) {
        Some(person) => {
            let json_data = to_string_pretty(person).unwrap();
            (StatusCode::OK, json_data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Person not found".to_string()).into_response(),
    }
}

async fn add_new_person(
    State(list): State<Arc<Mutex<PersonList>>>,
    Json(person_request): Json<PersonRequest>,
) -> impl IntoResponse {
    let mut person_list = list.lock().unwrap();
    let new_person = Person {
        id: if person_list.list.len() == 0 {
            0
        } else {
            person_list.list.last().unwrap().id + 1
        },

        name: person_request.name,
        email: "default@example.com".to_string(),
    };
    person_list.list.push(new_person);
    let json_data = serde_json::to_string_pretty(&person_list.list).unwrap();
    (StatusCode::OK, json_data)
}

async fn remove_person(
    State(list): State<Arc<Mutex<PersonList>>>,
    Path(id): Path<u32>,
) -> impl IntoResponse {
    let mut person_list = list.lock().unwrap();
    match person_list.list.iter().position(|person| person.id == id) {
        Some(index) => {
            let removed_person = person_list.list.remove(index);
            let json_data: String = to_string_pretty(&removed_person).unwrap();
            (StatusCode::OK, json_data).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Person not found".to_string()).into_response(),
    }
}

async fn update_person(
    State(list): State<Arc<Mutex<PersonList>>>,
    Path(id): Path<u32>,
    Json(person_request): Json<PersonRequest>,
) -> impl IntoResponse {
    let mut person_list = list.lock().unwrap();
    match person_list.list.iter().find(|person| person.id == id) {
        Some(_person) => {
            let new_list = person_list
                .list
                .iter()
                .map(|person| {
                    if person.id == id {
                        Person {
                            id: person.id,
                            name: person_request.name.clone(),
                            email: person.email.clone(),
                        }
                    } else {
                        person.clone()
                    }
                })
                .collect::<Vec<Person>>();

            person_list.list = new_list;
            (StatusCode::OK, "Person updated".to_string()).into_response()
        }
        None => (StatusCode::NOT_FOUND, "Person not found".to_string()).into_response(),
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
struct Person {
    id: u32,
    name: String,
    email: String,
}

#[derive(Clone)]
struct PersonList {
    list: Vec<Person>,
}

#[derive(Deserialize)]
struct PersonRequest {
    name: String,
}
