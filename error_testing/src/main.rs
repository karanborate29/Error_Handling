use std::fmt::Error;

fn main() {
    println!("Practicing Error Handling and Testing");

    let contact = Detail {
        mob: 7770003000,
        name: String::from("Karan"),
    };
    println!("{:?}", contact.name);
}

struct Detail {
    mob: i64,
    name: String,
}

#[cfg(test)]
mod tests {
    use super::add_num;
    use super::is_even;
    #[test]
    fn add() {
        assert_eq!(3 + 2, 5)
    }
    #[test]
    fn add_fail() {
        assert_eq!(3 + 3, 5, "Not Equal")
    }
    #[test]
    fn add_with_nq() {
        assert_ne!(3 + 2, 5, "Should Fail If Equal")
    }
    #[test]
    fn try_with_panic() {
        panic!("In Panic..!")
    }
    #[test]
    fn lets_add() {
        let result: i32 = add_num(3, 2);
        assert_eq!(5, result);
        assert_eq!(6, result, "Got Failed");
    }
    #[test]
    fn even_odd() {
        let array: [i32; 5] = [1, 2, 3, 4, 5];
        for element in array.iter() {
            println!("{:?}", is_even(element)); //Handle Error if No is Odd Or Ok if No is Even
        }
    }
    ///parsing &str to i32 type and also using Option Type
    #[test]
    fn get_data() {
        let str: &str = "karan";
        match str.parse::<i32>() {
            //Error Handling with Parsing string to i32
            Ok(t) => println!("Hey {}", t),
            Err(e) => println!("Error: {}", e),
            /* Some("Karan") => println!("This is karan"),  //Using Option for Parsing String to i32
             Some(unknown) => println!("This is not Karan, This is {}",unknown),
             None => println!("This is unknown"),
            _ => {}*/
        }
    }
    ///Testing by passing Odd No. to Show Error
    #[test]
    fn even_odd_err() {
        println!("{:?}", is_even(&3));
    }
}
///Using add_num Function to use "use::super.*"
fn add_num(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

pub fn is_even(num: &i32) -> Result<&i32, Error> {
    if num % 2 == 0 {
        Ok(num)
    } else {
        Err(Error)
    }
}
