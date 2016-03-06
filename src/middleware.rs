use std::marker::Reflect;

use std::sync::Arc;
use std::error::Error as StdError;

use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_diesel::{ConnectionManager};
use r2d2::{Pool, HandleError, Config, PooledConnection};
use typemap::{Key};
use plugin::{Pluggable, Extensible};
use diesel::{Connection};


pub struct DieselMiddleware<T> where
    T: Connection + Send + Reflect + 'static
{
    pub pool: Arc<Pool<ConnectionManager<T>>>
}

impl<T> DieselMiddleware<T> where
    T: Connection + Send + Reflect + 'static
{
    pub fn new(connect_str: &str,
               num_connections: u32,
               error_handler: Box<HandleError<::r2d2_diesel::Error>>)
                    -> Result<DieselMiddleware<T>, Box<StdError>> {
        let manager = ConnectionManager::<T>::new(connect_str);

        let config = Config::builder()
          .pool_size(num_connections)
          .error_handler(error_handler)
          .build();

        let pool = try!(Pool::new(config, manager));

        Ok(DieselMiddleware { pool: Arc::new(pool) })
    }
}

impl<T> Key for DieselMiddleware<T> where
   T: Connection + Send + Reflect + 'static
{
    type Value = Arc<Pool<ConnectionManager<T>>>;
}

impl<T, D> Middleware<D> for DieselMiddleware<T> where
   T: Connection + Send + Reflect + 'static
{
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<DieselMiddleware<T>>(self.pool.clone());
        Ok(Continue(res))
    }
}

pub trait DieselRequestExtensions<T> where
   T: Connection + Send + Reflect + 'static
{
    fn db_conn(&self) -> PooledConnection<ConnectionManager<T>>;
}

impl<'a, 'b, T, D> DieselRequestExtensions<T> for Request<'a, 'b, D> where
   T: Connection + Send + Reflect + 'static
{
    fn db_conn(&self) -> PooledConnection<ConnectionManager<T>> {
        self.extensions().get::<DieselMiddleware<T>>().unwrap().get().unwrap()
    }
}
