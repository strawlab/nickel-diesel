#[macro_use]
extern crate nickel;
extern crate nickel_diesel;
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;

use r2d2_diesel::ConnectionManager;
use r2d2::{Pool, NopErrorHandler};

use nickel::Nickel;
use nickel_diesel::DieselMiddleware;
use diesel::sqlite::SqliteConnection;

#[test]
fn test_sqlite_middleware_new() {
    let mut server = Nickel::new();
    let database_url = ":memory:";
    let dbpool: DieselMiddleware<SqliteConnection> =
        DieselMiddleware::new(&*database_url, 5, Box::new(NopErrorHandler)).unwrap();
    server.utilize(dbpool);
}

#[test]
fn test_sqlite_middleware_from_pool() {
    let mut server = Nickel::new();
    let database_url = ":memory:";

    let manager = ConnectionManager::<SqliteConnection>::new(database_url);

    let pool = Pool::builder()
        .max_size(5)
        .error_handler(Box::new(NopErrorHandler))
        .build(manager).unwrap();
    let diesel_mw = DieselMiddleware::from_pool(pool);
    server.utilize(diesel_mw);
}
