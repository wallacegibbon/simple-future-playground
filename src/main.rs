use simple_future_playground::{new_executor_and_spawner, TimerFuture};
use std::time::Duration;

fn main() {
  let (executor, spawner) = new_executor_and_spawner();

  spawner.spawn(async {
    println!("start!");
    TimerFuture::new(Duration::new(2, 0)).await;
    println!("done.");
  });

  // Drop the spawner (SyncSender) so that executor can exit when the old task finishes.
  drop(spawner);

  executor.run();
}
