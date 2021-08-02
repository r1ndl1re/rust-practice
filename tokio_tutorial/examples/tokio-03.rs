use std::rc::Rc;
use tokio::task::yield_now;

#[tokio::main]
async fn main() {
    tokio::spawn(async {
        let rc = Rc::new("hello");

        // `rc` は `.await` のあとに使用されている
        // つまり、タスクのステートとして保持されなければならない
        yield_now().await;
        println!("{}", rc);
    });
}
