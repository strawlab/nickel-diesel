use std::sync::Arc;
use std::error::Error as StdError;

use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_diesel::{ConnectionManager};
use r2d2::{Pool, HandleError, Config, PooledConnection};
use typemap::Key;
use plugin::{Pluggable, Extensible};

pub struct DieselMiddleware {
    pub pool: Arc<Pool<ConnectionManager>>
}

impl DieselMiddleware {
    pub fn new(connect_str: &str,
               num_connections: u32,
               error_handler: Box<HandleError<::r2d2_diesel::Error>>)
                    -> Result<DieselMiddleware, Box<StdError>> {
        let manager = ConnectionManager::new(connect_str);

        let config = Config::builder()
          .pool_size(num_connections)
          .error_handler(error_handler)
          .build();

        let pool = try!(Pool::new(config, manager));

        Ok(DieselMiddleware { pool: Arc::new(pool) })
    }
}

impl Key for DieselMiddleware { type Value = Arc<Pool<ConnectionManager>>; }

impl<D> Middleware<D> for DieselMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<DieselMiddleware>(self.pool.clone());
        Ok(Continue(res))
    }
}

pub trait DieselRequestExtensions {
    fn db_conn(&self) -> PooledConnection<ConnectionManager>;
}

impl<'a, 'b, D> DieselRequestExtensions for Request<'a, 'b, D> {
    fn db_conn(&self) -> PooledConnection<ConnectionManager> {
        self.extensions().get::<DieselMiddleware>().unwrap().get().unwrap()
    }
}
