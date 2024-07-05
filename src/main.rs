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

// println!("{}", is_even(2));

// }

// pub fn is_even(num: u8) -> bool {
//     let digit: u8 = num% 2;
//     digit == 0 // return bool
// }

//MUTABILITY~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// LET mut NUM=5;//using the keyword mut
// num=3;
// println!("{}",num);


//ARRAYS AND SLICES~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// let arr =[0,1,2];//know the length
// let slice= &arr[1..3];//[1,2] dont know the length 
// borrowing_slice(arr,slice);

// }
// fn borrowing_slice(arr:[u8; 4], slice: &[u8]){

//     println!("{:?}", arr);
//     println!("{:?}", slice);
//     println!("length: {}", slice[1]);
//     //slice
//     let num = 5;

//STRINGS~~~~~~~~~~~~~~~~~~~~~~~~
// let str: &str = "hello world";
// let string: String = String:: from("Hello World");


// let slice = &string[..6];
// slice.len();

// string.push('1');
// string.push_str("! Bob");
// string=string.replace("hello","bye");
// println!("{}", string);




//for statements~~~~~~~~~~~~~~~~~~~~~~~~~~~~

//if statements~~~~~~~~~

// let n = 3;
// if n>0{
//     println!("greater than 0");

// }
// else if n<0 {
//     println!("less than 0");
// }
// else{
//     println!("is 0")
// }

//FOR LOOP~~~~~~~~~~~~~~~~~~~~~~~~~

// for i in 0..6{
//     println!("{}", i);

// }
//}

//WHILE LOOP~~~~~~~~~~~~


// let mut i=0;
// while i< 4{
//     println!("{}",i);
//     i +=1;
//     if i==3{
//         println!("exit");
//         break
//     }



// MATCH STATEMENT~~~~~~~~~~~~~
LET I =5;
match i {
    0=> println!("0"),
    1|2=> println!("1,2"),
    3..=4=> println!("3,4"),
    _ =>println!("default")
}

 }


 
