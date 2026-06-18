use std::time::Duration;


fn main() {
    //note the tx wont drop automatically unless we move tx to the async block itself.
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future")
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1000)).await;
        }

        //on receiver side, we want to wait until channel is closed 
        //unbounded loop. Recv returns a Future which will resolve to either Some or None

        while let Some(value) = rx.recv().await {
            println!("Receiveing value = {}", value);
        }
        // it does not end as tx was never dropped and rx was never closed
        
    });

}
