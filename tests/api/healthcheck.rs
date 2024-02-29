use crate::helpers::spawn_app;

#[actix_web::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();
    let url = format!("{}/v1/healthcheck", &app.address);

    // Act
    let response = client
        .get(&url)
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert_eq!(200, response.status().as_u16());
    assert_eq!(Some(0), response.content_length());
}
