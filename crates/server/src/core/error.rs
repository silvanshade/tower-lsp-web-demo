use crate::core;
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    #[error("ClientNotInitialzed")]
    ClientNotInitialized,
    #[error("core::SessionResourceNotFound: kind={kind:?}, uri={uri:?}")]
    SessionResourceNotFound {
        kind: core::session::SessionResourceKind,
        uri: lsp::Url,
    },
}

pub struct IntoJsonRpcError(pub anyhow::Error);

impl From<IntoJsonRpcError> for tower_lsp::jsonrpc::Error {
    fn from(error: IntoJsonRpcError) -> Self {
        let mut rpc_error = tower_lsp::jsonrpc::Error::internal_error();
        rpc_error.data = Some(serde_json::to_value(format!("{}", error.0)).unwrap());
        rpc_error
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, IntoJsonRpcError};

    #[test]
    fn from() {
        let error = Error::ClientNotInitialized;
        let error = error.into();

        let mut expected = tower_lsp::jsonrpc::Error::internal_error();
        expected.data = Some(serde_json::to_value(format!("{}", error)).unwrap());

        let actual: tower_lsp::jsonrpc::Error = IntoJsonRpcError(error).into();

        assert_eq!(expected, actual);
    }
}
