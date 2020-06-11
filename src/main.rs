mod methods;
mod closure;
mod higher_order_functions;


fn print_value(x:i32){
    println!("value={}",x);
}
fn increase(x: &mut i32){
    *x+=1;
}
fn functions(){
    print_value(32);
    let mut z=1;
    increase(&mut z);
    print_value(z);

}

fn main() {
    //functions();
    //methods::methods();
    //closure::closure();
    higher_order_functions::hof();

}
