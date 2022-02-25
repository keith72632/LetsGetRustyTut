//----Generics

#[derive(Debug)]
struct Point<T>{
    x: T,
    y: T
}


// Using U to uncouple it from the struct
impl<U> Point<U> {
    fn get_x(&self) -> &U {
        &self.x
    }
}

// Hardcoded
struct PointFloat<f64> {
    x: f64,
    y:f64
}

impl PointFloat<f64> {
    fn get_x(&self) -> f64 {
        self.x
    } 
}

struct Mix<T, U> {
    x: T,
    y: U
}

// When implementing struct functions that also use generics, use different letters so it isn't constrained to the types of 
//the struct

impl <T, U> Mix<T, U> {
    fn mixup<V, W>(self, other: Mix<V, W>) -> Mix<T, W> {
        Mix {
            x: self.x, // takes from original "constructor"
            y: other.y // Set in this function
        }
    }
}

#[derive(Debug)]
enum Test{
    X(i32),
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let val = get_largest(nums);

    println!("Largest value {}", val);

    let point = Point {
        x: 77,
        y: 66
    };

    let point_float = PointFloat {
        x: 7.77,
        y: 6.66
    };

    println!("Point values {:?}, x = {}, y = {}", point, point.get_x(), point.y);
    println!("Point Float values x = {}, y = {}", point_float.get_x(), point_float.y);

    let mix1 = Mix {
        x: 8.88,
        y: 7
    };

    let mix2 = Mix {
        x: 9.99,
        y: 1
    };

    println!("Before mix up; x = {} y = {}", mix1.x, mix1.y);

    let new_mix = mix1.mixup(mix2);
    println!("New mix x = {} y = {}", new_mix.x, new_mix.y);

    let x = Test::X(500);
    println!("{:?}", x);
}

// PartialOrd and Copy assure that the values being compared, CAN be compared, aka primitive types
fn get_largest<T: PartialOrd + Copy>(numbers_list: Vec<T>) -> T {
    let mut largest = numbers_list[0];

    for i in numbers_list {
        if i > largest {
            largest = i;
        }
    }
    
    largest
}