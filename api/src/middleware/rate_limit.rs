//! Sprint 168: Per-store rate limiting using governor token bucket.
//! Key: store_id (NOT IP) — 10 devices per store behind same router.

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use dashmap::DashMap;
use governor::{Quota, RateLimiter, clock::DefaultClock, state::{InMemoryState, NotKeyed}};
use std::num::NonZeroU32;
use std::sync::Arc;

use crate::middleware::tenant::TenantContext;

/// Per-store rate limiter using governor's token bucket.
/// DashMap provides lock-free concurrent access by store_id.
type DirectLimiter = RateLimiter<NotKeyed, InMemoryState, DefaultClock>;
pub type StoreRateLimiters = Arc<DashMap<i32, DirectLimiter>>;

/// Default: 100 requests per second per store
const DEFAULT_RATE_PER_SECOND: u32 = 100;

/// Create a new StoreRateLimiters instance.
pub fn new_store_limiters() -> StoreRateLimiters {
    Arc::new(DashMap::new())
}

/// Axum middleware: rate limit by store_id from TenantContext.
/// Returns 429 Too Many Requests when store exceeds quota.
pub async fn rate_limit_middleware(
    Extension(ctx): Extension<TenantContext>,
    State(limiters): State<StoreRateLimiters>,
    request: Request,
    next: Next,
) -> Response {
    let store_id = ctx.store_id;

    // Get or create limiter for this store
    let limiter = limiters
        .entry(store_id)
        .or_insert_with(|| {
            RateLimiter::direct(
                Quota::per_second(NonZeroU32::new(DEFAULT_RATE_PER_SECOND).unwrap())
            )
        });

    match limiter.check() {
        Ok(_) => next.run(request).await,
        Err(_) => {
            tracing::warn!(
                "🚫 Rate limit exceeded: store_id={}, quota={}/s",
                store_id, DEFAULT_RATE_PER_SECOND
            );
            (
                StatusCode::TOO_MANY_REQUESTS,
                [("Retry-After", "1")],
                "Rate limit exceeded. Please slow down.",
            ).into_response()
        }
    }
}
