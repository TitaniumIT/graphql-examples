use std::{sync::Arc, time::Duration};
use tokio::{sync::{broadcast::{Receiver, Sender}, RwLock}, time::sleep};

use crate::model::{
    backorder::ProductInBackorder, categorie::Category, intransit::ProductInTransit,
    product::Product,
};

pub struct StaticData {
    pub products: Arc<RwLock<Vec<Product>>>,
    pub categories: Arc<RwLock<Vec<Category>>>,
    pub products_in_transit: Arc<RwLock<Vec<ProductInTransit>>>,
    pub products_in_backorders: Arc<RwLock<Vec<ProductInBackorder>>>,
    pub status_channel: (Sender<ProductInTransit>, Receiver<ProductInTransit>),
}

impl StaticData {
    pub async fn categories(&self) -> Vec<Category> {
        sleep(Duration::from_millis(1000)).await;
        self.categories.read().await.to_vec()
    }
}
