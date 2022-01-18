fn main(){
    println!("pi value is {}",get_pi());

    let mut no:i32 = 5;
    mutate_no_to_zero(&mut no);
    println!("The value of no is:{}",no);

    let name:String = String::from("TutorialsPoint");
    display(name); //cannot access name after display
    // println!("{}",name) // error: value moved
}

fn get_pi() -> f64{
    22.0/7.0
}

fn mutate_no_to_zero(param_no:&mut i32){
    *param_no =0; //de reference
}

fn display(param_name:String){
    println!("param_name value is :{}",param_name);
}