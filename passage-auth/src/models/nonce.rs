use serde::{Deserialize, Serialize};

/// Nonce : the nonce to exchange for an authentication token
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Nonce {
    #[serde(rename = "nonce")]
    pub nonce: String,
}

impl Nonce {
    /// the nonce to exchange for an authentication token
    pub fn new(nonce: String) -> Nonce {
        Nonce { nonce }
    }
}
