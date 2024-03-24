

#[derive(PartialEq, Debug)]
enum TextNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine
}


pub fn process_file(content: &str) {

    println!("Processing day 1");
    let mut sum: i32 = 0;

    // Split the content into lines and print them
    for line in content.lines() {
        let mut first_dig: i32 = -1;
        let mut second_dig = -1;
        let mut found_first = false;

        let mut line_idx = 0;
        while line_idx < line.len() {
            let curr_char = line.chars().nth(line_idx).expect("Expected to find a char from iter");

            let curr_dig;
            if curr_char.is_numeric() {
                curr_dig = curr_char as i32 - '0' as i32;
                line_idx += 1;
            } else {
                let substr = &line[line_idx..];
                let maybe_match: Option<TextNumber> = match_number(substr);
                if maybe_match == None {
                    line_idx += 1;
                    continue;
                }

                let number_match = maybe_match.expect("Should've checked option already");
                let idx_progress = progress_index(&number_match);
                let parsed_num = get_numberic(&number_match);
                curr_dig = parsed_num;
                line_idx += idx_progress;
            }

            if found_first {
                second_dig = curr_dig;
            } else {
                first_dig = curr_dig;
                second_dig = curr_dig;
                found_first = true;   
            }
        }
        // Take first and second parsed digits, move 1st to tens and add to sum
        let to_add = (10 * first_dig) + second_dig;
        sum += to_add;
        if first_dig == -1 || second_dig == -1 || !found_first {
            println!("unexpected state first_dig: {}, second_dig: {}, found_first: {}, line: {}", first_dig, second_dig, found_first, line);
        }
    }
    println!("Finished processing, sum: {}", sum);
}

fn match_number(substring: &str) -> Option<TextNumber> {
    if substring.len() < 3 {
        return None
    }

    for i in 3..substring.len() + 1 {
        let test_str = &substring[0..i];
        let maybe_number = try_match_number(test_str);
        match maybe_number {
            Some(num) => return Some(num),
            None => continue
        }
    }

    None
}

fn try_match_number(substring: &str) -> Option<TextNumber>{
    match substring.to_lowercase().as_str() {
        "one" => Some(TextNumber::One),
        "two" => Some(TextNumber::Two),
        "three" => Some(TextNumber::Three),
        "four" => Some(TextNumber::Four),
        "five" => Some(TextNumber::Five),
        "six" => Some(TextNumber::Six),
        "seven" => Some(TextNumber::Seven),
        "eight" => Some(TextNumber::Eight),
        "nine" => Some(TextNumber::Nine),
        _ => None,
    }
}

fn progress_index(num: &TextNumber) -> usize {
    match num {
        TextNumber::One | TextNumber::Two | TextNumber::Six => 3,
        TextNumber::Four | TextNumber::Five | TextNumber::Nine => 4,
        TextNumber::Three | TextNumber::Seven | TextNumber::Eight => 5
    }
}

fn get_numberic(text: &TextNumber) -> i32 {
    match text {
        TextNumber::One => 1,
        TextNumber::Two => 2,
        TextNumber::Three => 3,
        TextNumber::Four => 4,
        TextNumber::Five => 5,
        TextNumber::Six => 6,
        TextNumber::Seven => 7,
        TextNumber::Eight => 8,
        TextNumber::Nine => 9
    }
}
