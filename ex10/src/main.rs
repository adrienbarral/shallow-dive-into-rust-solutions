use std::ops::Add;

struct Complex {
    real: f64,
    imag: f64,
}

impl Default for Complex {
    fn default() -> Complex {
        Complex {
            real: 0.0,
            imag: 0.0,
        }
    }
}

impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}

fn integrate<T: IntoIterator>(addable: T) -> T::Item 
where T::Item: Add<Output = T::Item> + Default
{
    addable.into_iter().fold(T::Item::default(), |acc, x| acc + x)
}

fn main() {
    let complexes = [Complex { real: 1.0, imag: 2.0 }, Complex { real: 3.0, imag: 4.0 }];
    let sum = integrate(complexes);
    println!("Hello, world!");
}
