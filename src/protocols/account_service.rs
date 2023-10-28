#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoginStatus {
    LoggedIn = 0,
    LoggedOut = 1,
    LogginFailed = 2,
}
impl LoginStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoginStatus::LoggedIn => "LoggedIn",
            LoginStatus::LoggedOut => "LoggedOut",
            LoginStatus::LogginFailed => "LogginFailed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LoggedIn" => Some(Self::LoggedIn),
            "LoggedOut" => Some(Self::LoggedOut),
            "LogginFailed" => Some(Self::LogginFailed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AccountStatus {
    AccountStopped = 0,
    AccountActived = 1,
}
impl AccountStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AccountStatus::AccountStopped => "AccountStopped",
            AccountStatus::AccountActived => "AccountActived",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AccountStopped" => Some(Self::AccountStopped),
            "AccountActived" => Some(Self::AccountActived),
            _ => None,
        }
    }
}
/// 使用手机号码 密码登录
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginRequest {
    #[prost(string, tag = "1")]
    pub area_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub person: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub token: ::prost::alloc::string::String,
}
/// 登出
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutRequest {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogoutResponse {
    #[prost(enumeration = "LoginStatus", tag = "1")]
    pub result: i32,
}
/// 使用校验码登录
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginWithValidCodeRequest {
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub valid_code: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoginWithValidCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 取得校验码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidateCodeRequest {
    #[prost(string, tag = "1")]
    pub phone: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidateCodeResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 添加账号, 需要手工添加账号的情景
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAccountRequest {
    #[prost(string, tag = "1")]
    pub area_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub nick_name: ::core::option::Option<::manage_define::cashmere::Name>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NewAccountResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 注册账号, 用户需要自己注册账号的情景
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterRequest {
    #[prost(string, tag = "1")]
    pub organization_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub department_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub phone: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisterResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 自己修改手机号码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPhoneRequest {
    #[prost(string, tag = "1")]
    pub old_phone: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_phone: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangePhoneOwnResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 加入组
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAccountIntoGroupRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddAccountIntoGroupResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 移出组
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAccountFromGroupRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub group_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveAccountFromGroupResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 改变帐号状态
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeAccountStatusRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(enumeration = "AccountStatus", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeAccountStatusResponse {
    #[prost(enumeration = "AccountStatus", tag = "1")]
    pub result: i32,
}
/// 修改自己的密码
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPasswordRequest {
    #[prost(string, tag = "1")]
    pub old_password: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeOwnPasswordResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// 重置密码, 管理员操作或者后台操作
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeAccountPasswordRequest {
    #[prost(string, tag = "1")]
    pub account_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub new_password: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeAccountPasswordResponse {
    #[prost(string, tag = "1")]
    pub result: ::prost::alloc::string::String,
}
/// Generated server implementations.
pub mod account_grpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with AccountGrpcServer.
    #[async_trait]
    pub trait AccountGrpc: Send + Sync + 'static {
        /// 登录
        async fn login(
            &self,
            request: tonic::Request<super::LoginRequest>,
        ) -> std::result::Result<tonic::Response<super::LoginResponse>, tonic::Status>;
        /// rpc Logout (LogoutRequest) returns (LogoutResponse);
        async fn new_account(
            &self,
            request: tonic::Request<super::NewAccountRequest>,
        ) -> std::result::Result<
            tonic::Response<super::NewAccountResponse>,
            tonic::Status,
        >;
        async fn add_account_into_group(
            &self,
            request: tonic::Request<super::AddAccountIntoGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddAccountIntoGroupResponse>,
            tonic::Status,
        >;
        async fn remove_account_from_group(
            &self,
            request: tonic::Request<super::RemoveAccountFromGroupRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveAccountFromGroupResponse>,
            tonic::Status,
        >;
        /// 修改自身密码
        async fn change_own_password(
            &self,
            request: tonic::Request<super::ChangeOwnPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeOwnPasswordResponse>,
            tonic::Status,
        >;
        /// 修改帐号状态
        async fn change_account_status(
            &self,
            request: tonic::Request<super::ChangeAccountStatusRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeAccountStatusResponse>,
            tonic::Status,
        >;
        async fn change_account_password(
            &self,
            request: tonic::Request<super::ChangeAccountPasswordRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ChangeAccountPasswordResponse>,
            tonic::Status,
        >;
    }
    #[derive(Debug)]
    pub struct AccountGrpcServer<T: AccountGrpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: AccountGrpc> AccountGrpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for AccountGrpcServer<T>
    where
        T: AccountGrpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/account_service.AccountGrpc/Login" => {
                    #[allow(non_camel_case_types)]
                    struct LoginSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<T: AccountGrpc> tonic::server::UnaryService<super::LoginRequest>
                    for LoginSvc<T> {
                        type Response = super::LoginResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::LoginRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::login(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = LoginSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/account_service.AccountGrpc/NewAccount" => {
                    #[allow(non_camel_case_types)]
                    struct NewAccountSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<
                        T: AccountGrpc,
                    > tonic::server::UnaryService<super::NewAccountRequest>
                    for NewAccountSvc<T> {
                        type Response = super::NewAccountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::NewAccountRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::new_account(&inner, request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = NewAccountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/account_service.AccountGrpc/AddAccountIntoGroup" => {
                    #[allow(non_camel_case_types)]
                    struct AddAccountIntoGroupSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<
                        T: AccountGrpc,
                    > tonic::server::UnaryService<super::AddAccountIntoGroupRequest>
                    for AddAccountIntoGroupSvc<T> {
                        type Response = super::AddAccountIntoGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddAccountIntoGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::add_account_into_group(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddAccountIntoGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/account_service.AccountGrpc/RemoveAccountFromGroup" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveAccountFromGroupSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<
                        T: AccountGrpc,
                    > tonic::server::UnaryService<super::RemoveAccountFromGroupRequest>
                    for RemoveAccountFromGroupSvc<T> {
                        type Response = super::RemoveAccountFromGroupResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveAccountFromGroupRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::remove_account_from_group(
                                        &inner,
                                        request,
                                    )
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveAccountFromGroupSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/account_service.AccountGrpc/ChangeOwnPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeOwnPasswordSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<
                        T: AccountGrpc,
                    > tonic::server::UnaryService<super::ChangeOwnPasswordRequest>
                    for ChangeOwnPasswordSvc<T> {
                        type Response = super::ChangeOwnPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeOwnPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::change_own_password(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeOwnPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/account_service.AccountGrpc/ChangeAccountStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeAccountStatusSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<
                        T: AccountGrpc,
                    > tonic::server::UnaryService<super::ChangeAccountStatusRequest>
                    for ChangeAccountStatusSvc<T> {
                        type Response = super::ChangeAccountStatusResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeAccountStatusRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::change_account_status(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeAccountStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/account_service.AccountGrpc/ChangeAccountPassword" => {
                    #[allow(non_camel_case_types)]
                    struct ChangeAccountPasswordSvc<T: AccountGrpc>(pub Arc<T>);
                    impl<
                        T: AccountGrpc,
                    > tonic::server::UnaryService<super::ChangeAccountPasswordRequest>
                    for ChangeAccountPasswordSvc<T> {
                        type Response = super::ChangeAccountPasswordResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeAccountPasswordRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                <T as AccountGrpc>::change_account_password(&inner, request)
                                    .await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ChangeAccountPasswordSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: AccountGrpc> Clone for AccountGrpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: AccountGrpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: AccountGrpc> tonic::server::NamedService for AccountGrpcServer<T> {
        const NAME: &'static str = "account_service.AccountGrpc";
    }
}
