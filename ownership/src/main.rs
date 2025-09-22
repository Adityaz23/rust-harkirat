// fn main() {
    // println!("Hello, world!");
    // println!("Hi");
    // let a = 123;
    // let b = 12342;
    // let c/ = 13413;
//    let d = (a+b)-c;
    // println!("d is :{}",d);

    // So, the stack example are ->
    // let x = 1;
    // this will give an error saying that the variable y is not in the scope.
    // {
    // let y = 4;
    // }
    // let z = x+y;
    // now we will do the scoping in the same variable.
    // println!("{}",y);
    // println!("{}",z); // in here the main function is the owner of the x and y and the owner is z
    // sum(12, 21);
// }
// fn sum(a:i32,b:i32){
    // and here the owner of a and b is c variable.
    // let c = a+b;
    // println!("{}",c);
// }

// the all above examples are for stack and now the below one are heap's.
// Rmemeber one thing that heap will aoways works with string and vectors.
fn main(){
    // let mut  s1 = String::from("Hello");
    // s1.push_str(",Aditya");
    // let s2 = s1;
    // // s2 is not the owner of the variable then here the 1 become invalid for the string s1.
    // //println!("{}",s1); // borrow of moved value: `s1`
    // println!("{}",s2);

    // let my_string = String::from("Hello");
    // println!("{}",my_string); // right here it will not give any error cause we did not pass the takes_ownership function right now
    // takes_ownership(my_string);
    // println!("{}",my_string); // there is the error in this line of code cause the owner of this stirng has been changed to the fn takes_ownership,  but now we have given the ownership of the my_string to the takes_ownership so now we are getting the compile time error.
    // and you can not use the .clon() function because by default you ca not clone in the. heap.
    // println!("{}",my_string);

    let mut my_string =String::from("hello") ;
    my_string = takes_ownership(my_string);
    println!("{}",my_string);
}
fn takes_ownership(some_string: String) -> String{
    println!("{}",some_string);
    return  some_string;
}
