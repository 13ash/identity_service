#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserIdentifier {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusModel {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "2")]
    pub status: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusResponse {
    #[prost(string, tag = "1")]
    pub uuid: ::prost::alloc::string::String,
    #[prost(enumeration = "Status", tag = "2")]
    pub status: i32,
    #[prost(enumeration = "Action", tag = "3")]
    pub action: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    Add = 0,
    Remove = 1,
    Update = 2,
    Login = 3,
    Logout = 4,
    Get = 5,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::Add => "ADD",
            Action::Remove => "REMOVE",
            Action::Update => "UPDATE",
            Action::Login => "LOGIN",
            Action::Logout => "LOGOUT",
            Action::Get => "GET",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ADD" => Some(Self::Add),
            "REMOVE" => Some(Self::Remove),
            "UPDATE" => Some(Self::Update),
            "LOGIN" => Some(Self::Login),
            "LOGOUT" => Some(Self::Logout),
            "GET" => Some(Self::Get),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Offline = 0,
    Online = 1,
    Busy = 2,
    Away = 3,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Offline => "OFFLINE",
            Status::Online => "ONLINE",
            Status::Busy => "BUSY",
            Status::Away => "AWAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "OFFLINE" => Some(Self::Offline),
            "ONLINE" => Some(Self::Online),
            "BUSY" => Some(Self::Busy),
            "AWAY" => Some(Self::Away),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod presence_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct PresenceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PresenceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PresenceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PresenceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PresenceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn add(
            &mut self,
            request: impl tonic::IntoRequest<super::UserIdentifier>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/presence.Presence/Add");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("presence.Presence", "Add"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn remove(
            &mut self,
            request: impl tonic::IntoRequest<super::UserIdentifier>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/presence.Presence/Remove");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("presence.Presence", "Remove"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::UserIdentifier>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/presence.Presence/Get");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("presence.Presence", "Get"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn logout(
            &mut self,
            request: impl tonic::IntoRequest<super::UserIdentifier>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/presence.Presence/Logout");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("presence.Presence", "Logout"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn login(
            &mut self,
            request: impl tonic::IntoRequest<super::UserIdentifier>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/presence.Presence/Login");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("presence.Presence", "Login"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::StatusModel>,
        ) -> std::result::Result<tonic::Response<super::StatusResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/presence.Presence/Update");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new("presence.Presence", "Update"));
            self.inner.unary(req, path, codec).await
        }
    }
}
