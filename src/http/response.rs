use serde::Serialize;
#[derive(Serialize)]
pub struct ResponseError {
    pub error: String,
    pub code: i32,
}

impl ResponseError {
    pub fn new(error: String, code: i32) -> Self {
        Self { error, code }
    }

    pub fn new_response(error: String, code: i32) -> Response<Self> {
        Response::<Self>::new("error", Self { error, code })
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
