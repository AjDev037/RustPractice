fn main() {
    let x = 4; //variable, let works the same as var/let in c#/js, when we talk about no need to assign manually a type.
    //But this works as a constant it seems.
    println!("x is: {}",x);
    //We add "mut" to our variables to be able to change its value.
    let mut y = 5;
    println!("y is: {}",y);
    y = 6;
    println!("y is now: {}",y);
    //If we dont use the variables before mutating them, we'll get a warning it runs but will display the warning.
    //Same if we declare a varable mutable and we dont change it.
    let mut z = 0;
    z = 1;
    println!("z is: {}",z);
    //If we create another variable with the same name that works. Example x
    let x = 10;
    println!("x is now: {}",x);
    //This is not modifying X value, but we are going to create a new variable with the same name.
    //We can use the old X value to do an increment or modifying the new variable.

    //Now if there is an scope issue, like 2 variables with the same name, it will get the closest value in the scope and ignore the other.
    let m = 1;
    {
        let m = 51;
        println!("m is now in scope: {}",m);
    }
    //Anything inside the scope stays in the scope.
    println!("m is now: {}",m);
    //We can use the variable outside the scope inside the scope, similar to what we mentioned on using the first variable to modify the second one.
    {
        let m = m + 1;
        println!("m is now in scope modified: {}",m);
    }
    //We can re-declare the variable with another type (This is a fucking headache) so I really need to be careful with this.
    let x = "This is an example";
    println!("x is a string now: {}",x);
    //We still use constants with similar conventions, for example all caps. And needs to be defined, not just declared
    const THIS_IS_A_CONSTANT: u32 = 0;
    println!("const is: {}",THIS_IS_A_CONSTANT);
    //The biggest difference is that a constant cannot be re-defined, if we create one and then try to create a new one the same way we did with
    //unmutable variables, it throws an error.
    println!("This is practice #2");
}
