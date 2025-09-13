fn main() {
    println!("Hello, world!");
    /*
        the first reason to use the rust instead of nodejs is typesafety :-
        let x = 1;
        x = aditya;
        console.log(x);
        Output -> harkirat what about x?
     */
    // let x = 1;
    // // x = "aditya"; //mismatched types expected integer, found `&str` this is a compliation error.
    // let f = 1000.00;
    // let y = -34;
    // // let a: u32 = -1000; // an usgined number needs to be postivie.
    // let a: u32 = 12321;
    // println!("x:{},y:{},f:{},a:{}",x,y,a,f);
    // Q. What if the code overflow?
    // Ans. the compiler will not let you compile the code until you give tit the same type is the data type which you are using.
    // Ex:-
//     let b:i8 = 12341414;
//     literal out of range for `i8`
//   --> main.rs:20:16
//    |
// 20 |     let b:i8 = 12341414;
//    |                ^^^^^^^^
//    |
//    = note: the literal `12341414` does not fit into the type `i8` whose range is `-128..=127`
//    = help: consider using the type `i32` instead
//    = note: `#[deny(overflowing_literals)]` on by default


// let mut a = 10;
// for _i in 0..1000{
//     a = a+100; // this will give the error because we cannot use the a again to use it agian we will have to use the mut
//     println!("{}",a);

let is_male = true;
let is_above_18 = false;

if is_male{
    println!("You are a male")
}
else {
    println!("You are a female")
}
if is_male && is_above_18{
    println!("You are legal")
}

// String :-  the value of string can be modified by the runtime.
let greet = String::from("Hello Aditya");

let char1 = greet.chars().nth(10);
// println!("{}",char1.unwrap()); this will give the error called 
// it got panicked :-
// thread 'main' panicked at src/main.rs:54:21:
// called `Option::unwrap()` on a `None` value
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

println!("Char: {:?}",char1);

let is_even = false;
if is_even{
    println!("Even number")
}
else if !is_even {
    println!("Odd number")
}

// the three things for which we need to iterate over with the loops are arrays,strings and maps.


let n = 100;
for i in 0..n{
    println!("Hello World: {}",i);
}
let sentence = String::from("Aditya from the main function.");
let first_word = get_first_word(sentence);
println!("First word is: {}", first_word);

// fn calling 
let answer = do_sum(12, 23);
println!("The sum of a and b is: {}",answer);
// both does the same thing but the only difference is that the above is really small and we are directly giving the value to the integers deirectly in the argumnets of the funciton .
// the other way to call the function for the sum of two numbers:-

// let a =10;
// let b = 23;
// let sum = do_sum(a, b);
// println!("The sum of 2 numbers a and b are: {}",sum);
}
// this is how you create a function.
// the only thing to remember is that the return type must be given by yourself it cannot be inferred like int x:i32 = 23; no you will have to give it on your own.
fn do_sum( a:i32, b:i32)-> i32{
    return a+b;
}

fn get_first_word(sentence: String)-> String{
 let mut ans = String::from("");
 for char in sentence.chars(){
    ans.push_str(char.to_string().as_str());
    if char == ' ' {
        break;
    }
 }
 return ans;
}

