use crate::appf::auth::{get_jwt, JWT};
use crate::appf::error::ApiError;
use axum::body::Body;
use axum::http::{header, Request, Response, Uri};
use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;
use tower_http::auth::{AsyncAuthorizeRequest, AsyncRequireAuthorizationLayer};

static AUTH_LAYER: LazyLock<AsyncRequireAuthorizationLayer<JWTAuth>> =
    LazyLock::new(|| AsyncRequireAuthorizationLayer::new(JWTAuth::new(get_jwt())));

#[derive(Clone)]
pub struct JWTAuth {
    jwt: &'static JWT,
}

impl JWTAuth {
    pub fn new(jwt: &'static JWT) -> Self {
        Self { jwt }
    }
}

impl AsyncAuthorizeRequest<Body> for JWTAuth {
    type RequestBody = Body;
    type ResponseBody = Body;
    type Future = Pin<
        Box<
            dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>
                + Send
                + 'static,
        >,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        let jwt = self.jwt;

        // ========== 核心修改：路径白名单判断 ==========
        let uri = request.uri().clone();
        tracing::info!("tracing url {}", uri.path().to_string());
        // println!("tracing url {}", uri.path().to_string());
        // 如果不是 api/ssrp/*** 路径，直接放行
        if !is_path_protected(&uri) {
            return Box::pin(async move { Ok(request) });
        }
        Box::pin(async move {
            let token = request
                .headers()
                .get(header::AUTHORIZATION)
                .map(|value| -> Result<_, ApiError> {
                    let token = value
                        .to_str()
                        .map_err(|_| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization请求头不是一个有效的字符串",
                            ))
                        })?
                        .strip_prefix("Bearer ")
                        .ok_or_else(|| {
                            ApiError::Unauthenticated(String::from(
                                "Authorization请求头必须以 Bearer 开头",
                            ))
                        })?;

                    Ok(token)
                })
                .transpose()?
                .ok_or_else(|| {
                    ApiError::Unauthenticated(String::from("Authorization请求头必须存在"))
                })?;

            let principal = jwt.decode(token).map_err(|err| ApiError::Internal(err))?;
            request.extensions_mut().insert(principal);

            Ok(request)
        })
    }
}

pub fn get_auth_layer() -> &'static AsyncRequireAuthorizationLayer<JWTAuth> {
    &AUTH_LAYER
}

// 支持多个保护前缀
const PROTECTED_PREFIXES: &[&str] = &["/api/ssrp/", "/api/admin/"];

fn is_path_protected(uri: &Uri) -> bool {
    let path = uri.path();
    PROTECTED_PREFIXES
        .iter()
        .any(|prefix| path.starts_with(prefix))
}
