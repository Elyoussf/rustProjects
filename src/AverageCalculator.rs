fn main() {
    println!("Hello to the arithmetic Average calculator!");
    println!("Remark: When you want to stop giving numbers, just type 'e' instead of a number.");

    let mut sum: f64 = 0.0;
    let mut count = 0;

    loop {
        let mut num = String::new(); // declare the buffer
        std::io::stdin().read_line(&mut num).expect("Failed to grasp data!");

        if num.trim() == "e" {
            break;
        }

        count += 1;
        let n: f64 = match num.trim().parse() {
            Ok(value) => value,
            Err(_) => {
                println!("Something went wrong with the specified input.");
                continue; // Skip this iteration and continue with the loop
            }
        };

        sum += n;
    }

    if count == 0 {
        println!("No numbers entered!!");
    } else {
        let res = sum / count as f64;
        println!("The average is: {}", res);
    }
}