
fn main() {
    //this is rust comments
    /*
    this will help for 
    multi line commend.
     */
    // println!("Hello, world!");
    // println!("This is my first rust program!");
    print();
    variables();
    mutable();
   
}

fn print() {
    println!("Rust is fun,
    can't wait to learn");
    println!("this new line character can\nprint line in new lines.")
}

fn variables() {
    let x = 20;
    let y = 10;
    let z = 35;
    let name = "Sakthi";
    let age = 20;
    println!("This is {} and {}" , x , y);
    println!("we can declare like this {0} and {1}", x , y);
    println!("we can also print variable directly like this {x} and {y} finally {z}");
    println!("the name is {1} and {0}",name,age);
}

fn mutable() {
    let mut a = 20;
    println!("here is {a} value");

     a = 23;
    println!("here is after change the value {a} ");

    const MY: &str = "dev";
    println!("This is Constant Variable {MY}")
    
    
}