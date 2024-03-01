use crate::helpers::spawn_app;
use rest_example::{database::PizzaDataTrait, domain::pizza::Pizza};
use uuid::Uuid;

#[actix_web::test]
async fn show_pizza_returns_a_204_for_no_record() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let response = app.show_pizzas().await;

    //Assert
    assert_eq!(200, response.status().as_u16());

    // Act
    let pizzas = response
        .json::<Vec<Pizza>>()
        .await
        .expect("Failed request serialized");

    //Assert
    assert_eq!(0, pizzas.len())
}
#[actix_web::test]
async fn show_pizza_returns_a_200_for_with_one_record() {
    // Arrange
    let app = spawn_app().await;
    let test_pizza = Pizza::new(Uuid::new_v4().to_string(), String::from("Italian"));
    let created_record = app.db_client.create_pizza(&test_pizza).await;

    // Act
    let response = app.show_pizzas().await;

    //Assert
    assert_eq!(true, created_record.is_some());
    assert_eq!(200, response.status().as_u16());
}
