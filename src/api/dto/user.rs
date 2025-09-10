use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize)]
pub struct GetNonceDTO {
    pub address:String
}

#[derive(Deserialize, Serialize)]
pub struct VerifySignatureDTO {
    pub address: String,
    pub signature: String,
}