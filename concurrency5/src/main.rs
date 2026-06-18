use std::time::Duration;



//resolving issue from Concurrency2 program
//now messages will come in desired gaps & program will return as tx get dropped
//by putting the tx and rx in separate asyncs
fn main() {
    trpl::block_on(async {

        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();

        let tx1_fut = async move {
             let vals = vec![
                String::from("1 more"),
                String::from("1 messages"),
                String::from("1 from"),
                String::from("1 her")
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received {value}");
            }
        };

        let tx_fut = async  move{
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future")
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut); //this decides in which order the futurew will be awaited
    });
}