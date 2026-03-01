mod common;

use crate::common::setups::server_setup;

#[test]
fn test_server_start() {
    let shutdown = server_setup();
    shutdown.send(()).unwrap();
}
