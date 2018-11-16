use jsonrpc_core::{IoHandler, Result as JsonRpcResult};

use client::Client;
use primitives::types::TransactionBody;

build_rpc_trait! {
    pub trait TransactionApi {
        /// Receive new transaction
        #[rpc(name = "receive_transaction")]
        fn rpc_receive_transaction(&self, TransactionBody) -> JsonRpcResult<()>;
    }
}

impl TransactionApi for Client {
    fn rpc_receive_transaction(&self, t: TransactionBody) -> JsonRpcResult<()> {
        Ok(self.receive_transaction(&t))
    }
}

pub fn get_handler(client: Client) -> IoHandler {
    let mut io = IoHandler::new();
    io.extend_with(client.to_delegate());
    io
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate jsonrpc_test;
    extern crate primitives;

    use self::jsonrpc_test::Rpc;
    use primitives::types::TransactionBody;

    #[test]
    fn test_call() {
        let handler = get_handler(Client::default());
        let rpc = Rpc::from(handler);
        let t = TransactionBody {
            nonce: 0,
            sender: 1,
            receiver: 0,
            amount: 0,
        };
        assert_eq!(rpc.request("receive_transaction", &[t]), "null");
    }
}
