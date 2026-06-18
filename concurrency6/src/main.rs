use std::time::Duration;
use std::thread;

//CPU-bound vs I/O bound work

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms)); //blocking sleep as there is no await
    println!("'{name}' ran for {ms}ms");
}

fn main() {
    // no interleaving output only sequential
        trpl::block_on(async {
            let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.") 
        };

        trpl::select(a, b).await;
    });
}
