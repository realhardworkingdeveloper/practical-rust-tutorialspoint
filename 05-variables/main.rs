fn main() {
    // default datatype is immutable
    // should use mut keyword
    let mut fees:i32 = 25_000;
    println!("fees is {} ",fees);

    fees=35_000;
    println!("fees changed is {}",fees);

    // constant data value is uppercase
    const USER_LIMIT:i32=100; // Declare a integer constant
    const PI:f32 = 3.14;//Declare a float constant
    
    println!("user limit is {}",USER_LIMIT); //Display value of the constant
    println!("pi value is {}",PI); //Display value of the constant

    // optional mutation
    // let salary = 100.00;
    // let salary = 1.50 ; // reads first salary
    // println!("The value of salary is :{}",salary);

    let uname="Mohtashim";
    let uname= uname.len();
    println!("name changed to integer : {}",uname);

    // Unlike variables, constants cannot be shadowed.
}