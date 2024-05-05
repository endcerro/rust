//https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html
#[derive(Debug)]
struct Rectangle {
	width : u32,
	height : u32
}
impl Rectangle {
	fn area(self: &Self) -> u32{
		self.height * self.width
	}
	fn can_hold(&self, other : &Rectangle) -> bool{
		self.width > other.width && self.height > other.height
	}
	fn square(size : u32) -> Self {
		Self {
			width : size,
			height: size
		}
	}
}

enum IpAddrKind {
	V4(u8,u8,u8,u8),
	V6(String)
}

// fn area(width :u32, height: u32) -> u32{
// 	width * height
// }
// fn tuple_area(dimension: (u32,u32)) -> u32{
// 	dimension.0 * dimension.1
// }
// fn struct_area(rec: &Rectangle) -> u32{
// 	rec.width * rec.height
// }

enum Message {
	Quit,
	Move { x: i32, y: i32},
	Write(String),
	ChangeColor(i32,i32,i32)
}	

impl Message {
	fn call(&self){

	}
	
}

fn main() {
	// let width : u32 = 20;
	// let height : u32 = 30;

	// let tuple_rec = (30,50);
	// let struct_rec = Rectangle {
	// 	width : dbg!(10 * width),
	// 	height : 10
	// };
	let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

	// println!("The area is {}",area(width, height));
	// println!("The tuple area is {}",tuple_area(tuple_rec));
	// println!("The struct area is {}",struct_area(&struct_rec));
	// println!("The struct  {:#?}",struct_rec);
	// dbg!(&struct_rec);
	// println!("The struct and its method  {}",struct_rec.area());

	print!("rect1 can hold rect2 ? {}\n",rect1.can_hold(&rect2) );
	print!("rect1 can hold rect3 ? {}\n", rect1.can_hold(&rect3));
	print!("rect2 can hold rect3 ? {}\n", rect2.can_hold(&rect3));

	let _square : Rectangle = Rectangle::square(10);

	let four : IpAddrKind = IpAddrKind::V4(127,0,0,1);
	let six : IpAddrKind = IpAddrKind::V6(String::from("::1"));

	let m : Message = Message::Write(String::from("Hello world"));
}

fn route(ip_kind : IpAddrKind){

}