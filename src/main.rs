use futures::future::{FutureExt, TryFutureExt};

async fn do_work() {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let future03 = async {
        tokio::time::delay_for(std::time::Duration::from_millis(100)).await;
    };
    let res = core.run(future03.unit_error().boxed_local().compat());
    assert!(res.is_ok());
    println!("We good");
}

#[tokio::main]
async fn main() {
    do_work().await;
}

#[tokio::test]
async fn test() {
    do_work().await;
}
