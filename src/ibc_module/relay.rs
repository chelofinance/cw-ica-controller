#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    DepsMut, Env, IbcBasicResponse, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse,
    Never,
};

use crate::{
    state::{ChannelState, STATE},
    ContractError,
};

/// Handles the `PacketTimeout` for the IBC module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // Due to the semantics of ordered channels, the underlying channel end is closed.
    STATE.update(
        deps.storage,
        |mut contract_state| -> Result<_, ContractError> {
            contract_state.channel_state = ChannelState::Closed;
            Ok(contract_state)
        },
    )?;

    Ok(IbcBasicResponse::default())
}

/// Handles the `PacketReceive` for the IBC module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    _deps: DepsMut,
    _env: Env,
    _msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, Never> {
    // An ICA controller cannot receive packets, so this is a no-op.
    // It must be implemented to satisfy the wasmd interface.
    Ok(IbcReceiveResponse::default())
}
