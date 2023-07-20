use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use futures::{
    future::BoxFuture,
    task::{waker_ref, ArcWake},
    Future, FutureExt,
};

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

impl Future for TimerFuture {
    type Output = bool;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("[{:?}] Polling TimerFuture ...", thread::current().id());
        let mut shared_state = self.shared_state.lock().unwrap();

        if shared_state.completed {
            println!("[{:?}] TimerFuture completed ...", thread::current().id());
            Poll::Ready(true)
        } else {
            shared_state.waker = Some(cx.waker().clone());
            println!("[{:?}] TimerFuture pendding ...", thread::current().id());
            Poll::Pending
        }
    }
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        println!(
            "[{:?}] 开始创建新的 TimerFuture completed ...",
            thread::current().id()
        );
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            println!(
                "[{:?}] TimerFuture 生成新线程并开始睡眠 ...",
                thread::current().id()
            );
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();

            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                println!(
                    "[{:?}] TimerFuture 新线程获取 waker，并进行 wake() ...",
                    thread::current().id()
                );
                waker.wake()
            } else {
                println!(
                    "[{:?}] TimerFuture 新线程没获取 waker ...",
                    thread::current().id()
                );
            }
        });

        println!("[{:?}] 返回新的 TimerFuture ...", thread::current().id());
        TimerFuture { shared_state }
    }
}

struct Executor {
    ready_queue: Receiver<Arc<Task>>,
}

impl Executor {
    fn run(&self) {
        println!("[{:?}] Executor running!", thread::current().id());
        while let Ok(task) = self.ready_queue.recv() {
            println!("[{:?}] 接受到任务 ...", thread::current().id());
            let mut future_slot = task.future.lock().unwrap();

            if let Some(mut future) = future_slot.take() {
                println!("[{:?}] 从任务中取得 future ...", thread::current().id());

                // 从任务本身创建一个 'Local Waker'
                let waker = waker_ref(&task);

                let context = &mut Context::from_waker(&*waker);
                println!(
                    "[{:?}] 获得 context, 准备进行 poll() ...",
                    thread::current().id()
                );

                if future.as_mut().poll(context).is_pending() {
                    // 还没有处理完任务，把它再次放回去，以便未来再次执行
                    *future_slot = Some(future);
                    println!("[{:?}] Poll::Pendding ===", thread::current().id());
                } else {
                    println!("[{:?}] Poll::Ready ===", thread::current().id());
                }
            }
        }
        println!("[{:?}] Executor run 结束", thread::current().id());
    }
}

#[derive(Clone)]
struct Spawner {
    task_sender: SyncSender<Arc<Task>>,
}

impl Spawner {
    fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed();
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)),
            task_sender: self.task_sender.clone(),
        });
        println!(
            "[{:?}] 将 Future 组成 Task, 放入 Channel  ...",
            thread::current().id()
        );
        self.task_sender.send(task).expect("too many task queued");
    }
}

struct Task {
    future: Mutex<Option<BoxFuture<'static, ()>>>,
    task_sender: SyncSender<Arc<Task>>,
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        println!("[{:?}] wake_by_ref ...", thread::current().id());
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued!")
    }
}

fn new_executer_and_spawner() -> (Spawner, Executor) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);
    println!(
        "[{:?}] 生成 Executor 和 Spawner (含发送端和接受端) ...",
        thread::current().id()
    );
    (Spawner { task_sender }, Executor { ready_queue })
}

#[test]
fn test() {
    let (spawner, executer) = new_executer_and_spawner();

    spawner.spawn(async {
        println!("[{:?}] howdy!", thread::current().id());
        TimerFuture::new(Duration::new(2, 0)).await;
        println!("[{:?}] done!", thread::current().id());
    });

    // 丢弃生成器，以便我们的执行者知道它已经完成了
    // 并且不回再接受更多的任务来运行
    println!("[{:?}] Drop Spawner!", thread::current().id());
    drop(spawner);

    // 运行执行者直到队列为空为止
    // 这会打印 "howdy", 暂停， 然后打印 "done!"
    executer.run();
}
