use attendance_service::schema::period::Period;

use crate::common::{
    app::spawn_app,
    helpers::{create_period, user_logged},
};

mod common;

#[tokio::test]
async fn test_create_period() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let response = create_period(&app.server, &token).await;
    response.assert_status_success();
}

#[tokio::test]
async fn test_delete_period() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;
    let new_period = create_period(&app.server, &token).await;
    new_period.assert_status_success();

    let period_id = new_period.json::<Period>().id.to_string();
    let response = app
        .server
        .delete(&format!("/period/{}", period_id))
        .authorization_bearer(&token)
        .await;
    response.assert_status_success();
}

#[tokio::test]
async fn test_get_periods() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let new_period = create_period(&app.server, &token).await;
    new_period.assert_status_success();

    let response = app.server.get("/period").authorization_bearer(&token).await;
    response.assert_status_success();
}
