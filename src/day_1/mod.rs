

pub fn process_file(content: &str) {
    // Example processing logic for module 1
    println!("Processing file content using Module 1:");

    let mut sum: i32 = 0;

    // Split the content into lines and print them
    for line in content.lines() {
        let mut first_dig: i32 = -1;
        let mut second_dig: i32 = -1;
        let mut found_first = false;
        for curr_char in line.chars() {
            if curr_char.is_numeric() {
                if found_first {
                    second_dig = curr_char as i32 - '0' as i32;
                } else {
                    found_first = true;
                    first_dig = curr_char as i32 - '0' as i32;
                    second_dig = curr_char as i32 - '0' as i32;
                }
            }
        }
        let to_add = (10 * first_dig) + second_dig;
        sum += to_add;
        println!("Got value: {}, first: {}, second: {} for line: {}", to_add, first_dig, second_dig, line);
        if first_dig == -1 || second_dig == -1 || !found_first {
            println!("unexpected state first_dig: {}, second_dig: {}, found_first: {}, line: {}", first_dig, second_dig, found_first, line);
        }
    }
    println!("Finished processing, sum: {}", sum);
}