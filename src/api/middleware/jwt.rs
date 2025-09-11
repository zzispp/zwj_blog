use std::future::{ready, Ready};

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, Result,
};
use futures_util::future::LocalBoxFuture;
use jwt_simple::prelude::*;

pub struct JwtMiddleware;

impl<S, B> Transform<S, ServiceRequest> for JwtMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(JwtMiddlewareService { service }))
    }
}

pub struct JwtMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for JwtMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().as_str();
        let path = req.path();

        // 检查是否需要JWT验证的路径和方法
        let needs_auth = match method {
            "POST" if path.contains("/create") => true,
            "PUT" => true,
            "DELETE" => true,
            "PATCH" => true,
            _ => false,
        };

        if !needs_auth {
            // 不需要验证，直接调用下一个服务
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            // 需要JWT验证
            let auth_header = req.headers().get("Authorization");

            match auth_header {
                Some(header_value) => {
                    match header_value.to_str() {
                        Ok(auth_str) => {
                            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                                match verify_jwt_token(token) {
                                    Ok(_) => {
                                        // Token有效，继续处理请求
                                        let fut = self.service.call(req);
                                        Box::pin(async move {
                                            let res = fut.await?;
                                            Ok(res)
                                        })
                                    }
                                    Err(_) => {
                                        // Token无效
                                        Box::pin(async move {
                                            Err(actix_web::error::ErrorUnauthorized(
                                                "Invalid or expired token",
                                            ))
                                        })
                                    }
                                }
                            } else {
                                // Authorization header 格式错误
                                Box::pin(async move {
                                    Err(actix_web::error::ErrorUnauthorized(
                                        "Authorization header must start with 'Bearer '",
                                    ))
                                })
                            }
                        }
                        Err(_) => {
                            // Authorization header 不是有效的UTF-8字符串
                            Box::pin(async move {
                                Err(actix_web::error::ErrorUnauthorized(
                                    "Invalid authorization header format",
                                ))
                            })
                        }
                    }
                }
                None => {
                    // 缺少Authorization header
                    Box::pin(async move {
                        Err(actix_web::error::ErrorUnauthorized(
                            "Missing authorization header",
                        ))
                    })
                }
            }
        }
    }
}

// JWT令牌验证函数
fn verify_jwt_token(token: &str) -> Result<JWTClaims<NoCustomClaims>, jwt_simple::Error> {
    // 从文件读取JWT密钥（与用户服务使用相同的密钥）
    let key = get_jwt_key_from_file()?;

    // 验证并解码token
    key.verify_token::<NoCustomClaims>(token, None)
}

// 从文件读取JWT密钥（与用户服务保持一致）
fn get_jwt_key_from_file() -> Result<HS256Key, jwt_simple::Error> {
    let key_file_path = "./jwt_key.bin";

    // 读取密钥文件
    let key_bytes = std::fs::read(key_file_path)
        .map_err(|e| jwt_simple::Error::msg(format!("Failed to read JWT key file: {}", e)))?;

    Ok(HS256Key::from_bytes(&key_bytes))
}

// JWT令牌生成函数（用于测试或其他地方）
pub fn generate_jwt_token() -> Result<String, jwt_simple::Error> {
    let key = get_jwt_key_from_file()?;

    let claims = Claims::create(Duration::from_hours(24)); // 24小时有效期
    key.authenticate(claims)
}
