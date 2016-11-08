use rustc_serialize::json::DecoderError;

new_api_error!(InvalidSchemaError);
api_error_gen_from_error!(DecoderError, InvalidSchemaError);

#[derive(Debug, RustcDecodable)]
pub struct SigninData {
    pub login: String,
    pub pass_md5: String,
}

#[derive(Debug, RustcDecodable)]
pub struct SignupData {
    pub login: String,
    pub email: String,
    pub pass_md5: String,
}

#[derive(Debug, Copy, Clone, RustcEncodable)]
pub struct Roles {
    pub client: bool,
    pub owner: bool,
    pub manager: bool,
    pub cleaner: bool,
    pub receptionist: bool
}