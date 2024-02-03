const DEATH_STAR_POINTS: u32 = 1_00_000; // constant variable

fn main() {
    // Variables
    // variable names are usually snake case
    let _character_name = "Luke Skywalker"; // immutable variable
    // character_name = "Han Solo";  Error: cannot assign twice to immutable variable `character_name`
    let _character_name = "Darth Vader"; // shadowing

    // while shadowing the variable name, the type of the variable can be changed
    let _character_name = 20; // shadowing

    // let mut age = 20; // mutable variable
    // age = 21; // warning: value assigned to `age` is never read
    println!("Hello, {} , Points {}", _character_name, DEATH_STAR_POINTS);

    // Tuple
    let my_profile = ("Leia Organa", 20u8);
    let (name, age) = my_profile;
    println!("Name: {} , Age: {}", name, age);

    // Ownership
    // moving happens using trait(mixin) called copy
    let sith_name = String::from("Darth Maul");
    let jedi_name = sith_name; // moving
    // here basically both sith_name and jedi_name are pointing to the same memory location
    // println!("{}", sith_name); // Error: value borrowed here after move
    println!("{}", jedi_name);

    // same example will work for integers
    // reason for this is that integers are stored on the stack and strings are stored on the heap
    // so when we assign sith_name to jedi_name, the value of sith_name is moved to jedi_name and sith_name is no longer valid
    // but in the case of integers, the value is copied to y, and x is still valid since stack memory is copied
    let x_wing = 5;
    let y_wing = x_wing;
    println!("X-Wing: {} , Y-Wing: {}", x_wing, y_wing);

    // THE WAY
    let sith_name = String::from("Kylo Ren");
    let _jedi_name = &sith_name;
    // now the pointer is copied to jedi_name and sith_name is still valid
    println!("{}", sith_name); // works fine
    println!("{}", _jedi_name); // works fine

    let mut rebel_message = String::from("Hello");
    string_reference(&rebel_message);
    mutable_string_reference(&mut rebel_message);
    // we can not make more than one mutable reference to the same memory location

    /*
    Data Rance condition prevention in rust

    let mut rebel_message = String::from("Hello");
    let r1 = &rebel_message;
    let r2 = &mut rebel_message;

    Error: cannot borrow `rebel_message` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);
    */

    // Dangling References
    // let _dangling_reference = get_character_name();
    // println!("{}", _dangling_reference);




}

fn string_reference(s: &String) {
    // here the reference is passed to the function, so the ownership is not moved
    println!("Rebel Message: {}", s);
}

// this
fn mutable_string_reference(s: &mut String) {
    // here the reference is mutable so the value can be changed
    s.push_str(" World");
    println!("Updated Rebel Message: {}", s);
}


// Missing Lifetime Annotations
// ths is a dangling reference
// fn get_character_name() -> &String {
//     &"Obi-Wan Kenobi".to_string()
// }
