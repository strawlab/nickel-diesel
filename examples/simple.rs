extern crate nickel;
extern crate nickel_diesel;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;

use r2d2::{NopErrorHandler, PooledConnection};
use r2d2_diesel::{ConnectionManager};
use nickel_diesel::{DieselMiddleware, DieselRequestExtensions};

use diesel::pg::PgConnection;

use std::env;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use diesel::connection::Connection;

fn one<'a> (request: &mut Request, response: Response<'a>) -> MiddlewareResult<'a> {
    let connection: PooledConnection<ConnectionManager<PgConnection>> = request.db_conn();

    // Presumably you would use your auto-generated schema code to perform a database query. For
    // this example, we don't want to require codegen and thus we execute an SQL statement without
    // much benefit from the ORM.
    let result = connection.execute("SELECT 1").unwrap();
    response.send(format!("{}",result))
}

fn main() {
    dotenv::dotenv().ok();

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/one", one );

    let postgres_url = env::var("DATABASE_URL")
        .expect("No DATABASE_URL given.");
    let num_connections_str = env::var("DATABASE_CONNECTIONS").unwrap_or("5".to_owned());
    let num_connections = num_connections_str.parse::<u32>().unwrap();
    let dbpool: DieselMiddleware<PgConnection> = DieselMiddleware::new(&*postgres_url,
                                                                       num_connections,
                                                                       Box::new(NopErrorHandler)).unwrap();
    server.utilize(dbpool);
    server.utilize(router);

    let serve_host = env::var("SERVE_HOST").unwrap_or("127.0.0.1".to_owned());
    let serve_port_str = env::var("SERVE_PORT").unwrap_or("9000".to_owned());
    let serve_port = serve_port_str.parse::<u16>().unwrap();

    server.listen((&*serve_host,serve_port)).unwrap();
}
