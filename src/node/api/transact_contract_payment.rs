use super::messages::{TransactContractPaymentRequest, TransactContractPaymentResponse};
use super::{NodeContext, NodeError};
use crate::blockchain::{Blockchain, TransactionStats};
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn transact_contract_payment<B: Blockchain>(
    context: Arc<RwLock<NodeContext<B>>>,
    req: TransactContractPaymentRequest,
) -> Result<TransactContractPaymentResponse, NodeError> {
    let mut context = context.write().await;
    let now = context.network_timestamp();
    // Prevent spamming mempool
    if context.blockchain.validate_contract_payment(&req.tx)? {
        context
            .contract_payment_mempool
            .insert(req.tx, TransactionStats { first_seen: now });
    }
    Ok(TransactContractPaymentResponse {})
}
