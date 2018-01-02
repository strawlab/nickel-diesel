extern crate nickel;
extern crate nickel_diesel;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use r2d2::{NopErrorHandler, PooledConnection};
use r2d2_diesel::ConnectionManager;
use nickel_diesel::{DieselMiddleware, DieselRequestExtensions};

use diesel::sqlite::SqliteConnection;

use nickel::{Nickel, HttpRouter, Request, Response, MiddlewareResult};
use diesel::connection::Connection;

fn one<'a>(request: &mut Request, response: Response<'a>) -> MiddlewareResult<'a> {
    let connection: PooledConnection<ConnectionManager<SqliteConnection>> = request.db_conn();

    // Presumably you would use your auto-generated schema code to perform a database query. For
    // this example, we don't want to require codegen and thus we execute an SQL statement without
    // much benefit from the ORM.
    let result = connection.execute("SELECT 1").unwrap(); // returns 0 because no rows affected
    response.send(format!("{}", result))
}

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/one", one);

    let database_url = ":memory:";
    let dbpool: DieselMiddleware<SqliteConnection> =
        DieselMiddleware::new(&*database_url, 5, Box::new(NopErrorHandler)).unwrap();
    server.utilize(dbpool);
    server.utilize(router);

    let serve_host = "127.0.0.1";
    let serve_port = 9001;

    server.listen((&*serve_host, serve_port)).unwrap();
}
