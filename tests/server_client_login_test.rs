mod common;

use crate::common::setups::logged_in_setup;

#[tokio::test]
async fn login_test() {
    let (_app, shutdown) = logged_in_setup().await;
    shutdown.send(()).unwrap();
}
