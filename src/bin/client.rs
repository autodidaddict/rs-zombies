extern crate rs_zombies;
extern crate grpc;
extern crate futures;

use rs_zombies::zombies_grpc::*;
use rs_zombies::zombies::*;


fn main() {
    let client = ZombiesClient::new_plain("localhost", 50051, Default::default()).unwrap();

    let mut req = SightingReportRequest::new();
    req.set_name("bob".to_string());

    let mut location = Location::new();
    location.latitude = 40.730610;
    location.longitude = -73.935242;
    req.set_location(location);

    let resp = client.report_sighting(grpc::RequestOptions::new(), req);

    println!("{:?}", resp.wait());
}
