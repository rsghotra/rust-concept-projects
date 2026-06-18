use std::{future, time::Duration};

use trpl::Either;

fn main() {

    trpl::block_on(async {

        async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
            match trpl::select(future_to_try, trpl::sleep(max_time)).await {
                Either::Left(output) => Ok(output),
                Either::Right(_) => Err(max_time)
            }
        }

        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally Finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with message = {message}"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });

}
