use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{accounts::AccountRepository, api::ApiErrorKind};



#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlayerInfoParam {
    id: u32,
}

#[derive(Error, Debug)]
pub(super) enum PlayerError {
    #[error("player not found")]
    NotFound,
}

impl ApiErrorKind for PlayerError {
    fn kind(&self) -> &'static str {
        match self {
            PlayerError::NotFound => "PLAYER_NOT_FOUND",
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct PlayerInfoResponse {
    id: u32,
    name: Option<String>,
    gen: i32,
    xp: i32,
    active_calling_card_index: i32,
    active_callsign_icon_index: i32,
    active_callsign_icon_style_index: i32,
    net_worth: i32,
}

pub(super) async fn player_info(
    param: PlayerInfoParam,
    accounts: AccountRepository,
) -> Result<PlayerInfoResponse, PlayerError> {
    let raw_data = accounts.get_data(param.id).await.map_err(|_| PlayerError::NotFound)?;
    let player_data = player_data::from_u8(&raw_data).unwrap();

    Ok(PlayerInfoResponse {
        id: param.id,
        name: accounts.get_name(param.id).await.unwrap(),
        gen: player_data.gen,
        xp: player_data.xp,
        active_calling_card_index: player_data.activeCallingCardIndex,
        active_callsign_icon_index: player_data.activeCallsignIconIndex,
        active_callsign_icon_style_index: player_data.activeCallsignIconStyleIndex,
        net_worth: player_data.netWorth,
    })
}
