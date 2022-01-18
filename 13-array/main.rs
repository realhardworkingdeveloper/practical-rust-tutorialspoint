fn main(){
    let arr:[i32;4] = [10,20,30,40];

    println!("array is {:?}", arr);
    println!("array size is :{}", arr.len());

    for val in arr.iter(){
        println!("value is :{}",val);
    }

    update(arr);
    println!("Inside main {:?}",arr);

    let arr1:[i32;4] = [-1;4];  // -1, -1, -1, -1
    println!("array is {:?}", arr1);
    println!("array size is :{}", arr1.len());

    let mut arr2:[i32;4] = [10,20,30,40];
    arr2[1]=0;
    println!("{:?}",arr2);
}

fn update(mut arr:[i32;4]){
    for i in 0..4{
        arr[i]=0;
    }
    println!("Inside update {:?}",arr);
}