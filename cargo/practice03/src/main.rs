fn main() {
    println!("--This is practice 3--");

    let x:i32 = 2; //Intenger i8 i16 i32 i64 i128 this work the same with uX
    //The same type of intengers uX and iX (TODO add notes on how it works different)
    let a:f32 = 2.0;
    let a:f64 = 2.00;
    //f32 normal float values
    //f64 would be the same as double in c#
    let b:bool = true; //Exactly like how the rest works

    //No really need to have an example, but char works the same as C#.
    //We can do implicity declaration.

    let tup = (0,true,'s'); //tupple with implicit assigment.
    let tup2: (i32,bool,char) = (0,true,'s'); //Same with express sassigment.
    //from the other example, we cannot just print the variable directly.
    println!("{}",tup.0); //We do the index like this.
    println!("{}",tup.1);
    println!("{}",tup.2);

    //Same rules about mutability applies here.

    let mut aTup = (0,1,2);
    println!("{}",aTup.0); 
    println!("{}",aTup.1);
    println!("{}",aTup.2);
    //We can now mutate the values.
    aTup.0 = 15;
    println!("{}",aTup.0); 

    //Arrays
    let arr = [1,2,3,4,5];
    //We use square brackets instead of parenthesis.
    //The biggest difference like in other programming languages, is that arrays need to have the same type of variables.
    let mut arrMuyt = [1,2,3,4,5];
    println!("{}",arrMuyt[0]); 
    arrMuyt[0] = 15;
    println!("{}",arrMuyt[0]); 

    //How to initalize the array
    //let mut anotherArray: [char;5];
    //anotherArray[0] = 'a';
    //println!("{}",anotherArray[0]); 
    //This doesnt work as the array needs to be initialized, not just declared.

    //Now we data types, if we get the value of another variable inside a new one, it copies it's type;
    let x: u8 = 4;
    let y = x;

    println!("----------------------");
}
