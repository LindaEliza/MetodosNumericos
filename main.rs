//Simpson's Methods 1/3
fn main() {
    println!("The approximate value of the integral is: {}", simpson(0.0, 0.8, 4));
}

fn simpson(a: f64, b: f64, n: i32) -> f64 {
    let mut y: f64 = funcion(a) + funcion(b);
    let mut x: f64 = a;
    let h: f64 = (b-a)/(n as f64);
    
    for i in 1..n {
      x = x + h;
      if i % 2 == 0{
        y= y + 2.0*funcion(x);
      }else{
        y= y + 4.0*funcion(x);
      }
    }
    
    return (b-a) * y / (3.0*(n as f64));
}

fn funcion(x: f64)-> f64 {
    return 0.2 + 25.0*x - 200.0*x*x+675.0*x*x*x;
}
