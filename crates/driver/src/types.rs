use std::str::FromStr;

use ethers::{
    prelude::SignerMiddleware,
    providers::{Provider, Ws},
    signers::LocalWallet,
    types::H256,
};
use serde::{Deserialize, Serialize};

/// The [GameType] enum defines the different types of dispute games with cloneable
/// implementations in the `DisputeGameFactory` contract.
#[repr(u8)]
pub enum GameType {
    Fault = 0,
    Validity = 1,
    OutputAttestation = 2,
}

impl TryFrom<u8> for GameType {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameType::Fault),
            1 => Ok(GameType::Validity),
            2 => Ok(GameType::OutputAttestation),
            _ => Err(anyhow::anyhow!("Invalid game type")),
        }
    }
}

/// The [SignerMiddlewareWS] type is a [SignerMiddleware] that uses a [Provider] with a [Ws] transport.
pub(crate) type SignerMiddlewareWS = SignerMiddleware<Provider<Ws>, LocalWallet>;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct OutputAtBlockResponse {
    pub output_root: H256,
}

/// The [ChallengerMode] enum defines the different modes of operation for the challenger.
#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum ChallengerMode {
    /// The challenger will only listen for new games and report
    /// the disputes to the console without sending any transactions.
    ListenOnly,
    /// The challenger will listen for new disputes and respond to them
    /// by sending transactions.
    #[default]
    ListenAndRespond,
}

impl FromStr for ChallengerMode {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "listen-only" => Ok(ChallengerMode::ListenOnly),
            "listen-and-respond" => Ok(ChallengerMode::ListenAndRespond),
            _ => Err(anyhow::anyhow!(
                "Invalid challenger mode. Supported modes are: `listen-only` and `listen-and-respond`"
            )),
        }
    }
}
