use serde::Serialize;
#[derive(Serialize, Clone)]
pub struct ResponseError {
    pub cause: String,
    pub code: i32,
}

impl ResponseError {
    pub fn new(cause: &str, code: i32) -> Self {
        Self {
            cause: cause.to_owned(),
            code,
        }
    }

    pub fn new_response(cause: &str, code: i32) -> Response<Self> {
        Response::<Self>::new(
            "error",
            Self {
                cause: cause.to_owned(),
                code,
            },
        )
    }
}

#[derive(Serialize)]
pub struct Response<S>
where
    S: Serialize,
{
    pub status: String,
    pub data: S,
}

impl<S> Response<S>
where
    S: Serialize,
{
    pub fn new(status: &str, data: S) -> Response<S> {
        Self {
            status: status.to_owned(),
            data,
        }
    }
}
