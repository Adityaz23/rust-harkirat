// // Structs let's you generate the objects just like javascript let's you do the same.
// // First you will have to define the structs.
// // First,you will also have to define the type of the structs just like we do in the typescript.


// struct User{
//     active: bool,
//     name: String,
//     age: i32,
//     email: String,
//     sign_in: u32
// }

// struct Own{
//     active: bool,
//     sign_in: u32,
// }
// fn main(){
//     let mut user1 = User{
//         active: false,
//         name: String::from("Aditya Soni"),
//         age:22,
//         email: String::from("soniadityakumar651@gmail.com"),
//         sign_in: 12,
//     };
//     user1.age = 23; // here we can update the value of the objects but you will have to define the object as mutable.
//     println!("User1 name: {:?}",user1.name);
//     println!("User1 is active or not: {:?}",user1.active);
//     println!("User1 age: {:?}",user1.age);
//     println!("User1 age: {:?}",user1.age);
//     println!("User1 email: {:?}",user1.email);
//     println!("User1 sign_in or not: {:?}",user1.sign_in);

//     let user2 = Own{
//         active: false,
//         sign_in: 42
//     };
//     print_own(user2);
//     // println!("User 2 is the exxample of the ownership: {:?}",user2.active); // you cannot pass the value of the ownership of the struct to the other function and cannot pass it.

// }

// fn print_own(user2:Own){
//     println!("{:?}",user2.active);
//     println!("{:?}",user2.sign_in);

// }
// //Q. Does the struct is stored in heap or stack?
// // Ans. They are stored in stack.

// Now, we will learn the copy trait->

// #[derive(Clone, Copy)]
// struct Copies{
//     active: bool,
//     sign_in: i32,
// }

// fn main(){
// let copies2 = Copies{
//     active: true,
//     sign_in: -23,
// };
// print_bool(&copies2); here we are giving reference to the copies2 value.
// {
//     println!("{}",copies2.active); // then this will run.
// }
// }
// fn print_bool(copies2: &Copies){
//     println!("{}",copies2.active); // first this will run.
//     println!("{}",copies2.sign_in); // then this will run.
// }

// Code for adding strings ->
// #[derive(Clone)]
// struct  User{
//     active: bool,
//     sign_in: i32,
//     username: String,
// }
// fn main(){
//     let user1 = User{
//         active: true,
//         sign_in: -12 ,
//         username: String::from("Aditya"),
//     };
//     println!("Is active or not: {}",user1.active);
//     println!("Is integer or not: {}",user1.sign_in);
//     println!("Username: {}",user1.username);
//     change_user(&user1); // after borrowing the value now we are giving it to the user1 which is getting passed on the change_user() function.
//     println!("{}",user1.username);
// }
// fn change_user(user1:&User){ // here we borrowed the value of the struct user to the function.
//     println!("{}",user1.username);
// }

// now the same above example but using the clone trait ->

// #[derive(Clone)]
// struct  User{
//     active: bool,
//     sign_in: i32,
//     username: String,
// }
// fn main(){
//     let user1 = User{
//         active: true,
//         sign_in: -12 ,
//         username: String::from("Aditya"),
//     };
//     println!("Is active or not: {}",user1.active);
//     println!("Is integer or not: {}",user1.sign_in);
//     println!("Username: {}",user1.username);
//     change_user(&user1.clone()); // here we have used the clone trait by which we have implemented the clone trait.
//     println!("{}",user1.username);
// }
// fn change_user(user1:&User){ // here we borrowed the value of the struct user to the function.
//     println!("{}",user1.username);
// }

// now we can also use the struct and implement them like giving the value to the struct and then using the keyword impl to use them , in simple it means we cana ttach the function to the struct ->
// This is just as classes in javascript and typescript.

// struct User{
//    width: u32,
//    height: u32,
// }
// impl User {
//     fn area(&self)-> u32{
//         self.width * self.height
//     }
// }

// struct Impl{
//     pie: f64,
//     radius: f64,
// }
// impl Impl {
//     fn para(&self)->f64{
//         self.radius*self.radius*self.pie
//     }
// }
// fn main(){
//     let rear = User{
//         width: 23,
//         height: 12,
//     };
//     println!("The area of rectangle is: {:?}",rear.area());
//     let area = Impl{
//         radius: 23.4341,
//         pie: 3.14156
//     };
//     println!("{:?}",area.para());
// }

// // Now, enums , they are similar to the enums in typescript , they allows you to define a type by enumerating its possible variant.
// enum Directions {
//     north,
//     south,
//     west,
//     east,
// }
// fn main(){
//     let my_direction = Directions::north; // this is used in the game logic implementation.
//     let new_direction = my_direction; // no error cause they are a copy.
//     move_around(new_direction);
// }
// fn move_around(direction: Directions){

// }

// Now, enum with values ->
    // Define an enum called shape :-

  #[derive(Debug)]
    enum Shape{
        Circle(f64),
        Square(f64),
        Rectangle(f64,f64),
    }

    fn cal_area(shape: &Shape)->f64{
        match shape {
        Shape::Circle(r) => std::f64::consts::PI * r * r,
        Shape::Square(s) => s * s,
        Shape::Rectangle(w, h) => w * h,
    }
    }
    fn main(){
        let circle = Shape::Circle(5.0);
        let square = Shape::Square(5.0);
        let rectangle = Shape::Rectangle(5.0,6.0);

        println!("{:?} area ={}",circle,cal_area(&circle));
        println!("{:?} area ={}",square,cal_area(&square));
        println!("{:?} area ={}",rectangle,cal_area(&rectangle));
    }
