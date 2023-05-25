fn main() {
    println!("Welcome to Fizz Buzz Program");
    fizzy();
}

fn fizzy(){
    let mut count = 0;
    while count < 301 {
        
        if count % 3 == 0 && count % 5 == 0{
            println!("fizz buzz");}
        else if count % 3 == 0{
            println!("fizz");
        }
        else if count % 5 == 0{
            println!("buzz");
        }

        count += 1;
    }
}
