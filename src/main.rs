// use std::io::stdin;

// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("the value of x in the inner scope is: {x}");
    // }
    // println!("the value of x is: {x}");
    // let sum = 5 + 10;

    // let difference = 95.5 - 4.3;

    // let product = 4 * 30;

    // let quotient = 56.7 / 32.2;
    // let floored = 2 / 3;

    // let remainder = 43 % 5;

    // println!("{floored}");

    // let tup = (2, 2.4, 'c');

    // let (x, y, z) = tup;

    // println!("The value of x is {x}, y is {y}, z is {z}");

    // let arr = [1, 2, 3, 4, 5];

    // let arr2 = [3; 10];

    // let a = [1, 2, 3, 4, 5];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // stdin().read_line(&mut index).expect("Failed to read line");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}")

    // another_function(10);

    // print_label_measurement(32, 'e')

    // let a = 10;
    // let b = 5;

    // println!("{b}, {a}");

    // let c = five();
    // println!("{c}")

    // let number = 3;

    // if number < 5 {
    //     println!("condition was true");
    // } else {
    //     println!("condition was false");
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;

    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");

    //         if 9 == remaining {
    //             break;
    //         }

    //         if 2 == count {
    //             break 'counting_up;
    //         }

    //         remaining -= 1;
    //     }
    //     count += 1;
    // }

    // while count < 2 {
    //     println!("count = {count}");

    //     let mut remaining = 10;

    //     while remaining >= 9 {
    //         println!("remaining = {remaining}");
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }

    // println!("End count = {count}");

    // let arr = [1, 2, 3, 4, 5, 6];

    // let mut index = 0;

    // while index < arr.len() {
    //     println!("The element value is {}", arr[index]);
    //     index += 1;
    // }

    // for element in arr {
    //     println!("The element value is {element}");
    // }

    // for number in (1..4).rev() {
    //     println!("the value is {number}");
    // }

    // let mut s1 = "hello";

    // println!("s1 = {s1}");

    // let mut s2 = String::from("hello");

    // s2.push_str(", rust");

    // println!("s2 = {s2}");

    // let mut s1 = String::from("hello");
    // let mut s2 = s1;

    // s1.push_str(", rust");

    // println!("s1 = {}, s2 = {}", s1, s2);

    // let mut s1 = String::from("hello");
    // let s2 = s1.clone();

    // s1.push_str(", rust");
    // println!("s1 = {}, s2 = {}", s1, s2);

    // let s = String::from("hello");
    // move_string(s);
    // println!("s = {}", s); throw error

    // let x = 10;
    // clone_int(x);

    // println!("End: x is {}", x);

    // let s = String::from("hello");

    // let len = calc_str(s);

    // println!("s is {}", s);

    // let len = calc_str(&s);
    // println!("s is {}", s);
    // println!("len is {}", len);

    // let s = String::from("hello rust");
    // let len = first_word(&s);
    // println!("result: {len}");

    // let s = String::from("hello rust");
    // let hello = &s[0..5];
    // let rust = &s[6..10];
    // println!("{hello}- {rust}-");

    // let mut s = String::from("hello rust");
    // let res = first_word(&s);
    // s.clear(); error
    // println!("result: {}", res);

    // let mut user1 = User {
    //     active: false,
    //     username: String::from("ly_911"),
    //     email: String::from("again911@gmail.com"),
    //     sign_in_count: 0,
    // };

    // println!("user name: {}", user1.username);

    // user1.username = String::from("ly_911_new");
    // println!("New: user name: {}", user1.username);

    // let user2 = build_user(String::from("test user"), String::from("gl@911.com"));

    // println!(
    //     "user2 info: username:{}, email:{}",
    //     user2.username, user2.email
    // );

    // let user3 = User {
    //     username: String::from("user3"),
    //     ..user2
    // };

    // println!("user2 info: username:{},", user2.username,);

    // println!(
    //     "user3 info: username:{}, email:{}",
    //     user3.username, user3.email
    // );

    let rect1 = Rectangle {
        width: 20,
        height: 33,
    };

    println!("rect1 area is {}", rect1.area());

    let rect2 = Rectangle {
        width: 33,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 10,
        height: 20,
    };

    println!("rect1 hold rect2 ? {}", rect1.can_hold(&rect2));
    println!("rect1 hold rect3 ? {}", rect1.can_hold(&rect3));

    let ip = IpAddressKind::V4;

    println!("ip kind is : {:?}", ip);
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

// fn first_word(s: &String) -> &str {
//     for (i, &item) in s.as_bytes().iter().enumerate() {
//         if item == b' ' {
//             return &s[..i];
//         }
//     }

//     &s[..]
// }

// fn first_word(s: &String) -> usize {
//     for (i, &item) in s.as_bytes().iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn calc_str(s: &String) -> usize {
//     s.len()
// }

// fn move_string(s: String) {
//     println!("s = {}", s);
// }

// fn clone_int(x: i32) {
//     println!("x = {}", x);
// }

// fn another_function(x: i32) {
//     println!("Another function: {x}")
// }

// fn print_label_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn five() -> i32 {
//     5
// }

struct Rectangle {
    height: usize,
    width: usize,
}

impl Rectangle {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }
}

#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}
