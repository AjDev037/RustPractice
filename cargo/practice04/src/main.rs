use std::io;
//We are using std create
//Importing io module.

fn main() {
    println!("--This is practice 4/ video 5--");


    //Prelude: This would be the stuff that we don't need to import to work. 
    //Like println of the example. 

    //To import we do the following:  it would be a create. Very similar to a package in c#
    //We use modules of the crates.

    let mut input = String::new(); //:: it's a separator that will tell the code that we want to access a module.
    //THis is a mutable string.

    io::stdin().read_line(&mut input).expect("failed to read line"); //to input values we send as a parameter a mutable reference of our input variable.
    //If we send the input directly without the $mut it will just be a copy and wouldn't let us change anything.
    //The expect will catch any errors. Basically thats a catch.

    //This is just to check the print
    println!("{}",input);
}
