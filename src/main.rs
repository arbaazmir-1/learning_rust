use std::{io::Read, vec};

use module::function_module;

mod module {
    pub mod function_module;
}
use std::io::stdin;
// // variables
fn variables() {
    //first lesson variables in rust

    //definitions
    let x = 10000;
    println!("x is : {x}");
    // mutability  - mut keyword before variable to make it changeable
    let mut y = "text";
    y = "Test";

    //scope just like js baby
    {
        let z = 20;
    }
    //accessing z outside of that scope will generate error

    //shadowing
    let t = 20;
    let t = t + 20;
    println!("t is : {t}");

    //constants are gonna scream baby
    const _HELP_ME: &str = "Dear God";
}

// data types

fn data_types() {
    // Scalar Types

    // Integer Types
    // Signed Integers can be negative and positive
    let i8: i8 = -128; // 8-bit signed integer
    let i16: i16 = -32768; // 16-bit signed integer
    let i32: i32 = -2147483648; // 32-bit signed integer
    let i64: i64 = -9223372036854775808; // 64-bit signed integer
    let i128: i128 = -170141183460469231731687303715884105728; // 128-bit signed integer
    let isize: isize = -9223372036854775808; // Pointer-sized signed integer (architecture-dependent)

    // Unsigned Integers only positive
    let u8: u8 = 255; // 8-bit unsigned integer
    let u16: u16 = 65535; // 16-bit unsigned integer
    let u32: u32 = 4294967295; // 32-bit unsigned integer
    let u64: u64 = 18446744073709551615; // 64-bit unsigned integer
    let u128: u128 = 340282366920938463463374607431768211455; // 128-bit unsigned integer
    let usize: usize = 18446744073709551615; // Pointer-sized unsigned integer (architecture-dependent)

    // Floating-Point Types
    let f32: f32 = 3.14; // 32-bit floating-point
    let f64: f64 = 3.141592653589793; // 64-bit floating-point

    // Character Type
    let c: char = 'A'; // A single Unicode scalar value (4 bytes)

    // Boolean Type
    let t: bool = true; // Boolean value, can be either true or false

    // Compound Types

    // Tuple
    let tuple: (i32, f64, u8) = (500, 6.4, 1);

    // Array
    let array: [i32; 3] = [1, 2, 3];

    // Slice
    let slice: &[i32] = &array[1..3];

    // String
    let s: &str = "Hello, world!"; // String slice (&str)
    let string: String = String::from("Hello, Rust!"); // Growable, heap-allocated string (String)

    // aliasing
    type Age = u8;
    let my_age: Age = 20;
    println!("my age is {my_age}");

    //compound data types
    let fixed_string: &str = "this cannot be changed"; // the cannot be modified or any values cannot be added to it
    let mut changeable_string: String = String::from("Life is good when you can add things");
    changeable_string.push('h');

    //array
    let new_number_array: [u32; 4] = [1, 3, 4, 5]; // this cannot be changed, the type definiation format is as such : [type of items in array; length]
    let mut changeable_array: [u8; 5] = [10; 5]; // this can be mutated and the array is the number 10 repeat over 5 times, format is : [number; number of items]
    changeable_array[2] = 4;

    // to print an array all you need is a question mark
    println!("{:?}", changeable_array);

    // but if you need an array that you can change you can use a vector Vec<i32> as it can grow and shrink but you gotta add mut to mutate it otherwise no change
    let mut some_vec: Vec<u32> = vec![123, 123, 123, 123];
    some_vec.push(202);

    //fun world of tuples
    let my_data: (&str, u32, &str, u32) = ("Random", 34, "Country Code", 97);
    // accessing is like js in someway
    let name = my_data.0;
    // destructing a tuple
    let (name_data, age, country, code) = my_data;
    // single element tuple
    let single_element_tuple = (42,);
    println!("{:?}", single_element_tuple); // Output: (42,)
}

fn main() {
    //calling function from another file
    function_module::life_name("Alice in wonder land");
    // making func
    let func = function_module::life_name;
    let name: &str = "Random";
    func(name);
    let num = multiply(20, 40);
    println!("Number is {num }");
    let multiple_return = cal_me(23, 43);
    println!("{:?}", multiple_return);
    let outer_variable = 10;
    // some notes
    // 1. Lexical Scoping
    // Rust uses lexical scoping, meaning that the visibility and lifetime of variables are determined by their location in the source code. Code blocks define scopes where variables are valid and accessible.

    // 2. Controlling Lifetime
    // Variables declared within a block exist only within that block's scope. When the block ends, the variables go out of scope, and their memory is deallocated. This allows for precise control over resource management and prevents memory leaks.

    // 3. Encapsulation
    // Blocks allow you to encapsulate related code together, making it easier to understand and reason about. They can be used to group statements logically, improving code organization and readability.

    // 4. Shadowing
    // Blocks enable variable shadowing, where you can declare a new variable with the same name as an existing variable in an outer scope. This can be useful for temporarily reusing a variable name without affecting the outer variable.

    // 5. Error Handling
    // Blocks are often used in error handling, especially with the Result and Option types. They allow you to handle errors locally and provide more context-specific error messages.
    {
        let inner_variable = 20;
        println!("Inner variable: {}", inner_variable);

        let outer_variable = 30; // Shadowing the outer_variable
        println!("Shadowed outer variable: {}", outer_variable);
    }

    // println!("Inner variable: {}", inner_variable); // Error: `inner_variable` is out of scope
    println!("Outer variable: {}", outer_variable);
    conditional_statements(96);
    control_flow()
}

