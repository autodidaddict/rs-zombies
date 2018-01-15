extern crate futures;
extern crate futures_cpupool;

extern crate grpc;
extern crate protobuf;
extern crate rs_zombies;

use std::thread;

use rs_zombies::zombies_grpc::*;
use rs_zombies::zombies::*;

struct ZombiesServiceImpl;

impl Zombies for ZombiesServiceImpl {
    fn report_sighting(
        &self,
        _m: grpc::RequestOptions,
        req: SightingReportRequest,
    ) -> grpc::SingleResponse<SightingReportResponse> {
        let mut r = SightingReportResponse::new();

        println!(
            "Received zombie sighting report of {} at {}, {}",
            req.get_name(),
            req.get_location().latitude,
            req.get_location().longitude
        );

        r.set_accepted(true);
        grpc::SingleResponse::completed(r)
    }

    fn zombies_nearby(
        &self,
        _m: grpc::RequestOptions,
        _req: ProximityRequest,
    ) -> grpc::SingleResponse<ProximityResponse> {
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
    let mut server = grpc::ServerBuilder::new_plain();
    server.http.set_port(8080);
    server.add_service(ZombiesServer::new_service_def(ZombiesServiceImpl));
    server.http.set_cpu_pool_threads(4);
    let _server = server.build().expect("server");

    loop {
        thread::park();
    }
}
