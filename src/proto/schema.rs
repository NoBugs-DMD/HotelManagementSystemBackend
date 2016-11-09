use rustc_serialize::json::DecoderError;

new_api_error!(InvalidSchemaError);
new_api_error!(IncompleteDataError);
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

#[derive(Debug, RustcDecodable)]
pub struct UpdateAccountInfoData {
    pub NewName: Option<String>,
    pub NewEmail: Option<String>,
    pub OldPassHash: Option<String>,
    pub NewPassHash: Option<String>,
}