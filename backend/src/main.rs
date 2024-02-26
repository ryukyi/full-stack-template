use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tokio_postgres::{NoTls, Error as PgError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use dotenv::dotenv;
use std::cell::RefCell;
use std::env;
use std::error::Error;
use std::fmt;
use log::{info, error};
use env_logger;

#[derive(Debug, Serialize, Deserialize)]
struct Meal {
    id: i32,
    name: String,
    description: String,
    price: f64,
}

#[derive(Debug)]
enum AppError {
    DatabaseError(PgError),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::DatabaseError(ref err) => write!(f, "Database error: {}", err),
            AppError::IoError(ref err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for AppError {}

impl From<PgError> for AppError {
    fn from(err: PgError) -> AppError {
        AppError::DatabaseError(err)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> AppError {
        AppError::IoError(err)
    }
}

// Update the list_meals function signature to match the Arc-wrapped client
async fn list_meals(db_pool: web::Data<Arc<tokio_postgres::Client>>) -> impl Responder {
    let query = "SELECT id, name, description, price FROM meals";
    match db_pool.query(query, &[]).await {
        Ok(rows) => {
            let meals: Vec<Meal> = rows.iter().map(|row| Meal {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
                price: row.get("price"),
            }).collect();
            HttpResponse::Ok().json(meals)
        },
        Err(e) => {
            eprintln!("Failed to dexecute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

// Add orders
async fn insert_order(
    db_pool: web::Data<Arc<tokio_postgres::Client>>,
    user_id: i32,
    meal_id: i32,
    quantity: i32,
) -> impl Responder {
    let query = "
        INSERT INTO orders (user_id, meal_id, quantity, total_price)
        SELECT 
            $1 AS user_id, 
            $2 AS meal_id, 
            $3 AS quantity, 
            ($3 * meals.price) AS total_price
        FROM meals
        WHERE meals.id = $2
        RETURNING id;
    ";

    match db_pool.query_one(query, &[&user_id, &meal_id, &quantity]).await {
        Ok(row) => {
            let order_id: i32 = row.get(0);
            HttpResponse::Ok().json(order_id) // Respond with the ID of the newly inserted order
        },
        Err(e) => {
            eprintln!("Failed to insert order: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

fn create_database_url() -> String {
    dotenv().ok();

    let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
    let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
    let postgres_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
    let postgres_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
    format!("postgres://{}:{}@{}:{}/{}", postgres_user, postgres_password, postgres_host, postgres_port, postgres_db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let dburl = create_database_url();
    // Connect to the database
    let (client, conn) = tokio_postgres::connect(&dburl, NoTls).await.expect("Failed to connect to database");

    // Wrap the client in an Arc for shared ownership
    let client = Arc::new(client);

    // Spawn a separate task to handle the database connection
    tokio::spawn(async move {
        if let Err(e) = conn.await {
            error!("Connection error: {}", e);
        }
    });

    info!("Starting the Actix Web server with the database client...");

    // Start the Actix Web server with the database client
    // Update the Data wrapping in the HttpServer::new closure
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone())) // Directly pass the Arc-wrapped client
            .route("/meals", web::get().to(list_meals))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
    }