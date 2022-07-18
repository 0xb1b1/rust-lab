fn main() {
    println!("---\nPRIMITIVE DATA TYPES\n---");
    let l_ptrstr: &str = "this is a string, wow";
    let l_string: String = l_ptrstr.to_string();
    let l_int: i32 = -1234;
    let l_parseint: i32 = "-11".parse().expect("NaN");
    let l_uint: u32  = 1234;
    let l_parseuint: u32 = "11".parse().expect("NaN");
    let l_boolean: bool = true;

    println!("l_ptrstr: {}\nl_string: {}\nl_int: {}\nl_parseint: {}\nl_uint: {}\nl_parseuint: {}\nl_boolean: {}",
             l_ptrstr,
             l_string,
             l_int,
             l_parseint,
             l_uint,
             l_parseuint,
             l_boolean);
    // (Im)mutable variables
    let age: u8 = 32;
    let mut m_age: u8 = 64;
    println!("age is {}", age);
    // age = 33;  will fail; vars are immutable by default
    println!("m_age is now {}", m_age);
    m_age = 64;
    println!("m_age is now {}", m_age);

    // Constants
    const ID: u16 = 1007;  // constants should be upper-case
    println!("ID: {}", ID);

    // Assign multiple vars at once
    let ( my_name, my_age ) = ("owo", 22);
    println!("my_name: {}\nmy_age: {}", my_name, my_age);

    // More primitive data types
    /* Integers: i{8-128}, u{8-128}
    Floats: f{32, 64}
    Boolean: bool
    Characters: char
    Tuples
    Arrays */
    // Rust is a statically typed language
    // It must know types of all vars at compile time
    // However, the compiler can usually infer
    // what type we want to use based on the value and
    // how we use it

    // Defaults to i32
    //let l_i32test = 1;

    // Defaults to f64
    //let l_f64test = 1.1;

    // Add explicit type
    // let l_expl_i64test: i64 = 2112312454313;

    // Find max size of primitive
    println!("Max i32: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = false;
    if is_active {
        println!("is_active is `true`!");
    } else {
        println!("is_active is `false`!")
    }

    // Get bool from expression eval
    let is_greater: bool = 100 > 5;
    if is_greater {
        println!("100 is greater than 5!")
    } else {
        println!("100 is less than 5!")
    }

    // char literal (unicode char)
    let l_char0: char = 'o';
    let l_char1: char = 'w';
    println!("{0}{1}{0}", l_char0, l_char1);

    // The two types of strings
    /* Primitive str is immutable, fixed-length string somewhere in memory
       String is growable, heap-allocated data struct for use when one needs to modify it or own string data */
    let mut l_hellostring: String = String::from("Hello");

    // Get length
    println!("{}'s length is {}", l_hellostring, l_hellostring.len());

    // Add to string (push(char) and push_str)
    l_hellostring.push(l_char0);
    println!("Hello is now {}", l_hellostring);

    l_hellostring.push_str(" world!");
    println!("Hello is now {}", l_hellostring);

    // Capacity in bytes
    println!("Capacity of l_hellostring: {}", l_hellostring.capacity());
    
    // Check if empty
    if l_hellostring.is_empty() {
        println!("l_hellostring is empty")
    } else {
        println!("l_hellostring is NOT empty")
    }

    // Contains
    println!("Contains `Hello`: {}", l_hellostring.contains("Hello"));
    println!("Contains `Helo`: {}", l_hellostring.contains("Helo"));

    // Replace
    println!("Replace: {}", l_hellostring.replace("world", "there"));

    // Loop through string by whitespace
    for word in l_hellostring.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut l_cap_string: String = String::with_capacity(10);
    l_cap_string.push('a');
    l_cap_string.push('b');

    println!("---\nASSERTION (silent but will fail if conditions are not met)\n---");
    // Assertion testing
    assert_eq!(2, l_cap_string.len());  // Will panic if the condition is not met
    assert_eq!(10, l_cap_string.capacity());

    println!("{}", l_cap_string);

    // Tuples
    /* Tuples group together values of different types
       Maximum 12 elements */
    
       let person: (&str, &str, u8) = ("voxel", "Dresden", 22);
       println!("{} if from {} and is {} yo", person.0, person.1, person.2);

    // Arrays
    // ...[type, len]
    println!("---\nARR\n---");
    let mut nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums);  // :? is the debug trait

    // Get a single value
    println!("{}", nums[0]);  // Looks almost too normal for rust

    // Print arr len
    println!("Arr is {} items long", nums.len());
    // Print arr space alloc
    println!("Arr nums occupies {} bytes in mem",
             std::mem::size_of_val(&nums));  // use std::mem can be used to omit std::

    // Loop through the arr
    for num in &nums {
        println!("{}", num);
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/for.html
    // Mutably iter through arr
    for num in nums.iter_mut() {
        *num += 2;  // *num = *num + 2;
        println!("num is now {}", num);
    }
    // Print new arr
    println!("\nNew arr:");
    for num in &nums {
        println!("{}", num);
    }

    // Get Slice
    let slice: &[i32] = &nums[1..3];
    println!("Slice: {:?}", slice);

    // Vectors
    // Vector - resizable arr
    println!("---\nVEC\n---");
    let mut v_num: Vec<i32> = vec![1, 2, 3];
    println!("{:?}", v_num);  // :? is the debug trait

    // Add on to vec
    v_num.push(123);
    v_num.push(321);

    // Pop from vec
    // https://stackoverflow.com/questions/64206004/vector-pop-returns-option
    // let popped_vec_val: i32 = v_num.pop().unwrap();  //-> if we know for a fact that pop will not return none
    let popped_vec_val = match v_num.pop() {
        Some(top) => top,
        None => { 0 }  // If returned None, return 0
    };

    println!("Popped {} from v_num", popped_vec_val);
    // Get a single value from vec
    println!("{}", v_num[0]);  // Looks almost too normal for rust

    // Print vec len
    println!("v_num is {} items long", v_num.len());
    // Print vec space alloc
    println!("v_num nums occupies {} bytes in mem",
             std::mem::size_of_val(&v_num));  // use std::mem can be used to omit std::

    // Loop through the vec
    for num in &v_num {
        println!("{}", num);
    }

    // Conditionals
    let c_age: u8 = 22;
    let checked_id: bool = false;

    // If/else
    if c_age >= 21 && checked_id {
        println!("Bartender: {} is old enough to drink", age);
    } else if c_age >= 21 && !checked_id {
        println!("Bartender: you are old enough! Lemme check your ID");
    } else if c_age < 21 && checked_id {
        println!("Bartender: yeah... I checked your ID and you are really not 21");    
    } else {
        println!("Bartender: the state mandated that you have to be at least 21 to drink, man")
    }

    // Shorthand if
    let is_of_age = if c_age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);

    // Loops
    println!("---\nLOOPS\n---");
    let mut counter: u8 = 0;
    let halt_point: u8 = 3;

    // Simple loop
    loop {
        counter += 1;
        println!("Number: {}", counter);
        if counter == halt_point { break; }
    }

    let mut count: u16 = 0;
    // While loop (FizzBuzz)
    while count <= 16 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count)
        }
        // Increment count
       count += 1; 
    }

    // For range loop
    for x in 0..16 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x)
        }
    }

    // Functions
    greeting("Hi", "voxel", 44);
    println!("Sum fn: {}", add(23, 45));

    // Closure (using outside vars)
    let aux_num: i32 = 3;
    let add_nums = |n0: i32, n1: i32| n0 + n1 + aux_num;
    println!("Anon func add_nums: {}", add_nums(12, 2));

    // Reference pointers
    // They point to a resource in memory
    
    // Primitive arr
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2 = arr1;
    println!("arr2 primitive: {:?}", arr2);

    // With non-primitives, if you assign another variable to a piece of data,
    // the first variable will no longer hold that value. You'll need to use a
    // reference (&) to point to the resource

    // Vector
    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2: &Vec<i32> = &vec1;

    // println!("Values of vec2: {:?}", (vec1, vec2))  // value used here after move
    println!("Values of vec2: {:?}", (&vec1, vec2));

    // Structs
    // Used to create custom data types
    let mut color0: Color = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    println!("color0: {} {} {}", color0.red, color0.green, color0.blue);

    // Change color0 values
    color0.red = 200;
    println!("color0: {} {} {}", color0.red, color0.green, color0.blue);

    // Tuple struct
    let mut colort0: ColorT = ColorT(255, 255, 0);
    println!("colort0: {} {} {}", colort0.0, colort0.1, colort0.2);

    // Change colort0 values
    colort0.2 = 13;
    println!("colort0: {} {} {}", colort0.0, colort0.1, colort0.2);

    // Person struct
    let mut person0 = Person::new("John", "Doe");
    println!("Person person0: {} {}", person0.first_name, person0.last_name);
    println!("Person person0 .full_name: {}", person0.full_name());

    // Change last name of person0
    person0.set_last_name("OwO");
    println!("Person person0 .full_name (new): {}", person0.full_name());

    // Use to_tuple() method of struct Person
    println!("Person person0 tuple: {:?}", person0.to_tuple());

    // Enums
    // ...are types which have a few definite vals
    let avatar0: Movement = Movement::Left;
    let avatar1: Movement = Movement::Up;
    let avatar2: Movement = Movement::Right;
    let avatar3: Movement = Movement::Down;

    move_avatar(avatar0);
    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);

    // CLI args
    let args: Vec<String> = std::env::args().collect();
    println!("Args: {:?}", args);
    let command: String = args[1].clone();
    println!("Command is: {}", command);
    if command == "Hello" {
        println!("world!");
    } else if command == "Hi" {
        println!("Hi {}, how are you?", args[2]);
    } else if command == "status" {
        println!("Status OK");
    } else {
        println!("{} is not a valid command!", args[1]);
    }
}

fn greeting(greet: &str, name: &str, age: u8) {
    println!("{} {}, you are {} yo!", greet, name, age)
}

fn add(n0: i32, n1: i32) -> i32 {
    n0 + n1
}

// Traditional struct
struct Color {
    red: u8,  // 0..256
    green: u8,
    blue: u8
}

// Tuple struct
struct ColorT(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String
}
impl Person {
    // Construct Person (method)
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last_name: &str) {
        self.last_name = last_name.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving down"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right")
    }
}
