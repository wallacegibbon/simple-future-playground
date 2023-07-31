use std::time::Duration;

use simple_future_playground::{TimerFuture, Spawner, Executor, new_executor_and_spawner};

fn main() {
	let (executor, spawner) = new_executor_and_spawner();

	spawner.spawn(async {
		println!("start!");
		TimerFuture::new(Duration::new(2, 0)).await;
		println!("done.");
	});

	drop(spawner);
	executor.run();
}

