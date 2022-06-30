use erased_serde::Serialize as ErasedSerialize;
use serde_derive::Serialize;
// use serde::Serialize;

#[derive(Serialize, Clone, Copy)]
pub struct Response<S>
where
    S: ErasedSerialize + Clone,
{
    pub status: ResponseStatus,
    pub data: S,
}

impl<S> Response<S>
where
    S: ErasedSerialize + Clone,
{
    pub fn new() -> ResponseBuilder<S> {
        ResponseBuilder {
            status: None,
            data: None,
        }
    }
}

pub struct ResponseBuilder<S>
where
    S: ErasedSerialize,
{
    pub status: Option<ResponseStatus>,
    pub data: Option<S>,
}

impl<S> ResponseBuilder<S>
where
    S: ErasedSerialize + Copy + Clone,
{
    pub fn status(&mut self, status: ResponseStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn data(&mut self, data: S) -> &mut Self {
        self.data = Some(data);
        self
    }
    pub fn build(&mut self) -> Response<S> {
        Response {
            status: self.status.unwrap(),
            data: self.data.unwrap(),
        }
    }
}
#[derive(Serialize, Clone, Copy)]

pub enum ResponseStatus {
    #[serde(rename(serialize = "success"))]
    Success,
    #[serde(rename(serialize = "failed"))]
    Failed,
}
#[derive(Serialize, Clone)]
pub struct ResponseError {
    pub code: u32,
    pub details: String,
}

impl ResponseError {
    pub fn new() -> ResponseErrorBuilder {
        ResponseErrorBuilder {
            code: None,
            details: None,
        }
    }
}

#[derive(Serialize)]

pub struct ResponseErrorBuilder {
    pub code: Option<u32>,
    pub details: Option<String>,
}

impl ResponseErrorBuilder {
    pub fn code(&mut self, code: u32) -> &mut Self {
        self.code = Some(code);
        self
    }
    pub fn details(&mut self, details: &str) -> &mut Self {
        self.details = Some(details.to_owned());
        self
    }

    pub fn build(&self) -> Response<ResponseError> {
        Response {
            status: ResponseStatus::Failed,
            data: ResponseError {
                code: self.code.unwrap_or(500),
                details: self.details.clone().unwrap_or( "There's an internal server error happend, Please check 'vns' logs for more details".to_owned()
                   ),
            },
        }.clone()
    }
}

#[test]
async fn test_serialize_error_response() {
    let json = serde_json::to_string_pretty(
        &ResponseError::new()
            .code(500)
            .details("internal server error")
            .build(),
    )
    .unwrap();

    println!("{}", json);
    assert!(true)
}
