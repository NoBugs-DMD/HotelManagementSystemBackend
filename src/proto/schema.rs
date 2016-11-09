use rustc_serialize::json::DecoderError;

new_api_error!(InvalidSchemaError);
api_error_gen_from_error!(DecoderError, InvalidSchemaError);

#[derive(Debug, RustcDecodable)]
pub struct SigninData {
    pub Login: String,
    pub PassHash: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SignupData {
    pub Login: String,
    pub Name: String,
    pub Email: String,
    pub PassHash: String,
}

#[derive(Debug, Copy, Clone, RustcEncodable)]
pub struct Roles {
    pub Client: bool,
    pub Owner: bool,
    pub Manager: bool,
    pub Cleaner: bool,
    pub Receptionist: bool,
}