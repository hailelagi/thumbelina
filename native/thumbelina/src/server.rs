// use tokio::runtime::{Builder, Runtime, Lazy};
// use tokio::task::JoinHandle;

// // SPAWN a tokio managed thread to send BEAM messages when ready
// static TOKIO: Lazy<Runtime> = Lazy::new(|| {
//     Builder::new_multi_thread()
//         .threaded_scheduler(2)
//         .build()
//         .expect("Must schedule a thread to send messages to the BEAM")
// });

// // uhm whut?
// pub fn spawn<T>(task: T) -> JoinHandle<T::Output>
// where
//     T: Future + Send + 'static,
//     T::Output: Send + 'static,
// {
//     TOKIO.spawn(task)
// }
