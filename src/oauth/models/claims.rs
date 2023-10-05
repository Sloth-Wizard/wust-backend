use serde::{Deserialize, Serialize};
///
/// This struct is the construction of the jwt payload to be sent when developping
/// to get full api access easily
/// 
#[derive(Debug, Serialize, Deserialize)]
pub struct DevClaims {
    iss: String,
    sub: String,
    aud: String,
    iat: usize,
    exp: usize,
    pub azp: String,
    scope: String,
    gty: String
}
