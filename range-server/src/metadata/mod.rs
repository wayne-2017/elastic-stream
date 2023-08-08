pub(crate) mod manager;
pub(crate) mod watcher;

use std::rc::Rc;

use model::{error::EsError, range::RangeLifecycleEvent, resource::ResourceEvent};
use pd_client::PlacementDriverClient;
use tokio::sync::mpsc;

#[cfg(any(test, feature = "mock"))]
use mockall::automock;

pub type MetadataListener = mpsc::UnboundedReceiver<ResourceEvent>;

#[cfg_attr(test, automock)]
pub(crate) trait MetadataWatcher {
    fn start<P: PlacementDriverClient + 'static>(&self, pd_client: Rc<P>);

    fn watch(&mut self) -> Result<MetadataListener, EsError>;
}

pub type RangeEventListener = mpsc::UnboundedReceiver<Vec<RangeLifecycleEvent>>;

#[cfg_attr(test, automock)]
pub(crate) trait MetadataManager {
    async fn start(&self);

    fn watch(&mut self) -> Result<RangeEventListener, EsError>;
}