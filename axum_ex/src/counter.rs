use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::sync::{Mutex, Arc};

use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::IntoResponse;

#[derive(Default)]
pub struct Counters(Mutex<HashMap<String, u64>>);

impl Counters {
    pub fn increase(&self, key: &str) {
        let mut map = self.0.lock().unwrap();
        match map.entry(key.to_string()) {
            Entry::Occupied(mut v) => *v.get_mut() += 1,
            Entry::Vacant(v) => {
                v.insert(1);
            }
        }
    }

    pub fn get(&self, key: &str) -> u64 {
        let map = self.0.lock().unwrap();
        map.get(key).copied().unwrap_or(0)
    }
}

pub async fn counter_mw(
    State(counters): State<Arc<Counters>>,
    request: Request,
    next: Next,
) -> impl IntoResponse {
    let uri = request.uri().to_string();
    counters.increase(&uri);
    tracing::info!("call to {} #{}", uri, counters.get(&uri));
    next.run(request).await
}