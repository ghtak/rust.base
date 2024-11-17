use futures::future::BoxFuture;
use futures::task::{waker_ref, ArcWake};
use futures::{AsyncRead, AsyncReadExt, FutureExt};
use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::time::Duration;
use std::{io, thread};

struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(
            SharedState {
                completed: false,
                waker: None,
            }
        ));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture {
            shared_state
        }
    }
}


struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap();
            if let Some(mut future) = future_slot.take() {
                let wake = waker_ref(&task);
                let context = &mut Context::from_waker(&*wake);
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future);
                }
            }
        }
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output=()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).unwrap();
    }
}
struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self.task_sender.send(cloned).unwrap();
    }
}

fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    (Executor { ready_queue }, Spawner { task_sender })
}

struct MyAsyncHandle {}


impl AsyncRead for MyAsyncHandle {
    fn poll_read(self: Pin<&mut Self>, _cx: &mut Context<'_>, _buf: &mut [u8]) -> Poll<io::Result<usize>> {
        println!("MyAsyncHandle::poll_read");
        Poll::Ready(Ok(0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_runtime() {
        let (executor, spawner) = new_executor_and_spawner();
        spawner.spawn(async {
            TimerFuture::new(Duration::new(2, 0)).await;
            let mut buf: [u8; 10] = [0; 10];
            /*
            pub trait AsyncReadExt: AsyncRead {
                fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> Read<'a, Self> where Self: Unpin
                { assert_future::<Result<usize>, _>(Read::new(self, buf)) }
            }
             */
            let mut async_handle = MyAsyncHandle {};
            async_handle.read(&mut buf).await.unwrap();
        });
        drop(spawner);
        executor.run();
    }
}

