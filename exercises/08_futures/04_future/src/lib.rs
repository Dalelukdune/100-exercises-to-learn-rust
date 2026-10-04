use std::rc::Rc;
use tokio::task::yield_now;

fn spawner() {
    tokio::spawn(example());
}

async fn example() {
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }
    yield_now().await;
}
