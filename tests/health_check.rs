// use zero2prod::main;
use std::net::TcpListener;
use sqlx::{PgConnection, Connection};

use zero2prod::startup;
use zero2prod::configuration::get_configuration;

fn spawn_app() -> String {

    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind random port");
    
    let port = listener.local_addr().unwrap().port();

    let server = startup::run(listener).expect("Failed to vind address");
    
    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
    

#[tokio::test]
async fn health_check_works(){

    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    
    // Act
    let res = client
        .get(&format!("{}/health_check", &address))
        .send()
        .await
        .expect("failed to execute request.");
    
    // Assert
    assert!(res.status().is_success());
    assert_eq!(Some(0), res.content_length());
}

#[tokio::test]
async fn subscribe_returns_200_when_valid_data() {

    // Arrange
    let address = spawn_app();
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let mut connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");
    let client = reqwest::Client::new();
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";

    // Act
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type","application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    // Assert
    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&mut connection)
        .await
        .expect("Failed to fetch saved subscriptions.");

    assert_eq!(saved.email,"ursula_le_guin@gmail.com");
    assert_eq!(saved.name, "le guin");

} 

#[tokio::test]
async fn subscribe_returns_400_when_invalid_data(){

    // Arrange
    let address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];

    for (body, error_message) in test_cases {
        // Act
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type","application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not faild with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}