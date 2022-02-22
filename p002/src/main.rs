struct User {
    username: String,
    email: String,
    sign_in_cnt: u64,
    is_active: bool
}

// Tuple structs
struct Color(i32, i32, i32);

fn main() {
    let mut _user1 = User {
        email: String::from("email.email"),
        username: String::from("Keith"),
        sign_in_cnt: 1,
        is_active: true
    };

    let _name = _user1.username;

    let _user2 = build_user(
        String::from("myemail.com"), 
        String::from("Tyler Perry")
    );

    // Build structs using other structs
    let _user3 = User {
        username: String::from("Name"),
        email: String::from("Whatevr"),
        .._user2
    };

    let _color = build_color(55, 66, 77);

    let area = get_area((4, 10));
    println!("Area {}", area);

    let rectangle = Rectangle {
        width: 10,
        length: 2
    };

    let area2 = get_area_struct(&rectangle);
    println!("Area 2 {}", area2);

    // To print debug of struct, use #[derive(Debug)];
    println!("rectangle {:#?}", rectangle);

    println!("This is area method {}", rectangle.get_area_x());

    let rectangle2 = Rectangle {
        width: 2,
        length: 4
    };

    println!("Is rectangle one bigger than two? {}", rectangle.can_hold(&rectangle2));
    
}

fn build_user(email: String, username: String) -> User {
    User {
        //if function arguments are the same the field names, just use name
        email/*: email*/,
        username/*: username*/,
        is_active: true,
        sign_in_cnt: 2
    }
}

fn build_color(red: i32, green: i32, blue: i32) -> Color {
    Color(red, green, blue)
}

// Instead of making a function with multiple arguments, pass a tuple
fn get_area(dimension: (u32, u32)) -> u32 {
    dimension.0 * dimension.1
}

// Or pass a reference to a structure
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32
}

impl Rectangle {
    fn get_area_x(&self) -> u32 {
        self.length * self.width
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.get_area_x() >= rect.get_area_x() {
            true
        } else {
            false
        }
    }
}
// Associative functions. Does not take self as argument
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size
        }
    }
}
fn get_area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
