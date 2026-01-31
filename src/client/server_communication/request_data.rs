use std::sync::Arc;
use crate::client::server_communication::{exercise_communicator, user_communicator};
use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::user_mod::user::UserInformation;

#[derive(Debug)]
pub struct LoginServerRequestData {
    pub user_information: UserInformation,
    pub exercises: Vec<Exercise>,
    //foreign_users: Vec<ForeignUsers>,
    //mascot_data: ...
}
pub async fn request_login_data(username: String) -> Result<Arc<LoginServerRequestData>, ServerRequestError> {
    let exercise_data = exercise_communicator::get_exercise_data_from_server(username.clone()).await?;
    let user_information = user_communicator::get_user_information_from_server(username).await?;

    Ok(Arc::new(LoginServerRequestData {
        exercises: exercise_data,
        user_information,
    }))
}
