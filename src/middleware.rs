use std::any::Any;

use std::sync::Arc;
use std::error::Error as StdError;

use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_diesel::{ConnectionManager};
use r2d2::{Pool, HandleError, PooledConnection};
use typemap::{Key};
use plugin::Extensible;
use diesel::{Connection};


pub struct DieselMiddleware<T> where
    T: Connection + Send + Any
{
    pub pool: Arc<Pool<ConnectionManager<T>>>
}

impl<T> DieselMiddleware<T> where
    T: Connection + Send + Any
{
    pub fn new(connect_str: &str,
               num_connections: u32,
               error_handler: Box<HandleError<::r2d2_diesel::Error>>)
                    -> Result<DieselMiddleware<T>, Box<StdError>> {
        let manager = ConnectionManager::<T>::new(connect_str);

        let pool = Pool::builder()
            .max_size(num_connections)
            .error_handler(error_handler)
            .build(manager)?;

        Ok(DieselMiddleware { pool: Arc::new(pool) })
    }

    pub fn from_pool(pool: Pool<ConnectionManager<T>>) -> DieselMiddleware<T> {
        DieselMiddleware { pool: Arc::new(pool) }
    }
}

impl<T> Key for DieselMiddleware<T> where
   T: Connection + Send + Any
{
    type Value = Arc<Pool<ConnectionManager<T>>>;
}

impl<T, D> Middleware<D> for DieselMiddleware<T> where
   T: Connection + Send + Any
{
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<DieselMiddleware<T>>(self.pool.clone());
        Ok(Continue(res))
    }
}

pub trait DieselRequestExtensions<T> where
   T: Connection + Send + Any
{
    fn db_conn(&self) -> PooledConnection<ConnectionManager<T>>;
}

impl<'a, 'b, T, D> DieselRequestExtensions<T> for Request<'a, 'b, D> where
   T: Connection + Send + Any
{
    fn db_conn(&self) -> PooledConnection<ConnectionManager<T>> {
        self.extensions().get::<DieselMiddleware<T>>().unwrap().get().unwrap()
    }
}
