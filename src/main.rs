struct Calculator {
    num1: i32,
    num2: i32,
}

impl Calculator {
    fn set(&mut self, num1: i32, num2: i32) {
        self.num1 = num1;
        self.num2 = num2;
    }
    fn add(&self) {
        println!("Addition: {}", self.num1 + self.num2);
    }
    fn sub(&self) {
        println!("Subtraction: {}", self.num1 - self.num2);
    }
    fn mul(&self) {
        println!("Multiplication: {}", self.num1 * self.num2);
    }
    fn div(&self) {
        println!("Division: {}", self.num1 / self.num2);
    }
    fn rem(&self) {
        println!("Remainder: {}", self.num1 % self.num2);
    }
    fn pow(&self) {
        println!("Power: {}", self.num1.pow(self.num2 as u32));
    }
}

fn main() {
    println!("Hello, world!");
    let mut res: Calculator = Calculator { num1: 10, num2: 20 };
    println!("{} , {}", res.num1, res.num2);
    res.add();
    res.sub();
    res.set(10, 3);
    println!("new values are: {} , {}", res.num1, res.num2);
    res.add();
    res.sub();
    res.mul();
    res.div();
    res.rem();
    res.pow();
}