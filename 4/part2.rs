fn main() {
    const START: &str = "272091";
    //7777, 7778, 7779, 7788, 7789, 7799, 7888, 7889, 7899, 7999
    const END: &str = "815432";
    let mut current = String::from(START);
    let mut matches = 0;
    // while current is less than end
    while current < END.to_string() {
        // from the left iterate over each digit
        let mut previous_number = '0';
        let mut next_str = String::with_capacity(current.len());
        let mut replaced = false;
        for number in current.chars() {
            // if a digit is less than the previous, make it equal
            // and if we've ever replaced a digit, replace the rest of the string
            if replaced {
                next_str.push(previous_number);
            } else if (number as i32) < (previous_number as i32) {
                next_str.push(previous_number);
                replaced = true;
            } else if number == previous_number {
                next_str.push(number);
            } else {
                next_str.push(number);
                previous_number = number;
            }
        }

        let mut has_double = false;
        let mut previous_number = '0';
        let mut matched_numbers = 0;
        for number in next_str.chars() {
            if number == previous_number {
                matched_numbers = matched_numbers + 1;
            } else {
                if matched_numbers == 1 {
                    has_double = true;
                }
                matched_numbers = 0;
            }
            previous_number = number;
        }
        if matched_numbers == 1 {
            has_double = true;
        }

        // check if number has a double
        if has_double {
            // if so add one to possibilities
            matches += 1;
            println!("{} Has Isolated Double - {}", next_str, matches);
        }

        // increase the number by one
        let next_number: i32  = next_str.parse().unwrap();
        let incremented: i32 = next_number + 1;
        let incremented_str = incremented.to_string();
        current.clear();
        current.push_str(&incremented_str);
    }
    println!("Total matches: {}", matches);
}