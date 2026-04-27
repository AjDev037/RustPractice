use std::io;

fn main() {
    
    //let x: u8 = 12;
    //let y:i8 = 10;

    //We cannot do the following as they are different type of data.
    //let z = x + y;
    //It will throw an error.
    //println!("{}",z);

    //This works similar to any other languages. Only matching types can do operations.
    //And bits limitation apply. u8 cannot have for example value of 256 as 255 is the max value.

    //Typical operators works (*, /, %, +, -) for operations.

    //We can cast values after we declare them like
    //let example = 255.0f32; //This is a valid f32
    //let example2 = 123_i64; //Underscores also works and can be used for readability.
    //let example3 = 123_000i64; //THis is the same as 123,000

    //We can also do explicit casts
    //let example4 = 123_000 as i64;

    //This can be done in operations.
    //let example_x = 123_000 as i64;
    //let example_y = 10_i32;

    //Usually this wont be possible to do operations with them.
    //But if we cast them it works


    //let example_z = example_x /(example_y as i64);
    //println!("{}",example_z);

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Expected read line");

    //trim() removes invisble spaces from the console when we type
    //parse() I thought this converted the value into it's type.
    //unwrap() Need to check this out. This is like it converts the text into the actual type we want
    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}",int_input);

}
