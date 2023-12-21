use std::fs;

fn main() {
    println!("Hello, world!");

    let binding = fs::read_to_string("./src/1/input.txt").unwrap();
    let contents = binding.lines();

    let mut total = 0;

    for line in contents {

        let first_num = line.chars().find(|c| c.is_numeric());
        let last_num = line.chars().rfind(|c| c.is_numeric());

        let mut complete = String::new();

        if(first_num.is_some()){
            complete.push(first_num.unwrap());
        }

        if(last_num.is_some()){
            complete.push(last_num.unwrap());
        }

        let parsed_number = complete.parse::<i32>();

        if(parsed_number.is_ok()){
            total = total + parsed_number.unwrap();
        }


        println!("Got {:?}", complete);
    }

    println!("total {:?}", total);


}
