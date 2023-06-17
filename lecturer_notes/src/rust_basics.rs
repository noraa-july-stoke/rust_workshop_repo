#![allow(dead_code)]
/*

Now that we have installed the Rust toolchain, let's go over some really basic items in rust:

macros: println!(), format!(), vec![], etc.

    -macros are used to generate code at compile time. They are similar to functions, but they
     are called with a ! at the end of their name. They are used to generate code that would
     be tedious to write by hand. For example, the println!() macro generates code that prints
     a string to the console. The format!() macro generates code that creates a String from
     a format string and a list of arguments. The vec![] macro generates code that creates
     a Vec<T> from a list of items. A Vec is a growable array. We will cover these in more
    detail later.

functions: fn main() {}, fn hello() -> String {}, etc.

    - Functions that return a value have a return type specified after the ->. Functions that
      do not return a value have a return type of (). The main() function is special in that
      it is the entry point of the program. It is the first function that is called when the
      program is run. It must have a return type of ().

    - functions are declared with the fn keyword followed by the name of the function, a list
      of parameters, and the return type. The body of the function is contained in curly braces.
      The last expression in the function is the return value. If you want to return early from
      a function you can use the return keyword followed by the value you want to return.
      Functions can be called by using their name followed by a list of arguments in parentheses.
      If the function returns a value you can assign it to a variable or use it in an expression.
      If the function does not return a value you can call it as a statement.

access modifiers: pub, etc.

    - access modifiers are used to control the visibility of items. They are used to specify
      whether an item can be accessed from outside the current module. The pub keyword is used
      to make an item public. If an item is not marked as public it is private by default. This
      means that it can only be accessed from within the current module. Any attempt to access
      a private item from outside the current module will result in a compiler error.

types: i32, f64, String, Vec<i32>, etc.

    - types in rust are similar to types in other languages. They are used to specify the type
      of a variable or function parameter. They can also be used to specify the return type of
      a function. Types are specified after the name of the variable, function parameter, or
      function return type. Types can be generic. For example, Vec<T> is a generic type that
      can be used to create a Vec of any type. We will cover generics in more detail later.

    - Rust probably contains many more types than you are used to, even compared to some other
      compiled languages. This is because Rust is a statically typed language. This means that
      the compiler needs to know the type of every variable at compile time. This allows the
      compiler to catch many errors at compile time that would otherwise be caught at runtime.
      You cannot change types at runtime in Rust. For example, you cannot assign an integer to
      a variable that has a type "string" during runtime based on, say a user's erroneous input.
      While in javascript, for example this might not necessarily cause an error, we may just
      get unexpected behavior, or a result such as NaN. In Rust, if the possibility for an
      erroneous user input hasn't been explicitly covered and handled, the progrram will panic
      and crash, due to a compile time error.

    - In addition to being statically typed, Rust is also strongly typed. This means that the
      compiler will not automatically convert between types. For example, you cannot add an
      i32 to an f64 without explicitly converting one of them to the other type. This is because
      the compiler does not know which type you want to convert to. You can convert between types
      by using the as keyword followed by the type you want to convert to. For example, 5 as f64
      will convert the integer 5 to a floating point number. We will cover type conversions in
      more detail later.

    - the basic types you want to be aware of for now are: i32, f64, String, Struct, and Enum. i32
      is a 32-bit signed integer. f64 is a 64-bit floating point number(a number containing a decimal
      point). String is a growable string. Struct is a user-defined type that can contain multiple
      values. Enum is a user-defined type that can contain multiple variants. We will cover structs
      and enums in more detail later.

variables: let x = 5, let mut x = 5, etc.

    - variables in rust are declared with the let keyword followed by the name of the variable,
      an optional type, and an optional value. If you do not specify a type the compiler will
      infer the type from the value. If you do not specify a value the compiler will initialize
      the variable to a default value. If you do not specify a type or a value the compiler will
      give you an error. Variables are immutable by default. This means that you cannot change
      their value after they have been initialized. If you want to be able to change the value
      of a variable you can use the mut keyword. This makes the variable mutable. You can also
      shadow a variable by using the let keyword again. This allows you to change the type of
      a variable without changing its name. We will cover shadowing in more detail later.

constants: const X: i32 = 5, etc.

    - constants in rust are declared with the const keyword followed by the name of the constant,
      a type, and a value. Constants are immutable by default. This means that you cannot change
      their value after they have been initialized. Constants must be initialized with a value.
      You cannot initialize a constant to a default value. Constants can be declared in any scope.
      Constants are always statically scoped. This means that they are valid for the entire time
      that the program is running. The name of constant should be SCREAMING_SNAKE_CASE. We will
      cover constants in more detail later.

code comments: //, /* */, etc.

    - comments in rust are similar to comments in other languages. They are used to document code.
      Comments are ignored by the compiler. Comments can be single line or multi line. Single line
      comments start with // and continue to the end of the line. Multi line comments start with
      /* and end with */. We will cover comments in more detail later.

One last note: Rust is pretty stric about basically everything. This includes unused variables,
unused imports, etc. If you don't use something, you will get a warning. If you don't use something
and you don't want to get a warning, you can prefix it with an underscore, like so: _x. This will
tell the compiler that you don't care about this variable, and it will not give you a warning.
You can also disable warnings for the entire file by adding the following line to the top of the file:
#![allow(dead_code)]
*/

// Okay, let's write some basic functions to get a feel for things.

pub fn main() {
    // macros: println!, format!, vec! etc.

    // println!() is a macro that prints a string to the console.
    println!("Hello, world!");

    // format!() is a macro that creates a String from a format string and a list of arguments.
    let s = format!("Hello, world!");
    println!("{}", s);

    // vec![] is a macro that creates a Vec<T> from a list of items.
    // Arrays in rust are not resizeable, but vectors are. Vectors are closer
    // to the arrays/lists you are used to working with in javscript and python.
    let mut v: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v);
    // now we add a new element to the vector
    v.push(4);
    println!("{:?}", v);

    // Here is the syntax for ways to destructure different things in Rust

    // Destructuring a tuple:
    let tuple: (i32, i32, i32) = (1, 2, 3);
    let (a, b, c) = tuple;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Desctructuring a struct:
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };
    let Point { x: a, y: b } = point;
    println!("a: {}, b: {}", a, b);

    // Destructuring an enum:

    enum Color {
        Red,
        Green(i32),
        Blue { alpha: f32, beta: f32 },
    }

    let color = Color::Green(42);
    match color {
        Color::Red => println!("Red"),
        Color::Green(value) => println!("Green: {}", value),
        Color::Blue { alpha, beta } => println!("Blue: {}, {}", alpha, beta),
    }

    // Array destructuring:

    let array = [1, 2, 3];
    let [a, b, c] = array;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // Vector destructuring: must be handled as a slice, and due to Rust's strictness with handling uncertainty with
    // variables that may or not be there, we need to do it like this...

    let vec = vec![1, 2, 3];
    if let [first, second, third] = vec.as_slice() {
        println!("first: {}, second: {}, third: {}", first, second, third);
    } else {
        println!("Unable to destructure Vec.");
    }

    //  Slice destructuring
    let slice = &[1, 2, 3];
    let [a, rest @ ..] = slice;
    println!("a: {}", a);
    for item in rest {
        println!("{}", item);
    }

    // Reference destructuring.
    let value = &42;
    let &x = value;
    println!("x: {}", x);
}
