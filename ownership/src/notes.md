# Ownership is like to tell the rust when to drop variables and when to deallocate memory:-
**Ownership makes more sense on the memory on the heap**
The main thing to remember is that there is no memory management in the stack like there is in the heap.

**Heap variables are like they need an owner all the time once the owner is dead then the variable try to find the new owner.**

**Ownership helps in the dangling pointers and the double free errors**

Q.Why ownership is helpful?
Ans. Memory maangement is very hard that's why onwership comes handy for the heap allocation.
