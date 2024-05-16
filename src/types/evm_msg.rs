//! This module defines types for EVM chains

use cosmwasm_schema::cw_serde;
use ethabi::Token;

///EVM transaction
#[cw_serde]
pub struct Transaction {
    ///value of the transaction
    pub value: u128,
    ///calldata of the transaction
    pub data: String,
    ///address where the transaction goes
    pub target: String,
}

///EVM message def
#[cw_serde]
pub struct EVMMessage {
    ///messages of the transaction
    pub messages: Vec<Transaction>,
}

impl Into<Token> for &Transaction {
    fn into(self) -> Token {
        Token::Tuple(vec![
            Token::Uint(self.value.into()),
            Token::String(self.data.clone()),
            Token::String(self.target.clone()),
        ])
    }
}

impl EVMMessage {
    ///ABI encode an EVM message
    pub fn encode(&self) -> Vec<u8> {
        let txs_tuples: Vec<Token> = self.messages.iter().map(|msg| msg.into()).collect();
        ethabi::encode(&[Token::Array(txs_tuples)])
    }
}
