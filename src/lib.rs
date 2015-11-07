extern crate iron;
extern crate rustc_serialize;
#[macro_use]
extern crate router;
extern crate bodyparser;
extern crate racer;
extern crate crypto;

use iron::{Iron, Chain};

mod engine;
mod types;

#[derive(Debug)]
pub struct Config {
    pub port: usize,
    pub secret_file: String
}

/// Start the http server using the given configuration
///
/// `serve` is a blocking call which runs for the duration of the HTTP server.
///
/// # Example
///
/// ```no_run
/// # use libracerd::{Config, serve};
/// let cfg = Config {
///     port: 3000,
///     secret_file: "/tmp/secret".to_string()
/// };
///
/// serve(cfg);
/// ```
///
pub fn serve(config: Config) {
    // For debugging. Torn about `log` crate since there's potentially a lot of noise from our
    // dependencies.
    println!("{:?}", config);

    let chain = Chain::new(router!(
        post "/parse_file"  => handle::parse_file,
        post "/completions" => handle::completions,
        post "/definition"  => handle::definition,
        post "/declaration" => handle::declaration,
        get  "/ping"        => handle::ping));

    let host = format!("localhost:{}", config.port);

    // TODO return this result instead of unwrapping. Need to add error type, first.
    Iron::new(chain).http(&host[..]).unwrap();
}

/// Handlers for Iron
#[allow(unused_variables)]
mod handle {
    use iron::prelude::*;
    use iron::status;

    /// Parse a file and return a list of issues (warnings, errors) encountered
    pub fn parse_file(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }

    /// Given a location, return a list of possible completions
    pub fn completions(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }

    /// Given a location, return where the identifier is defined
    pub fn definition(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }

    /// Given a location, return a location where the identifier is declared
    pub fn declaration(req: &mut Request) -> IronResult<Response> {
        unimplemented!();
    }

    /// Check if the server is accepting requests
    pub fn ping(req: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "{\"pong\": true}")))
    }
}
