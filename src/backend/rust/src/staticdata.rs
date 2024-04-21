use std::{sync::Arc, time::Duration};
use async_channel::{Receiver, Sender};
use tokio::{sync::RwLock, time::sleep};

use crate::{products::{backorder::ProductInBackorder, intransit::ProductInTransit, product::Product}, Category};


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