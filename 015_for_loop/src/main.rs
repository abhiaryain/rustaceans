fn main() {
    let statuses = ["pending", "accepted", "rejected", "completed", "cancelled"];

    // `for` loop on Range
    for idx in 0..statuses.len() {
        if idx % 2 == 0 {
            continue;
        }
        println!("status: {}", statuses[idx]);
    }

    println!();

    // `for` loop on array.
    for status in statuses {
        println!("status: {status}")
    }
}
