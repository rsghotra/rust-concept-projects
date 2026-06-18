use std::time::Duration;

fn main() {
    
    //top level async functiom for the run-time
    //await returns a Future

    trpl::block_on(async {

        let handle = trpl::spawn_task(async { //spawn_task
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await; 
            }
        });

        for i in 1..5 {
            println!("hi numer {i} from the second task");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}
