macro_rules! decode_body {
    ($req:ident) => (     
        match json::decode(&request_body($req)) {
            Ok(obj) => obj,
            Err(err) => {
                return Ok(InvalidSchemaError::from(err).into_api_response().into())
            }
        }
    );
}

macro_rules! authorize {
    ($conn:ident, $req:ident) => (
        match Authorizer::authorize_request(&$conn, $req) {
            Ok(client) => client,
            Err(err) => return Ok(err.into_api_response().into()),
        };
    )
}