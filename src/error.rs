use ffi::memcached_return_t;

pub struct MemcacheError {
    pub kind: memcached_return_t,
}

pub type MemcacheResult<T> = Result<T, MemcacheError>;
