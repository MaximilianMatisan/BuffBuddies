use buff_buddies::client::gui::bb_tab::login::LoginMessage;
use buff_buddies::client::gui::bb_tab::tab::Tab;
use buff_buddies::client::gui::user_interface::{App, Message};
use buff_buddies::client::server_communication::request_data::request_login_data;
use buff_buddies::client::server_communication::user_communicator::valid_register;
use buff_buddies::server::database_mod::database::setup_test_db;
use buff_buddies::server::server_main::create_app;
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio::sync::oneshot;

#[allow(dead_code)]
pub fn client_setup() -> App {
    App::default()
}

//sender to kill server once a test is finished so it doesn't run forever
#[allow(dead_code)]
pub fn server_setup() -> oneshot::Sender<()> {
    let (sender, receiver) = oneshot::channel();

    std::thread::spawn(|| {
        let runtime = tokio::runtime::Runtime::new().unwrap();
        runtime.block_on(server_main_test(receiver));
    });
    sender
}

///creates a new server for testing
///takes oneshot Receiver as input which is used to shut down the server at the end of the test
///otherwise it wouldn't ever terminate
///database is memory only to not affect the real one
#[allow(dead_code)]
pub async fn server_main_test(shutdown: oneshot::Receiver<()>) {
    let pool = setup_test_db().await;
    setup_test_mascots(&pool).await;
    //TODO add test exercises cause memory only database starts empty
    let app = create_app(pool);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind tcp listener");
    axum::serve(listener, app)
        .with_graceful_shutdown(async {
            shutdown.await.ok();
        })
        .await
        .expect("failed to start server")
}

///adds all mascots to database used for testing
pub async fn setup_test_mascots(pool: &SqlitePool) {
    for mascot in [
        "Duck",
        "Capybara",
        "Chameleon",
        "Dog",
        "Reindeer",
        "Shark",
        "Whale",
    ] {
        setup_mascot(pool, mascot).await;
    }
}

///adds mascot to database used for testing
pub async fn setup_mascot(pool: &SqlitePool, mascot: &str) {
    sqlx::query("INSERT INTO mascot (mascot_name, description) VALUES (?, ?)")
        .bind(mascot)
        .bind("test")
        .execute(pool)
        .await
        .expect("Mascot insert failed");
}

///returns a setup which has server and client and has the client logged in
///client username is: "12345"
///client password is: "1234"
#[allow(dead_code)]
pub async fn logged_in_setup() -> (App, oneshot::Sender<()>) {
    let mut app = client_setup();
    let shutdown = server_setup();
    let _ = app.update(Message::Login(LoginMessage::PasswordEntered(
        "1234".to_string(),
    )));
    let _ = app.update(Message::Login(LoginMessage::UsernameEntered(
        "12345".to_string(),
    )));
    let jwt = valid_register(app.login_state.try_login().unwrap())
        .await
        .unwrap();
    app.jsonwebtoken = Some(jwt);
    app.screen = Tab::Loading;
    let user_data = request_login_data(app.jsonwebtoken.clone()).await.unwrap();
    match Arc::try_unwrap(user_data) {
        Ok(data) => {
            app.update_app_on_login(data);
        }
        Err(_) => app.login_state.error_text = "Internal error: Arc".to_string(),
    }
    assert_eq!(app.user_manager.user_info.username, "12345".to_string());
    app.login_if_fetching_login_data_successful();
    (app, shutdown)
}
