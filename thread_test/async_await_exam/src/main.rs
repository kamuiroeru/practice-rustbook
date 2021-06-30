use futures::executor;

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    async_add(2, 3).await + async_add(3, 4).await + async_add(4, 5).await
}

fn main() {
    println!("{}", executor::block_on(something_great_async_function()));
}
