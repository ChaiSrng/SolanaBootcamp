fn main() {
    println!("Welcome to Fizz Buzz Program");
    let fbcount = fizzy();
    println!("Total fizz-buzz count is {}", fbcount);
}

fn fizzy()-> i32{
    let mut count = 0;
    let mut fbcount = 0;
    while count < 301 {
        
        if count % 3 == 0 && count % 5 == 0{
            println!("fizz buzz");
            fbcount +=1;
        }
        else if count % 3 == 0{
            println!("fizz");
        }
        else if count % 5 == 0{
            println!("buzz");
        }

        count += 1;
    }
    fbcount
}
