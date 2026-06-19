use core::num;
use std::sync::{Arc, Mutex};
use std::thread;



fn parallel_map(input: Vec<i32>, num_threads: usize) -> Vec<i32> {

    let input = Arc::new(Mutex::new(input));

    //shared output vector
    let output = Arc::new(Mutex::new(Vec::new()));

    let mut handles = Vec::new();


    //create worker threads

    for _ in 0..num_threads {

        //give each worker access to the shared input

        let input_clone = Arc::clone(&input);

        //give each worker access to the shared output

        let output_clone = Arc::clone(&output);


        let handle = thread::spawn(move || {
            //keep working until no more item exists

            loop {
                //take one item from input
                let item = {
                    let mut input_lock = input_clone.lock().unwrap();

                    input_lock.pop()
                };

                match item  {
                    //got work to do
                    Some(x) => {
                        let result = x * x;

                        //lock output vector
                        let mut output_lock = output_clone.lock().unwrap();

                        output_lock.push(result);
                    }
                    None => break
                }
            }
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    //lock outoput vector
    let result = output.lock().unwrap().clone();

    result

}


fn main() {
        let input =
        vec![1, 2, 3, 4, 5, 6];

    // Use 3 worker threads
    let result =
        parallel_map(input, 3);

    println!("{:?}", result);
}
