// use once_cell::sync::Lazy;
// use std::future::Future;
// use tokio::runtime::{Builder, Runtime};
// use tokio::task::JoinHandle;

// static TOKIO: Lazy<Runtime> = Lazy::new(|| {
//     Builder::new(2)
//         .threaded_scheduler()
//         .build()
//         .expect("Thumbelina.Internal - no runtime!")
// });

// pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
// where
//     T: Future + Send + 'static,
//     T::Output: Send + 'static,
// {
//     TOKIO.spawn(task)
// }
