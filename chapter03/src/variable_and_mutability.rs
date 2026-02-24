/// # Variables and Mutability
///
/// By default, when create a new variable in rust then this is immutable, you can't change value of them
/// However, you can make variable to mutable by adding 'mut' in front of the variable name when you create them
///
/// ## Example
///
/// ```
/// let x = 5
/// println!("The value of x is: {x}");
///
/// let mut y = 5
/// println!("The value of y is: {y}");
/// y = 6;
/// println!("The value of y is: {y}");
/// ```
///
/// # Shadowing
///
/// You can shadow a variable by using the same variable’s name and repeating the use of the let
/// keyword Shadowing is different mutable, Shadowing creating a new variable when we use the let
/// keyword again, we can change the type of the value but reuse the same name. Additional, By
/// using let, we can perform a few transformations on a value but have the variable be immutable
/// after those transformations have completed.
///
/// ## Example
///
/// ```
/// let x = 5;
/// let x = x + 1;
/// {
///     let x = x * 2;
///     println!("The value of x in the inner scope is: {x}");
/// }
/// println!("The value of x is: {x}");
///
/// let spaces = "   ";
/// let spaces = spaces.len();
///
/// let mut spaces = "   "; //Error
/// spaces = spaces.len();
/// ```
///
/// # Declaring Constants
///
/// Constants are values that are bound to a name and are not allowed to change, but there are a few
/// differences between constants and variables.
/// First, you aren’t allowed to use mut with constants. Constants aren’t just immutable by
/// default—they’re always immutable. You declare constants using the const keyword instead of the
/// let keyword, and the type of the value must be annotated.
/// Constants can be declared in any scope, including the global scope, which makes them useful
/// for values that many parts of code need to know about.
///
/// ## Example
/// ```
/// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
/// ```
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
pub fn variable_and_mutability() {
    let x = 5;
    println!("The value of x is: {x}");

    let mut y = 5;
    println!("The value of y is: {y}");
    y = 6;
    println!("The value of y is: {y}");

}
