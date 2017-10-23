//program that shows shadowing of a variable
fn main() {
    let x=5;		//declares immutable variable x
    let x=x*2;	        //shadows the variable by using same variable name and repeating the use of "let" keyword
    println!("x={}",x); 
}
