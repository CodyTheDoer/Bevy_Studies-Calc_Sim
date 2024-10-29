use bevy::prelude::*;

pub mod cam_ui;
pub mod cam_world;
pub mod game_env;

pub trait FlexInput {
    fn to_f64(self) -> f64;
}

impl FlexInput for f64 {
    fn to_f64(self) -> f64 {
        self
    }
}

impl FlexInput for usize {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl FlexInput for u32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl FlexInput for i32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
}

#[derive(Resource)]
pub struct OpIndex {
    pub index: u32,
}

impl OpIndex {
    pub fn new() -> Self {
        let index: u32 = 0;
        OpIndex {
            index,
        }
    }
}

#[derive(Debug)]
pub enum CalcOperations {
    Init,
    Clear,
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl CalcOperations {
    pub fn from_index(index: u32) -> Option<CalcOperations> {
        match index {
            0 => Some(CalcOperations::Init),
            1 => Some(CalcOperations::Clear),
            2 => Some(CalcOperations::Add),
            3 => Some(CalcOperations::Subtract),
            4 => Some(CalcOperations::Multiply),
            5 => Some(CalcOperations::Divide),
            _ => None, // Handle invalid index
        }
    }
}

pub fn sum_calc_operations(
    op: u32,
    var: &mut ResMut<SumVariable>,
) {
    if let Some(call) = CalcOperations::from_index(op) {
        match call {
            CalcOperations::Init => {
                info!("Init");
            },
            CalcOperations::Clear => {
                info!("Clear");
            },
            CalcOperations::Add => {
                info!("Add");
            },
            CalcOperations::Subtract => {
                info!("Subtract");
            },
            CalcOperations::Multiply => {
                info!("Multiply");
            },
            CalcOperations::Divide => {
                info!("Divide");
            },
            _ => {
                // Handle invalid button case, if needed
                info!("Invalid call, shouldn't even be possible @_@ What did you do?");
            },
        }
    }
}

#[derive(Resource)]
pub struct SumVariable {
    pub var: Vec<i32>,
    pub decimal_index: i32,
}

impl SumVariable {
    pub fn new() -> Self {
        let var: Vec<i32> = Vec::new();
        let decimal_index: i32 = 0;
        SumVariable {
            var,
            decimal_index,
        }
    }

    pub fn review(&self) {
        info!("var {:?}", self.var);
        info!("ind {:?}", self.decimal_index);
    }

    pub fn push(&mut self, input: i32) {
        self.var.push(input);
    }

    pub fn decimal(&mut self) {
        let len: i32 = self.var.len() as i32;
        self.decimal_index = len;
    }
}

#[derive(Debug, Resource)]
pub struct SumCurrent {
    pub sum: f64,
}

impl SumCurrent {
    pub fn new() -> Self {
        let sum: f64 = 0.0;
        SumCurrent{
            sum,
        }
    }

    // pub fn update_sum(
    //     mut var: ResMut<SumVariable>,
    // ) { // rebuild vec from SumVariable into f64 and pass it into the sum with maths if applicable
    //     let vvec = &var.var;
    //     let vindex = &var.decimal_index
    // }

    pub fn new_from<T: FlexInput>(input: T) -> Self {
        let sum: f64 = input.to_f64();
        SumCurrent{
            sum,
        }
    }

    pub fn zero(&self) -> Self {
        let sum: f64 = 0.0;
        SumCurrent{
            sum,
        }
    }

    pub fn add<T: FlexInput>(&mut self, input: T) -> Self {
        let sum: f64 = self.sum + input.to_f64();
        SumCurrent{
            sum,
        }
    }
    
    pub fn subtract<T: FlexInput>(&mut self, input: T) -> Self {
        let sum: f64 = self.sum - input.to_f64();
        SumCurrent{
            sum,
        }
    }
    
    pub fn multiply<T: FlexInput>(&mut self, input: T) -> Self {
        let sum: f64 = self.sum * input.to_f64();
        SumCurrent{
            sum,
        }
    }
    
    pub fn divide<T: FlexInput>(&mut self, input: T) -> Self {
        let value = input.to_f64();
        
        if value != 0.0 {
            let sum: f64 = self.sum / value;
            SumCurrent{
                sum,
            }
        } else {
            panic!("Division by zero is not allowed");
        }
    }
}

#[cfg(test)]
mod calc_backend_functionality {
    use super::*;

    #[test]
    fn check_sum() {
        let mut sum = SumCurrent::new_from(5.0);
        assert_eq!(sum.sum, 5.0);
    }

    #[test]
    fn check_add() {
        let mut sum = SumCurrent::new_from(5.0);
        assert_eq!(sum.sum, 5.0);

        let sum1 = sum.add(2);
        assert_eq!(sum1.sum, 7.0);

        let sum2 = sum.add(12.0);
        assert_eq!(sum2.sum, 17.0);

        let sum3 = sum.add(24.0 as usize);
        assert_eq!(sum3.sum, 29.0);
    }

    #[test]
    fn check_subtract() {
        let mut sum = SumCurrent::new_from(120.0);

        let sum1 = sum.subtract(2);
        assert_eq!(sum1.sum, 118.0);

        let sum2 = sum.subtract(12.0);
        assert_eq!(sum2.sum, 108.0);

        let sum3 = sum.subtract(24 as usize);
        assert_eq!(sum3.sum, 96.0);
    }

    #[test]
    fn check_multiply() {
        let mut sum = SumCurrent::new_from(120.0);
        
        let sum1 = sum.multiply(2);
        assert_eq!(sum1.sum, 240.0);
        
        let sum2 = sum.multiply(12.0);
        assert_eq!(sum2.sum, 1440.0);
        
        let sum3 = sum.multiply(24 as usize);
        assert_eq!(sum3.sum, 2880.0);
    }

    #[test]
    fn check_divide() {
        let mut sum = SumCurrent::new_from(120.0);
        
        let sum1 = sum.divide(2);
        assert_eq!(sum1.sum, 60.0);
        
        let sum2 = sum.divide(12.0);
        assert_eq!(sum2.sum, 10.0);
        
        let sum3 = sum.divide(24 as usize);
        assert_eq!(sum3.sum, 5.0);
    }
}