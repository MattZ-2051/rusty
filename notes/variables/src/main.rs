fn main() {
    println!("Hello, world!");

    // To change let variables use mut keywork this makes them "mutable"
    // but only with same time
    // Valid
    let mut x = 5;
    x = 6;
    // Inavlid
    let x = 5;
    x = 6;

    // Constants are immutable and have to be set to an expression and typed
    // not a value that is receeived at run time (like a return from a func)
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // Shadowing is similar to assiging a new value like let mut but we can change the type when re assigning it
    // we have to use the let keyword to reassign it or we will get a compile time error

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
