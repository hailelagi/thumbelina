use once_cell::sync::Lazy;
use std::env;
use std::future::Future;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

// There must exist a single runtime per virtual machine instance(BEAM node).
// Access to the thread pool is not init'd on startup like in most tokio applications in `main`,
// but rather lazily on the first call to `spawn` whenever the first NIF is called.
static TOKIO: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(set_workers())
        .build()
        .expect("Thumbelina.Internal - no runtime!")
});

// Asynchronously spawn a green thread that's to be managed on the tokio runtime.
pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    TOKIO.spawn(task)
}

fn set_workers() -> usize {
    match env::var("TOKIO_WORKER_THREADS") {
        Ok(val) => val.parse().unwrap(),
        Err(_e) => 1,
    }
}
