use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;

use ::schema::*;

pub type Token = String;
pub type SigninResult<'tmap> = Result<&'tmap Token, SigninError>; 
pub type SignupResult<'tmap> = Result<&'tmap Token, SignupError>;

new_api_error!(SigninError);
new_api_error!(SignupError);

pub struct Authorizer;
impl Authorizer {
    pub fn signin(signin_data: &SigninData) -> SigninResult<'static> {
        unimplemented!()
    }

    pub fn signup(signup_data: &SignupData) -> SignupResult<'static> {
        unimplemented!()
    }
}

lazy_static! {
    static ref TOKEN_MAP: TokenMap = TokenMap::new();
}

struct TokenMap {
    map: Arc<RwLock<HashMap<Token, i32>>>
}

impl TokenMap {
    pub fn new() -> TokenMap {
        TokenMap {
            map: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn get<U: AsRef<Token>>(&self, token: U) -> Option<i32> {
        self.map.read().unwrap().get(token.as_ref()).cloned()
    }

    pub fn put(&self, token: Token, id: i32) {
        self.map.write().unwrap().insert(token, id);
    }
}