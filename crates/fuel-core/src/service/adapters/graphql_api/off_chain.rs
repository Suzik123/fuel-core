use crate::{
    database::{
        transactions::OwnedTransactionIndexCursor,
        Database,
    },
    fuel_core_graphql_api::{
        database::OffChainView,
        ports::{
            worker,
            OffChainDatabase,
        },
    },
};
use fuel_core_storage::{
    iter::{
        BoxedIter,
        IntoBoxedIter,
        IterDirection,
    },
    not_found,
    transactional::AtomicView,
    Error as StorageError,
    Result as StorageResult,
};
use fuel_core_txpool::types::TxId;
use fuel_core_types::{
    fuel_tx::{
        Address,
        Bytes32,
        TxPointer,
        UtxoId,
    },
    fuel_types::{
        BlockHeight,
        Nonce,
    },
    services::txpool::TransactionStatus,
};
use std::sync::Arc;

impl OffChainDatabase for Database {
    fn owned_message_ids(
        &self,
        owner: &Address,
        start_message_id: Option<Nonce>,
        direction: IterDirection,
    ) -> BoxedIter<'_, StorageResult<Nonce>> {
        self.owned_message_ids(owner, start_message_id, Some(direction))
            .map(|result| result.map_err(StorageError::from))
            .into_boxed()
    }

    fn owned_coins_ids(
        &self,
        owner: &Address,
        start_coin: Option<UtxoId>,
        direction: IterDirection,
    ) -> BoxedIter<'_, StorageResult<UtxoId>> {
        self.owned_coins_ids(owner, start_coin, Some(direction))
            .map(|res| res.map_err(StorageError::from))
            .into_boxed()
    }

    fn tx_status(&self, tx_id: &TxId) -> StorageResult<TransactionStatus> {
        self.get_tx_status(tx_id)
            .transpose()
            .ok_or(not_found!("TransactionId"))?
    }

    fn owned_transactions_ids(
        &self,
        owner: Address,
        start: Option<TxPointer>,
        direction: IterDirection,
    ) -> BoxedIter<StorageResult<(TxPointer, TxId)>> {
        let start = start.map(|tx_pointer| OwnedTransactionIndexCursor {
            block_height: tx_pointer.block_height(),
            tx_idx: tx_pointer.tx_index(),
        });
        self.owned_transactions(owner, start, Some(direction))
            .map(|result| result.map_err(StorageError::from))
            .into_boxed()
    }
}

impl AtomicView<OffChainView> for Database {
    fn view_at(&self, _: BlockHeight) -> StorageResult<OffChainView> {
        unimplemented!(
            "Unimplemented until of the https://github.com/FuelLabs/fuel-core/issues/451"
        )
    }

    fn latest_view(&self) -> OffChainView {
        // TODO: https://github.com/FuelLabs/fuel-core/issues/1581
        Arc::new(self.clone())
    }
}

impl worker::OffChainDatabase for Database {
    fn record_tx_id_owner(
        &mut self,
        owner: &Address,
        block_height: BlockHeight,
        tx_idx: u16,
        tx_id: &Bytes32,
    ) -> StorageResult<Option<Bytes32>> {
        Database::record_tx_id_owner(self, owner, block_height, tx_idx, tx_id)
    }

    fn update_tx_status(
        &mut self,
        id: &Bytes32,
        status: TransactionStatus,
    ) -> StorageResult<Option<TransactionStatus>> {
        Database::update_tx_status(self, id, status)
    }
}
