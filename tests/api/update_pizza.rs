use crate::helpers::spawn_app;
use rest_example::domain::pizza::Pizza;

#[actix_web::test]
async fn update_pizza_returns_a_404_for_no_record() {
    // Arrange
    let app = spawn_app().await;
    let fake_uuid = "1234sadasd".to_string();

    // Act
    let response = app.update_pizza(fake_uuid).await;

    //Assert
    assert_eq!(404, response.status().as_u16());
}
#[actix_web::test]
async fn update_pizza_returns_a_200_for_fake_record() {
    // Arrange
    let app = spawn_app().await;
    let body = serde_json::json!({
        "name":"Mexico",
    });

    let response = app.buy_pizza(body).await;
    assert_eq!(200, response.status().as_u16());

    let new_pizza = response
        .json::<Pizza>()
        .await
        .expect("Failed request serialized");

    // Act
    let response = app.update_pizza(new_pizza.uuid).await;
    assert_eq!(200, response.status().as_u16());

    let new_pizza = response
        .json::<Pizza>()
        .await
        .expect("Failed request serialized");

    //Assert
    assert_eq!("Sold".to_owned(), new_pizza.name);
}
