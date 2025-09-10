use serde::{Deserialize, Serialize};



#[derive(Deserialize, Serialize)]
pub struct GetNonceDTO {
    pub address:String
}