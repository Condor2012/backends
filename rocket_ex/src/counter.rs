use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Default, Clone)]
pub struct CounterMiddleware {
    counters: Arc<Counters>,
}

#[rocket::async_trait]
impl Fairing for CounterMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Counter middleware",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, req: &mut Request<'_>, _data: &mut Data<'_>) {
        let addr = req
            .client_ip()
            .map(|addr| addr.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        self.counters.increase(&addr);
        let count = self.counters.get(&addr);
        log::info!("It's your {count} request");
    }
}

#[derive(Default)]
struct Counters(Mutex<HashMap<String, u64>>);

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
