// fn main() {
//     println!("Hello from the memory management");
// }

// Now we will be doing the jargons of the rust the jargons are :-

// 1. Mutability
// 2. Heap and memory
// 3. Ownership model
// 4. Borrowing and references.
// 5. Lifetime

/*
 Jargon 1:- Mutability =>Immutbale variables represent the variables whose values can't be changed one assigned.
    fn main(){
    let x = 32;
    x =2; Error causew the value of x is assigned which is 32 so can't assign 2 again
    }
    fn main(){
    let x = 23;
     x = 2;
    println!("{}",x);
        let x = 23;
    |         - first assignment to `x`
 23 |      x = 2;
    |      ^^^^^ cannot assign twice to immutable variable
    |
 help: consider making this binding mutable
    |
 22 |     let mut x = 23;
    |         +++
}
    By default, all varaibles in the rust are immutable because ->
    1. Immutable data is inherited thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
    2. Knowing that certain data will not change allows the compiler to optimize code better. 


    Jargon 2:- Stack vs Heap ->
    Stack vs. Heap Allocation
        Rust has clear rules about stack and heap data management:

        Stack is the fast way to allocate and deallocate, rust uses the stack for most o the primituve data types like int, i:32,f:64(cause here the size of the int is known at the compile time but not for the string.).

        Heap is used for the data that can grow at the compile time like string where you just added something at the last of the string then it will not get compile.

        Heap is much slower as compared to the stack.

        Q. What is stored in the stack?
        Ans. Number i:32,i:64,i:128 boolean true and false and fixed array size , the stack creates a stack frame for that integer number.

        Q. What is stored in the heap?
        Ans. Strings and vectors
*/
fn main(){
    // This is the heap and the string will be stored in the Heap data allocation.
    // let mut  adi = String::from("Hello aditya");
    // adi.push_str(" Soni");
    // println!("The push string is: {}",adi);
    stack_fn();
    heap_fn();
    update_string();
}

fn stack_fn(){
    let a = 10;
    let b = 13;
    let c = 15;
    let d = a+b+c;
    println!("The sum of a:{} b:{} and c:{} is: {}",a,b,c,d);
}

fn heap_fn(){
    let s1 = String::from("hello");
    let s2 = String::from("Aditya");
    let combine = format!("{} {}",s1,s2);
    println!("Heap function is: {}",combine);
}

fn update_string(){
    let mut s = String::from("real string");
    println!("Before update: {}",s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(),s.len(),s.as_ptr());
    // for printing the pointer you need to add the {:p} in the println
    // in the capacity even the empty space is count.
    // This three things are stored in the heap which stores the data.

  
    for _ in 0..100{
    s.push_str(" is actually added");
    // println!("After upadte: {}",s);
    println!("Capacity: {}, Length: {}, Pointer: {:p}", s.capacity(),s.len(),s.as_ptr());
    // the pointer will never change , and the capacity and the length will always be same.
    }
}