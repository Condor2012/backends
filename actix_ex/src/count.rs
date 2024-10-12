use std::collections::HashMap;
use std::future::{ready, Ready};
use std::sync::{Arc, Mutex};

use actix_web::dev::{self, Service, ServiceRequest, Transform};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
#[derive(Default, Clone)]
pub struct CountersTransform {
    counters: Arc<Counters>,
}

// Middleware factory is `Transform` trait from actix-service crate
impl<S: Service<ServiceRequest>> Transform<S, ServiceRequest> for CountersTransform {
    type Response = S::Response;
    type Error = S::Error;
    type Transform = CountersMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(CountersMiddleware {
            counters: self.counters.clone(),
            service,
        }))
    }
}

pub struct CountersMiddleware<S> {
    counters: Arc<Counters>,
    service: S,
}

impl<S> Service<ServiceRequest> for CountersMiddleware<S>
where
    S: Service<ServiceRequest>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let addr = req
            .peer_addr()
            .map(|a| a.ip().to_string())
            .unwrap_or_else(|| "unknown".to_string());
        self.counters.increase(&addr);
        let count = self.counters.get(&addr);
        log::info!("It's your {count} request");

        self.service.call(req)
    }
}

#[derive(Default)]
struct Counters(Mutex<HashMap<String, u64>>);

impl Counters {
    pub fn increase(&self, key: &str) {
        let mut map = self.0.lock().unwrap();
        *map.entry(key.to_string()).or_default() += 1;
    }

    pub fn get(&self, key: &str) -> u64 {
        let map = self.0.lock().unwrap();
        map.get(key).copied().unwrap_or(0)
    }
}
