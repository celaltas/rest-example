use crate::helpers::spawn_app;

#[actix_web::test]
async fn buy_pizza_returns_a_200_for_valid_form_data() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name":"Newsletter Title",
    });
    // Act
    let response = app.buy_pizza(body).await;

    //Assert
    assert_eq!(200, response.status().as_u16());
}

#[actix_web::test]
async fn buy_pizza_returns_a_400_when_data_is_missing() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name":"",
    });

    // Act
    let response = app.buy_pizza(body).await;
    assert_eq!(400, response.status().as_u16());
}
