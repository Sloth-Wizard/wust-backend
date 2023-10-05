/// Backend to backend impl
/// 
/// How to build our JWT
/// 
/// Head to the private repository and read instructions there.
/// If you don't have it, ask the wizard
/// 
/// To test with auth jwt
/// ```
/// curl -H "Authorization: Bearer <_JWT_TOKEN_>" 127.0.0.1:3000/api/series
/// ```
/// 
use dotenv;

use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation, Header, TokenData};
use hyper::header::AUTHORIZATION;
use hyper::{Body, Request};

use crate::oauth::models::claims::DevClaims as Claims; // Only for dev purposes, token authorization must be stricter that that

const __PUBLIC_KEY: &[u8] = include_bytes!("./pem/ec-pub.pem"); // Ask the wizard for the key

///
/// Validate the token given in the authorization header
/// Here we validate the type AND the token given
/// 
/// If this function returns an error, the api access isn't given
/// 
/// ```
/// req -> the incomming Request<Body>
/// ```
/// 
pub async fn validate(req: &Request<Body>) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    if !req.headers().get(AUTHORIZATION).is_none() {
        // Split the sent value in 2
        let auth_header: Vec<&str> = req.headers().get(AUTHORIZATION)
            .unwrap()
            .to_str()
            .unwrap()
            .split(" ")
            .filter(
                |s| !s.is_empty()
            )
            .collect();

        // Check if the token uses the right authorization token type
        if auth_header[0] == &*dotenv::var("API_AUTH").unwrap() {
            let header: Header = decode_header(auth_header[1])?;
            match header.kid { // Do we have a matching kid in our header token ?
                Some(_) => (),
                None => return Err("Token doesn't have a `kid` header field".into())
            };

            // Final matching stage, if all goes well, we spit out a TokenData struct to our handler
            decode_token(auth_header[1]).await   
        } else {
            return Err("Wrong type of authorization".into())
        }
    } else {
        return Err("No authorization found".into())
    }
}

///
/// Decode and verify the signature of the passed jwt token
/// 
/// ```
/// token -> a JWT formatted token
/// ```
/// 
async fn decode_token(token: &str) -> Result<TokenData<Claims>, Box<dyn std::error::Error>> {
    let token_data = match decode::<Claims>(
        token,
        &DecodingKey::from_ec_pem(__PUBLIC_KEY).unwrap(),
        &Validation::new(Algorithm::ES256)
    ) {
        Ok(c) => c,
        Err(err) =>  match *err.kind() {
            ErrorKind::InvalidToken => return Err("Invalid token given".into()),
            ErrorKind::InvalidSignature => return Err("Bad signature".into()),
            _ => unreachable!("Should be an error but none found")
        } 
    };

    Ok(token_data)
}
