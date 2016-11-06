use std::collections::HashMap;
use std::sync::RwLock;
use std::sync::Arc;

use ::proto::schema::*;
use ::proto::error::*;

pub type Token = String;

new_api_error!(SigninError);
new_api_error!(SignupError);
new_api_error!(NotAuthorizedError);

pub struct Authorizer;
impl Authorizer {
    pub fn signin(signin_data: &SigninData) -> ApiResult<&'static Token> {
        unimplemented!()
    }

    pub fn signup(signup_data: &SignupData) -> ApiResult<&'static Token> {
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