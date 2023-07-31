use once_cell::sync::Lazy;
use std::future::Future;
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

// There must exist a single runtime per virtual machine instance(BEAM node).
// Access to the thread pool is not init'd on startup like in most tokio applications in `main`,
// but rather lazily on the first call to `spawn` whenever the first NIF is called.
static TOKIO: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        // todo: determine empirically how many threads are appropriate
        // must be <= elixir/erlang side - System.schedulers_online()
        // pass in from client as `TOKIO_WORKER_THREADS`
        .worker_threads(4)
        .enable_time()
        .enable_io()
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
