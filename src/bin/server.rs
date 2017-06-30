extern crate futures;
extern crate futures_cpupool;

extern crate rs_zombies;
extern crate grpc;

use std::thread;

use futures_cpupool::CpuPool;

use rs_zombies::zombies_grpc::*;
use rs_zombies::zombies::*;

struct ZombiesImpl;

impl Zombies for ZombiesImpl {
    fn report_sighting(&self,
                       _m: grpc::RequestOptions,
                       req: SightingReportRequest)
                       -> grpc::SingleResponse<SightingReportResponse> {
        let mut r = SightingReportResponse::new();

        println!("Received zombie sighting report of {} at {}, {}", req.get_name(), req.get_location().latitude, req.get_location().longitude);

        r.set_accepted(true);
        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let _server = ZombiesServer::new_pool("[::]:50051",
                                          Default::default(),
                                          ZombiesImpl,
                                          CpuPool::new(4))
        .expect("server");

    loop {
        thread::park();
    }
}