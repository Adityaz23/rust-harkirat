# Memory Management :- **Whenever you run a program it allocates and deallocates memory on the RAM.**
## We will see how doe smemory management works in three ways :-

1. **Garbage Collector** :- Written by smart people , usually no dangling pointers/ memory issue, you can't do manual memory management Ex:- *Javascript, Java*

2. **Manual** :- You allocate and dellocate memory yourself , can lead to dangling pointers/memory issue, learning curve is high since you have to do manual Ex:- *C*

3. **The rust way** :- Rust has its own ownership model for memeory management, makes it extemely safe to memeory issues.

## Memory management is a crucial aspect of programming in Rust, designed to ensure *safety and efficiency* without the need of garbage collector.

## Not having a garbaage collector is one of the reasons rust is fast:-
It achieves the using the :-
1. Mutability
2. Heap and memory
3. Ownership model
4. Borrowing and references.
5. Lifetime