// function that can return value
fn multiply(num1: i32, num2: i32) -> i32 {
    // to return we have to mention the type with -> arrow and remove the ; in the return statement or using the return keyword
    num1 * num2
}
// multiple return with tuples love
fn cal_me(num1: i32, num2: i32) -> (i32, i32, i32) {
    (num1 * num2, num2 * num2, num1 + num2)
}

fn conditional_statements(num1: u32) {
    if num1 > 40 {
        println!("life is good");
    } else {
        println!("we are doomed");
    }

    let marks = num1;
    let mut grade = if marks >= 90 {
        'A'
    } else if marks >= 80 {
        'B'
    } else {
        'F'
    };
    println!("Grade is {}", grade);
    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        _ => grade = 'F',
    }
    println!("Grade is {}", grade);
}

fn control_flow() {
    let vector_num = vec![12, 32, 45, 76];
    for i in vector_num {
        println!("{i}")
    }
    let mut num = 10;
    while num < 20 {
        num = num + 1;
    }

    let mut name: String = String::new();
    stdin().read_line(&mut name).expect("Something is wrong");
    let name: &str = name.trim();
    println!("Your name is {name}")
}

// ownership in rust

// taking ownership and giving back isn't what the convention of rust is. Rather borrowing the value is.
fn take_give_ownership(mut input_vector: Vec<u32>) -> Vec<u32> {
    // This function takes ownership of input_vector, modifies it by adding a value, and then returns it.
    // This means that ownership is effectively given back to the function that called this one.
    input_vector.push(100);
    input_vector
}

fn demonstrate_ownership_in_rust(input_vector: Vec<u32>) {
    // Create a string
    let original_string: String = String::from("Life is so good");

    // Clone the string to a new variable. This allows us to use the original string later,
    // as Rust's ownership model means the original string would be moved, not copied, if we assigned it to a new variable.
    let cloned_string = original_string.clone();

    // We can print the original string because we cloned it, not moved it.
    println!("{}", original_string);

    // Primitive data types like bool, char, int are stored entirely on the stack.
    // This means they are copied, not moved, when assigned to a new variable.
    let original_int = 15;
    let copied_int = original_int;

    // We can print the original int because it was copied, not moved.
    println!("original_int is {}", original_int);

    // When this function is called and the input_vector is passed (a variable set in the calling function),
    // the original vector will be cleared from memory. The input_vector in this function will persist until
    // this function finishes execution, at which point it too is cleaned up.
    println!("input_vector : {:?}", input_vector);
    // Pass ownership of input_vector to take_give_ownership function
    let mut modified_vector = take_give_ownership(input_vector);

    // We can still use modified_vector here because take_give_ownership returned it, giving ownership back to this function.
    println!("modified_vector : {:?}", modified_vector);

    // Demonstrate borrowing in Rust
    // Rule 1 is that we don't talk about fight club lol
    // Rule 1: At any given time, you can have either one mutable reference or any number of immutable references.
    // Rule 2: References must always be valid.

    // Borrow modified_vector as a mutable reference. This reference is valid until the end of the current scope.
    let first_mutable_reference = &mut modified_vector;

    // At this point, first_mutable_reference goes out of scope, so we can create a new mutable reference.
    let second_mutable_reference = &mut modified_vector;

    // This line would violate Rule 1 if uncommented, as it attempts to use two mutable references at the same time.
    // println!("{:?} {:?}", first_mutable_reference, second_mutable_reference);

    // we can have infinite number of immutable reference such as
    let infinite_one = &modified_vector;
    let infinite_two = &modified_vector;
    // let mutable_infinite_one = &mut modified_vector; // this will two an error as we can have either infinite immuatble or one mutable ref
    println!("{:?} {:?}", infinite_one, infinite_two); // no error thrown
    let mutable_infinite_one = &mut modified_vector; // this works here because the before two are out of scope

    // This block of code is trying to create a reference to a vector that is defined inside a scope, and then use that reference outside the scope.
    // However, this will not work in Rust due to its ownership rules.

    /*
    let some_vec = {
        // Define a vector inside this scope
        let vec_in_scope = vec![1, 2, 4];

        // Try to return a reference to the vector
        &vec_in_scope // This will throw an error because `vec_in_scope` does not live long enough
    };

    // Here, we're trying to use `some_vec` outside the scope where `vec_in_scope` was defined.
    // But `vec_in_scope` was dropped at the end of the inner scope, so `some_vec` is now a dangling reference.
    // Rust's borrow checker will give a compile-time error, preventing this.
    */

    // To pass a reference to a function in Rust, we use the "&" symbol in the type of the parameter.
    // This indicates that the function does not take ownership of the value, but instead borrows it for the duration of the function call.

    /*
    fn print_vector(vec_data: &Vec<u32>) {
        // This function borrows `vec_data` for the duration of the function.
        // It does not take ownership, so the original vector can still be used after this function call.
        println!("{:?}", vec_data);
    }
    */

    // we can call this function with a reference to a vector like this:
    /*
    let data = vec![1, 2, 3];
    print_vector(&data);
    */
}

fn dereference_in_rust() {
    // Declare a mutable variable `x` and initialize it with the value 10
    let mut x = 10;

    // Declare a mutable reference `y` to `x`
    let y = &mut x;

    // Dereference `y` to get the value of `x` and assign it to `something_else`
    // At this point, `something_else` is 10, and `x` is also 10
    let something_else = *y;

    // Dereference `y` and change the value of `x` to 20
    // Now, `x` is 20, but `something_else` is still 10 because it was assigned the value of `x` before `x` was changed
    *y = 20;

    print!("{} is not {}", something_else, *y); // This will print: "10 is not 20"

    // this won't work with a heap allocated data such as vec
}
