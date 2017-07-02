extern crate futures;
extern crate futures_cpupool;

extern crate rs_zombies;
extern crate grpc;
extern crate protobuf;

use std::thread;

use futures_cpupool::CpuPool;

use rs_zombies::zombies_grpc::*;
use rs_zombies::zombies::*;

struct ZombiesService;

impl Zombies for ZombiesService {
    fn report_sighting(&self,
                       _m: grpc::RequestOptions,
                       req: SightingReportRequest)
                       -> grpc::SingleResponse<SightingReportResponse> {
        let mut r = SightingReportResponse::new();

        println!("Received zombie sighting report of {} at {}, {}", req.get_name(), req.get_location().latitude, req.get_location().longitude);

        r.set_accepted(true);
        grpc::SingleResponse::completed(r)
    }

    fn zombies_nearby(&self,
                      _m: grpc::RequestOptions,
                      req: ProximityRequest)
                      -> grpc::SingleResponse<ProximityResponse> {
        let mut r = ProximityResponse::new();

        let mut location = Location::new();
        location.latitude = 40.730610;
        location.longitude = -73.935242;

        let mut al = Zombie::new();
        al.set_name("Alfred".to_owned());
        al.set_location(location.clone());

        let mut bob = Zombie::new();
        bob.set_name("Bob".to_owned());
        bob.set_location(location.clone());

        let vec = vec![al, bob];
        let zombie_list = ::protobuf::RepeatedField::from_vec(vec);

        r.set_zombies(zombie_list);

        grpc::SingleResponse::completed(r)
    }
}

fn main() {
    let _server = ZombiesServer::new_pool("[::]:50051",
                                          Default::default(),
                                          ZombiesService,
                                          CpuPool::new(4))
        .expect("server");

    loop {
        thread::park();
    }
}