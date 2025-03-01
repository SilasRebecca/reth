use crate::blobstore::{BlobStore, BlobStoreError, BlobTransactionSidecar};
use reth_primitives::H256;

/// A blobstore implementation that does nothing
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Default)]
#[non_exhaustive]
pub struct NoopBlobStore;

impl BlobStore for NoopBlobStore {
    fn insert(&self, _tx: H256, _data: BlobTransactionSidecar) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn insert_all(&self, _txs: Vec<(H256, BlobTransactionSidecar)>) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn delete(&self, _tx: H256) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn delete_all(&self, _txs: Vec<H256>) -> Result<(), BlobStoreError> {
        Ok(())
    }

    fn get(&self, _tx: H256) -> Result<Option<BlobTransactionSidecar>, BlobStoreError> {
        Ok(None)
    }

    fn get_all(
        &self,
        _txs: Vec<H256>,
    ) -> Result<Vec<(H256, BlobTransactionSidecar)>, BlobStoreError> {
        Ok(vec![])
    }

    fn data_size_hint(&self) -> Option<usize> {
        Some(0)
    }
}
