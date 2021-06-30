use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32, u8);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready(format!("cd {}: Zero!!!", self.1))
        } else {
            println!("cd {}: {}", self.1, self.0);
            self.0 -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

fn main() {
    let count_down_future1 = CountDown(10, 1);
    let count_down_future2 = CountDown(20, 2);
    let cd_set = join_all(vec![count_down_future1, count_down_future2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s)
    }
}