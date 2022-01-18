fn main() {
    let company:&str="TutorialsPoint";
    let location:&str = "Hyderabad";
    
    println!("company is : {} location :{}",company,location);

    let empty_string = String::new();
    println!("length is {}",empty_string.len());
    
    let content_string = String::from("TutorialsPoint");
    println!("length is {}",content_string.len());
}