// 导入pulsar包
use pulsar::{Authentication, Error as PulsarError, Pulsar, TokioExecutor};

pub struct PulsarConf<'a> {
    addr: &'a str,          // pulsar address
    token: Option<&'a str>, // token optional param
}

impl<'a> PulsarConf<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self {
            addr: addr,
            token: None,
        }
    }

    pub fn with_token(mut self, token: &'a str) -> Self {
        self.token = Some(token);
        self
    }

    // create client pulsar client
    pub async fn client(&self) -> Result<Pulsar<TokioExecutor>, PulsarError> {
        let mut builder = Pulsar::builder(self.addr.to_string(), TokioExecutor);
        if let Some(token) = self.token {
            let authentication = Authentication {
                name: "token".to_string(),
                data: token.to_string().into_bytes(),
            };

            builder = builder.with_auth(authentication);
        }

        let client = builder.build().await;
        client
    }
}
