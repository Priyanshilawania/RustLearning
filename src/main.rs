fn main() {

//     let a=10;
//     let b=15;
//     println!("Hello, world! , {} {}",a,b);


//unsigned integer~~~~~
// //u8,u16,u32,u64,u128
// let unsigned: u8 = 10;


// // signed integer~~~~~~
// //i8, i16, i32, i64, i128
// let signed: i8= -10;

// //float is used for decimals~~~~~~~~~~
// let float: f32 = 1.0;
// println!("unsigned: {} sign:{} float:{}", unsigned,signed,float);

// //char -canonly be~~~~~~~
// let letter="c";
// let emoji ="\u{1f600}"; //:D


// println!("letter: {}, emoji:{}", letter,emoji);

// let is_true: bool = true;

// println!("isTrue: {}", is_true);



//ARRAYS~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// let arr: [u8; 3] = [1,2,3];
// let other_arr: [u8; 5] =[100;5];


// println!("index: {}, length: {}", arr[0], other_arr.len());

// //print structure of array and other objects
// println!("{:?}", other_arr);
// 


// //variable Tuple~~~~~~~~~~~~~~~~~

// let tuple: (u8, bool, f32)=(5, true, 2.1);
// let tuple2= (3,5);

// //print structure of array and other objects
// println!("first {}, second{},third{}",tuple.0,tuple.1,tuple.2);
// println!("{:?}", tuple2);

// let(a,b,c) = tuple;

// // destructuring
// println!("first{},second{},third{},a,b,c");



//FUNCTIONS~~~~~~~~~~~~~~~~~~~~~~~~~

println!("{}", is_even(2));

}

pub fn is_even(num: u8) -> bool {
    let digit: u8 = num% 2;
    digit == 0 // return bool
}



 
