# Simpson's Methods
###### NumericalMethods, SimpsonsMethods, Rust

Before explaining what the Simpson's methods are used for and give an example, it is necessary to give the definition of numerical method.

A numerical method is a mathematical technique used for solving mathematical problems that cannot be solved or are difficult to solve analytically. To solve a problem analytically is to give an exact answer in the form of a mathematical expression.

In other words, a numerical method is an algorithm that converges to a solution that approximates to the exact answer.  This solution is called _numerical solution_.

So, let's start with the main topic. Simpson's methods are used for approximating the value of an integral I( _f_ ) of a function _f_(_x_) over an interval from _a_ to _b_ using quadratic (Simpson's 1/3 method) and cubic (Simpson's 3/8 method) polynomials. These methods are used when analytical integration is difficult or not possible, and when the integrand is given as a set of discrete points.

> Simpson's 1/3 method uses a quadratic polynomial to approximate the integrand. We need three points to determine the coefficients of this polynomial. These points are _x_<sub>1</sub> = _a_, _x_<sub>3</sub> = _b_ and _x_<sub>2</sub> = (_a_+_b_)/2

> ![1/3](https://thepracticaldev.s3.amazonaws.com/i/milyzy904t8vhkj8v0u3.png)
>> The name 1/3 in the method comes from the factor in the expression.

> If you want to a more accurate evaluation of the integral with this method, you can use the composite Simpson's 1/3 method in which you must divide the whole interval into _n_ subintervals using an even number, because Simpson's 1/3 method needs three points for defining a quadratic polynomial, that means that this method applies two adjacent subintervals at a time.

> ![1/3C](https://thepracticaldev.s3.amazonaws.com/i/4a0qsfv9wczvdvxl1g6e.png)
>> Where, the subintervals _n_ must be equally space. And _h_ = (_b_-_a_)/_n_

---

> Simpson's 3/8 method uses a cubic polynomial to approximate the integrand. We need four points to determine the coefficients of this polynomial. These points are _x_<sub>1</sub> = _a_, _x_<sub>2</sub> = _a_+_h_, _x_<sub>3</sub> = _a_+2 _h_ and _x_<sub>4</sub> = _b_

> ![3/8](https://thepracticaldev.s3.amazonaws.com/i/pin7tseoulymiru9eonm.png)
>> The name 3/8 in the method comes from the factor in the expression.

> If you want to a more accurate evaluation of the integral with this method, you can use the composite Simpson's 3/8 method in which you have to divide the whole interval into a number _n_ of subintervals that is divisible by 3, because Simpson's 3/8 method need four points for constructing a cubic polynomial, that mean that this method applies three adjacent subintervals at a time.

> ![3/8C](https://thepracticaldev.s3.amazonaws.com/i/b3rsjarh9ku9ekovbyxr.png)
>> Where, the subintervals _n_ must be equally space. And _h_ = (_b_-_a_)/_n_

These methods are applied in the real world to calculate areas, volumes, curve lengths and other problems related to integrals.

For example: the company ECO wants to drain and fill a polluted marsh (see the image below) that has a depth of 5 feet. The CEO of ECO wants to know how many cubic feet of land are needed to fill the area after draining the marsh.

![Marsh](https://thepracticaldev.s3.amazonaws.com/i/r89dpf5gxg0853j18syq.png)

To solve this problem I used the composite Simpson's 1/3 method.

```rust
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
```

Solution: To calculate the volume of the marsh, we must first estimate the surface area using the composite Simpson's 1/3 method.

```rust
let mut resultado: i32 = simpson(vec![146, 122, 76, 54, 40, 30, 13], 20);

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
```

And finally multiply by 5.

```rust
resultado = resultado * 5;
```

Obtaining that the approximate volume is `40 500 cubic feet`.

In conclusion, integration with numerical methods is a useful technique when we try to integrate a complicated function or if we only have tabulated data. With the Simpson's methods we can approximate a complex integral to the integral of a polynomial and obtain a solution that approximates the exact answer, even in some cases we can obtain the exact answer.

### References
* Gilat A., Subramaniam, V. (2013). Numerical methods for engineers and scientists. Wiley. Third Edition.
* Heath M., (2002). Scientific Computing: An Introductory Survet. McGraw Hill. Second Edition.
* Chapra, S., Canale, R. (2011). Métodos Numéricos para ingenieros. McGraw Hill. Sexta edición.

### Additional resources
* [Numerical Methods](https://github.com/LindaEliza/NumericalMethods) repository in GitHub, by LindaEliza.
