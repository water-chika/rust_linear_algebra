struct Vector<T, const SIZE : usize> {
    elements : [T; SIZE]
}

impl<T: Copy, const SIZE : usize> Copy for Vector<T, SIZE> {}

impl<T, const SIZE : usize> Clone for Vector<T, SIZE>
where T: Clone {
    fn clone(&self) -> Self {
        Self{elements : self.elements.clone()}
    }
}

impl<T, const SIZE : usize> std::ops::Index<usize> for Vector<T, SIZE> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[index]
    }
}
impl<T, const SIZE : usize> std::ops::IndexMut<usize> for Vector<T, SIZE> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output{
        &mut self.elements[index]
    }
}

impl<T: std::fmt::Display, const SIZE : usize> std::fmt::Display for Vector<T, SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        '('.fmt(f)?;
        for index in 0..(SIZE-1) {
            self.elements[index].fmt(f)?;
            ", ".fmt(f)?;
        }
        if SIZE > 0 {
            self.elements[SIZE-1].fmt(f)?;
        }
        ')'.fmt(f)?;
        Ok(())
    }
}

impl<T: std::ops::Add<Output = T>, const SIZE : usize> std::ops::Add for Vector<T, SIZE>
where for<'a> &'a T: std::ops::Add<Output = T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self{
            elements:
                std::array::from_fn(
                    |index| {
                        &self[index] + &rhs[index]
                    }
                )
        }
    }
}

fn main() {
    let a = Vector{ elements: [1.0, 0.5] };
    let b = Vector{elements: [0.5, 1.5]};
    println!("{a} + {b} = {}", a+b);
    println!("Hello, world!");
}
