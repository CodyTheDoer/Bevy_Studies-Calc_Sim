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

#[derive(Clone, Resource)]
pub struct OpIndex {
    pub index: u32,
    pub last_op: u32,
    pub screen_color: u32,
}

impl OpIndex {
    pub fn new(    
        // mut op: ResMut<OpIndex>,
    ) -> Self {
        let (index, last_op, screen_color): (u32, u32, u32) = (0, 0, 0);
        OpIndex {
            index,
            last_op,
            screen_color,
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
    Sum,
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
            6 => Some(CalcOperations::Sum),
            _ => None, // Handle invalid index
        }
    }
}

pub fn sum_calc_operations(
    op: &mut ResMut<OpIndex>,
    var: &mut ResMut<SumVariable>,
    sum: &mut ResMut<SumCurrent>,
) {
    if let Some(call) = CalcOperations::from_index(op.index) {
        match call {
            CalcOperations::Init => {
                // info!("sum_calc_operations: Init");
                SumCurrent::update_sum(var, &call, sum, op);
                // op.index = 6;
            },
            CalcOperations::Clear => {
                // info!("sum_calc_operations: Clear");
                SumCurrent::zero(sum);
                var.clear();
            },
            CalcOperations::Add => {
                // info!("sum_calc_operations: Add");
                op.last_op = 1;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Subtract => {
                // info!("sum_calc_operations: Subtract");
                op.last_op = 2;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Multiply => {
                // info!("sum_calc_operations: Multiply");
                op.last_op = 3;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Divide => {
                // info!("sum_calc_operations: Divide");
                op.last_op = 4;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Sum => {
                // info!("sum_calc_operations: Sum");
                SumCurrent::update_sum(var, &call, sum, op);
            },
        }
    }
}

#[derive(Clone, Resource)]
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
        info!("Review: var.vec {:?}", self.var);
        info!("Review: var.index {:?}", self.decimal_index);
    }

    pub fn push(&mut self, input: i32) {
        self.var.push(input);
    }

    pub fn decimal(&mut self) {
        if self.decimal_index == 0 {
            let len: i32 = self.var.len() as i32;
            self.decimal_index = len;
        } else {
            info!("Triggered calc shake animation for duplicate decimals");
        }
    }

    pub fn clear(&mut self) {
        self.decimal_index = 0;
        while self.var.len() > 0 {
            self.var.pop();
        }
        // info!("clear: index: {:?}", self.decimal_index);
        // info!("clear: vec: {:?}", self.var);
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

    pub fn var_to_sum_if_sum_zero(
        var: &mut ResMut<SumVariable>,
        sum: &mut ResMut<SumCurrent>,
    ) {
        if var.decimal_index > 0 && sum.sum == 0.0 {
            let mut num: String = "".to_string();
            let mut multiplier: String = ".".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            for _ in 0..var.var.len() - var.decimal_index as usize - 1 {
                multiplier += "0";
            }
            multiplier += "1";
            // info!("num {:?}", num);
            // info!("multiplier {:?}", multiplier);

            let res_num: f64 = num.to_string().parse::<f64>().unwrap();
            let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
            let res = res_num * res_mul;

            sum.sum = res;
            // info!("update_sum: 1 Sum: {:?}", sum.sum);

        } else if var.decimal_index == 0 && sum.sum == 0.0 {
            // info!("update_sum: if 2");
            let mut num: String = "".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            let new_sum: f64 = if var.var.len() == 0 {
                0.0
            } else {
                num.to_string().parse::<f64>().unwrap()
            };

            sum.sum = new_sum;
        }
    }

    pub fn update_sum(
        var: &mut ResMut<SumVariable>,
        call: &CalcOperations,
        sum: &mut ResMut<SumCurrent>,
        op: &mut ResMut<OpIndex>,
    ) { // rebuild vec from SumVariable into f64 and pass it into the sum with maths if applicable
        // info!("update_sum: decimal_index: {:?}", var.decimal_index);

        if var.decimal_index > 0 && sum.sum == 0.0 {
            let mut num: String = "".to_string();
            let mut multiplier: String = ".".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            for _ in 0..var.var.len() - var.decimal_index as usize - 1 {
                multiplier += "0";
            }
            multiplier += "1";
            // info!("num {:?}", num);
            // info!("multiplier {:?}", multiplier);

            let res_num: f64 = num.to_string().parse::<f64>().unwrap();
            let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
            let res = res_num * res_mul;

            sum.sum = res;
            // info!("update_sum: 1 Sum: {:?}", sum.sum);

        } else if var.decimal_index == 0 && sum.sum == 0.0 {
            // info!("update_sum: if 2");
            let mut num: String = "".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            let new_sum: f64 = if var.var.len() == 0 {
                0.0
            } else {
                num.to_string().parse::<f64>().unwrap()
            };

            sum.sum = new_sum;
            
            // info!("update_sum: var.vec: {:?}", var.var);
            // info!("update_sum: 2 Sum: {:?}", sum.sum);

        } else if var.decimal_index > 0 && sum.sum != 0.0 {
            // info!("update_sum: if 3");
            let mut num: String = "".to_string();
            let mut multiplier: String = ".".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            for _ in 0..var.var.len() - var.decimal_index as usize - 1 {
                multiplier += "0";
            }
            multiplier += "1";
            // info!("num {:?}", num);
            // info!("multiplier {:?}", multiplier);

            let res_num: f64 = num.to_string().parse::<f64>().unwrap();
            let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
            let res = res_num * res_mul;

            match op.last_op {
                1 => {
                    // Add
                    sum.sum += res;
                },
                2 => {
                    // Subtract
                    sum.sum -= res;
                },
                3 => {
                    // Multiply
                    sum.sum *= res;
                },
                4 => {
                    // Divide
                    sum.sum /= res;
                },
                _ => {}, // Handle invalid index
            }
            
            // info!("update_sum: var.vec: {:?}", var.var);
            // info!("update_sum: 3 Sum: {:?}", sum.sum);

        } else if var.decimal_index == 0 && sum.sum != 0.0 {
            // info!("update_sum: if 4");
            let mut num: String = "".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            let res: f64 = if var.var.len() == 0 {
                0.0
            } else {
                num.to_string().parse::<f64>().unwrap()
            };

            // info!("op.index: {:?}", op.index);
            match op.last_op {
                1 => {
                    // info!("op.index: Add match");
                    // Add
                    sum.sum += res;
                },
                2 => {
                    // info!("op.index: Sub match");
                    // Subtract
                    sum.sum -= res;
                },
                3 => {
                    // info!("op.index: Mul match");
                    // Multiply
                    sum.sum *= res;
                },
                4 => {
                    // info!("op.index: Div match");
                    // Divide
                    sum.sum /= res;
                },
                _ => {}, // Handle invalid index
            }

            // info!("update_sum: var.vec: {:?}", var.var);
            // info!("update_sum: 4 Sum: {:?}", sum.sum);            
        } else {
            info!("update_sum: var.decimal_index: Failure, below 0 or invalid.")
        }
    }

    pub fn new_from<T: FlexInput>(input: T) -> Self {
        let sum: f64 = input.to_f64();
        SumCurrent{
            sum,
        }
    }

    pub fn zero(        
        sum: &mut ResMut<SumCurrent>,
    ) {
        let zero: f64 = 0.0;
        sum.sum = zero;
    }

    // pub fn zero(&self) -> Self {
    //     let sum: f64 = 0.0;
    //     SumCurrent{
    //         sum,
    //     }
    // }

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