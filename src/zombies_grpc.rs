// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait Zombies {
    fn report_sighting(&self, o: ::grpc::RequestOptions, p: super::zombies::SightingReportRequest) -> ::grpc::SingleResponse<super::zombies::SightingReportResponse>;
}

// client

pub struct ZombiesClient {
    grpc_client: ::grpc::Client,
    method_report_sighting: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::zombies::SightingReportRequest, super::zombies::SightingReportResponse>>,
}

impl ZombiesClient {
    pub fn with_client(grpc_client: ::grpc::Client) -> Self {
        ZombiesClient {
            grpc_client: grpc_client,
            method_report_sighting: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                name: "/zombies.Zombies/report_sighting".to_string(),
                streaming: ::grpc::method::GrpcStreaming::Unary,
                req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
            }),
        }
    }

    pub fn new_plain(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_plain(host, port, conf).map(|c| {
            ZombiesClient::with_client(c)
        })
    }
    pub fn new_tls<C : ::tls_api::TlsConnector>(host: &str, port: u16, conf: ::grpc::ClientConf) -> ::grpc::Result<Self> {
        ::grpc::Client::new_tls::<C>(host, port, conf).map(|c| {
            ZombiesClient::with_client(c)
        })
    }
}

impl Zombies for ZombiesClient {
    fn report_sighting(&self, o: ::grpc::RequestOptions, p: super::zombies::SightingReportRequest) -> ::grpc::SingleResponse<super::zombies::SightingReportResponse> {
        self.grpc_client.call_unary(o, p, self.method_report_sighting.clone())
    }
}

// server

pub struct ZombiesServer {
    pub grpc_server: ::grpc::Server,
}

impl ::std::ops::Deref for ZombiesServer {
    type Target = ::grpc::Server;

    fn deref(&self) -> &Self::Target {
        &self.grpc_server
    }
}

impl ZombiesServer {
    pub fn new<A : ::std::net::ToSocketAddrs, H : Zombies + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H) -> ::grpc::Result<Self> {
        let service_definition = ZombiesServer::new_service_def(h);
        Ok(ZombiesServer {
            grpc_server: ::grpc::Server::new_plain(addr, conf, service_definition)?,
        })
    }

    pub fn new_pool<A : ::std::net::ToSocketAddrs, H : Zombies + 'static + Sync + Send + 'static>(addr: A, conf: ::grpc::ServerConf, h: H, cpu_pool: ::futures_cpupool::CpuPool) -> ::grpc::Result<Self> {
        let service_definition = ZombiesServer::new_service_def(h);
        Ok(ZombiesServer {
            grpc_server: ::grpc::Server::new_plain_pool(addr, conf, service_definition, cpu_pool)?,
        })
    }

    pub fn new_service_def<H : Zombies + 'static + Sync + Send + 'static>(handler: H) -> ::grpc::server::ServerServiceDefinition {
        let handler_arc = ::std::sync::Arc::new(handler);
        ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/zombies.Zombies/report_sighting".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |o, p| handler_copy.report_sighting(o, p))
                    },
                ),
            ],
        )
    }
}
