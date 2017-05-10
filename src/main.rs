extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world, "index");
    router.post("/route", getFriend, "query");

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "hello world!")))
    }

    fn getRoute(_: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "Make dial happen here!")))
    }

    Iron::new(router).http("localhost:3000").unwrap();
    println!("on 3000!");
}
