#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, DepsMut, Env, IbcBasicResponse, IbcPacketAckMsg, IbcPacketReceiveMsg,
    IbcPacketTimeoutMsg, IbcReceiveResponse, Never,
};

use crate::types::{state::CHANNEL_STATE, ContractError};

use super::types::packet::acknowledgement::AcknowledgementData;

/// Implements the IBC module's `OnAcknowledgementPacket` handler.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    deps: DepsMut,
    _env: Env,
    ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // This lets the ICA controller know whether or not the sent transactions succeeded.
    match from_binary(&ack.acknowledgement.data)? {
        AcknowledgementData::Result(_resp) => ibc_packet_ack::success(deps),
        AcknowledgementData::Error(_err) => ibc_packet_ack::error(deps),
    }
}

/// Handles the `PacketTimeout` for the IBC module.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    _msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // Due to the semantics of ordered channels, the underlying channel end is closed.
    CHANNEL_STATE.update(
        deps.storage,
        |mut channel_state| -> Result<_, ContractError> {
            channel_state.close();
            Ok(channel_state)
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
    unreachable!("ICA controller cannot receive packets")
}

mod ibc_packet_ack {
    use crate::types::state::CALLBACK_COUNTER;

    use super::*;

    /// Handles the successful acknowledgement of an ica packet. This means that the
    /// transaction was successfully executed on the host chain.
    pub fn success(deps: DepsMut) -> Result<IbcBasicResponse, ContractError> {
        // Handle the success case. You need not deserialize the response.
        CALLBACK_COUNTER.update(deps.storage, |mut counter| -> Result<_, ContractError> {
            counter.success();
            Ok(counter)
        })?;
        Ok(IbcBasicResponse::default())
    }

    /// Handles the unsuccessful acknowledgement of an ica packet. This means that the
    /// transaction failed to execute on the host chain.
    pub fn error(deps: DepsMut) -> Result<IbcBasicResponse, ContractError> {
        // Handle the error.
        CALLBACK_COUNTER.update(deps.storage, |mut counter| -> Result<_, ContractError> {
            counter.error();
            Ok(counter)
        })?;
        Ok(IbcBasicResponse::default())
    }
}