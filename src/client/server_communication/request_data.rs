use crate::client::server_communication::exercise_communicator::ServerRequestError;
use crate::client::server_communication::{exercise_communicator, mascot_communicator, user_communicator};
use crate::common::exercise_mod::exercise::Exercise;
use crate::common::user_mod::user::UserInformation;
use std::sync::Arc;
use crate::common::mascot_mod::mascot_data_transfer::MascotDataServerClientTransfer;

#[derive(Debug)]
pub struct LoginServerRequestData {
    pub user_information: UserInformation,
    pub exercises: Vec<Exercise>,
    pub mascot_data: MascotDataServerClientTransfer,
    //foreign_users: Vec<ForeignUsers>,
}
pub async fn request_login_data(
    jwt: Option<String>,
) -> Result<Arc<LoginServerRequestData>, ServerRequestError> {
    if let Some(jwt_string) = jwt {
        let exercises =
            exercise_communicator::get_exercise_data_from_server(jwt_string.clone()).await?;
        let user_information =
            user_communicator::get_user_information_from_server(jwt_string.clone()).await?;
        let mascot_data = mascot_communicator::get_mascot_data_from_server(jwt_string).await?;

        Ok(Arc::new(LoginServerRequestData {
            exercises,
            user_information,
            mascot_data,
        }))
    } else {
        Err(ServerRequestError::NoJWTValidation)
    }
}
