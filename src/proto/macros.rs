macro_rules! new_api_error {
    ($ident:ident) => {
        #[derive(Debug, Clone)]
        pub struct $ident {
            description: String,
        }

        impl ::std::fmt::Display for $ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                write!(fmt, "{}", self.description())
            }
        }

        impl ApiError for $ident {
            fn code(&self) -> i32 {
                ErrorCode::$ident.into()
            }
        }

        impl ::std::error::Error for $ident {
            fn description(&self) -> &str {
                &self.description
            }
        }

        impl Into<IronError> for $ident {
            fn into(self) -> IronError {
                let mut response = Response::with(StatusCode::Forbidden);
                response.body = Some(box self.json());
                
                IronError {
                    error: box self,
                    response: response
                }
            }
        }

        #[allow(dead_code)]
        impl $ident {  
            pub fn from_str<U>(desc: U) -> Self 
                where U: Into<::std::borrow::Cow<'static, str>> + Display
            {
                Self {
                    description: format!("{}", &desc)
                }
            }
        }
    }
}

macro_rules! api_error_gen_from_error {
    ($from:ty, $to:ident) => {
        impl From<$from> for $to {
            fn from(raw: $from) -> Self {
                Self::from_str(raw.description().to_owned())
            }
        }
    }
}