macro_rules! new_api_error {
    ($ident:ident) => {
        #[derive(Debug, Clone)]
        pub struct $ident {
            description: ::std::borrow::Cow<'static, str>,
        }

        impl ::std::fmt::Display for $ident {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
                write!(fmt, "description: \"{}\"", &self.description)
            }
        }

        impl ::proto::error::ApiError for $ident {
            fn code(&self) -> i32 {
                ::proto::error::ErrorCode::$ident as i32
            }

            fn description(&self) -> ::std::borrow::Cow<'static, str> {
                self.description.clone()
            }
        }

        #[allow(dead_code)]
        impl $ident {  
            pub fn from_str<U>(desc: U) -> Self 
                where U: Into<::std::borrow::Cow<'static, str>>
            {
                Self {
                    description: desc.into()
                }
            }
        }
    }
}

macro_rules! api_error_gen_from_error {
    ($from:ty, $to:ident) => {
        impl From<$from> for $to {
            fn from(raw: $from) -> Self {
                $to {
                    description: ::std::borrow::Cow::from(format!("{}", raw))
                }
            }
        }
    }
}

#[macro_export]
macro_rules! data_into_api_response {
    ($ty:ty) => {
        impl Into<::response::ApiResponse<$ty>> for $ty {
            fn into(self) -> ::response::ApiResponse<$ty> {
                ::response::ApiResponse::Ok {
                    data: self
                }
            }
        }
    }
}