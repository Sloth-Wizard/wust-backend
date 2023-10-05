##
## Y = Base64URLEncode(header) + '.' + Base64URLEncode(payload)
## JWT token = Y + '.' + Base64URLEncode(ECDSASHA256(Y))
##
## Requires openssl, nodejs, jq
header='
{
    "typ": "JWT",
    "kid": "7Q27d_m2HW6HDG12kLdVn",
    "alg": "ES256"
}'
payload='
{
    "iss": "wust",
    "sub": "Ssg691_8s4g4-s4g8dfzg416d54g",
    "aud": "wust-api::jwt::validate",
    "iat": 1650458791,
    "exp": 32513202429,
    "azp": "7ad2fb9f-5eb4-4e72-8a63-47a4e52b1b05",
    "scope": "wust::endpoints",
    "gty": "api-access"
}'

# Clean the string from newlines and spaces
function cleanstr() {
     sed -z 's/\n\| //g'
}

# Encode in base64 and make url safe
function b64enc() {
  base64 | sed 's/\+/-/' | sed -E 's/=+$//'
}

if [ ! -f private-key.pem ]; then
  # Private and Public keys
  # For RSA (hasbeen)
  #
  # openssl genrsa 2048 > private-key.pem
  # openssl rsa -in ./private-key.pem -pubout -out public-key.pem
  #
  # For Elliptic Curve (average crypto enjoyer)
  # secp256r1 has been recently deprecated (still usable but c'mon)
  #
  # This command generates a basic key
  # openssl ecparam -name prime256v1 -genkey -noout -out private-key.pem
  #
  # What we need is that key but of format PKCS#8, so pipe to format it
  openssl ecparam -genkey -noout -name prime256v1 | openssl pkcs8 -topk8 -nocrypt -out ec-priv.pem
  openssl ec -in ec-priv.pem -pubout -out ec-pub.pem
fi

# Clean and encode our header and payload
encoded_header=$(echo -n "${header}" | cleanstr | b64enc)
encoded_payload=$(echo -n "${payload}" | cleanstr | b64enc)

# Concat our header and payload separating by a dot (jwt convention)
encoded_jwt=$(echo -n "${encoded_header}.${encoded_payload}" | cleanstr)

# Prepare our signature
encoded_signature=$(echo -n "${encoded_jwt}" | openssl dgst -sha256 -binary -sign ec-priv.pem | openssl enc -base64 | tr -d '\n=' | tr -- '+/' '-_')

# Assemble our jwt
echo -n "${encoded_jwt}.${encoded_signature}" > jwt.txt

# Create a JWK from our public key
# Use pem-jwk for RSA and eckles for Elliptic Curve
jwk=$(./node_modules/.bin/eckles ec-priv.pem)
# Add additional fields
jwk=$(echo '{"use":"sig"}' $jwk $header | jq -cs add)
# Export JWK
echo '{"keys":['$jwk']}'| jq . > jwks.json

echo "--- JWT ---"
cat jwt.txt
echo -e "\n--- JWK ---"
jq . jwks.json
