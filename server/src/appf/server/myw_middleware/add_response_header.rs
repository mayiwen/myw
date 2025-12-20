use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Request},
    middleware::{self, Next},
    response::Response,
};

// 中间件函数，为所有响应添加头
pub async fn i(request: Request<Body>, next: Next) -> Response {
    let mut response = next.run(request).await;
    // ResponseObj.setHeader('Content-Security-Policy', `frame-ancestors 'none'` );
    //       ResponseObj.setHeader('Referrer-Policy', 'no-referrer-when-downgrade' );
    //       ResponseObj.setHeader('X-Content-Type-Options', 'nosniff' );
    //       ResponseObj.setHeader('X-Download-Options', 'noopen' );
    //       ResponseObj.setHeader('X-Frame-Options', 'SAMEORIGIN' );
    //       ResponseObj.setHeader('X-Permitted-Cross-Domain-Policies', 'value' );
    //       ResponseObj.setHeader('X-XSS-Protection', '1;mode=block' );
    let headers = response.headers_mut();
    headers.insert(
        "Content-Security-Policy",
        HeaderValue::from_static("frame-ancestors 'none'"),
    );
    headers.insert(
        "Referrer-Policy",
        HeaderValue::from_static("no-referrer-when-downgrade"),
    );
    headers.insert(
        "X-Content-Type-Options",
        HeaderValue::from_static("nosniff"),
    );
    headers.insert("X-Download-Options", HeaderValue::from_static("noopen"));
    headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
    headers.insert(
        "X-Permitted-Cross-Domain-Policies",
        HeaderValue::from_static("value"),
    );
    headers.insert("X-XSS-Protection", HeaderValue::from_static("1;mode=block"));
    headers.insert("Cache-Control", HeaderValue::from_static("max-age=3600"));

    response
}
