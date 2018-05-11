//Simpson's Methods 1/3 to solve the marsh problem
fn main() {
    let mut resultado: i32 = simpson(vec![146, 122, 76, 54, 40, 30, 13], 20);
    println!("The approximate value of the integral is: {}", resultado);
    resultado = resultado * 5;
    println!("Approximately {} cubic feet of land is needed to fill the marsh.", resultado);
}

fn simpson(v: Vec<i32>, h: i32) -> i32 {
    let mut y: i32 = 0;
    let mut con: i32 = 0;
    let size: usize = v.len()-1;
    
    for i in v {
      if con == 0{
        y = y + i;
      }else if con == (size as i32) {
        y = y + i;
      }else{
        if con % 2 == 0{
          y= y + 2*i;
        }else{
          y= y + 4*i;
        }
      }
      con = con +1;
    }
    
    return h*y/3;
}
