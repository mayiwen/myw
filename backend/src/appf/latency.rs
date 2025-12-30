use axum::http::Response;
use std::fmt::{Display, Formatter};
use std::time::Duration;
use tower_http::trace::OnResponse;
use tracing::Span;

#[derive(Debug, Clone, Copy)]
pub struct LatencyOnResponse;

impl<B> OnResponse<B> for LatencyOnResponse {
    fn on_response(self, response: &Response<B>, latency: Duration, _span: &Span) {
        tracing::info!(
        latency = %Latency(latency),
         status = response.status().as_u16(),
       "finished processing request" );
    }
}

struct Latency(Duration);

impl Display for Latency {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.0.as_millis() > 0 {
            write!(f, "{} 毫秒", self.0.as_millis())
        } else {
            write!(f, "{} 微秒", self.0.as_millis())
        }
    }
}
