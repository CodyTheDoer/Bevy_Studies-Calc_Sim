use bevy::prelude::*;

pub mod calculator;
pub mod cam_ui;
pub mod cam_world;
pub mod cam_calc_screen;
pub mod game_env;

pub fn sum_calc_operations(
    op: &mut ResMut<OpIndex>,
    var: &mut ResMut<SumVariable>,
    sum: &mut ResMut<SumCurrent>,
) {
    if let Some(call) = CalcOperations::from_index(op.index) {
        match call {
            CalcOperations::Init => {
                SumCurrent::update_sum(var, sum, op);
            },
            CalcOperations::Clear => {
                SumCurrent::zero(sum);
                var.clear();
            },
            CalcOperations::Add => {
                op.last_op = 1;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Subtract => {
                op.last_op = 2;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Multiply => {
                op.last_op = 3;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Divide => {
                op.last_op = 4;
                SumCurrent::var_to_sum_if_sum_zero(var, sum);
                var.clear();
            },
            CalcOperations::Sum => {
                SumCurrent::update_sum(var, sum, op);
            },
        }
    }
}

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
    pub entities: u32,
}

impl OpIndex {
    pub fn new(    
    ) -> Self {
        let (index, last_op, screen_color, entities): (u32, u32, u32, u32) = (0, 0, 0, 9); // Startup Entities cannot be captured so they're hardcoded.
        OpIndex {
            index,
            last_op,
            screen_color,
            entities,
        }
    }

    pub fn add_entity(&mut self) {
        self.entities += 1;
        info!("Entity Added: New total: {:?}", self.entities);
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
    }
}

#[derive(Clone, Copy, Debug, Resource)]
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

            let res_num: f64 = num.to_string().parse::<f64>().unwrap();
            let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
            let res = res_num * res_mul;

            sum.sum = res;

        } else if var.decimal_index == 0 && sum.sum == 0.0 {
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
        sum: &mut ResMut<SumCurrent>,
        op: &mut ResMut<OpIndex>,
    ) { // rebuild vec from SumVariable into f64 and pass it into the sum with maths if applicable
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

            let res_num: f64 = num.to_string().parse::<f64>().unwrap();
            let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
            let res = res_num * res_mul;

            sum.sum = res;

        } else if var.decimal_index == 0 && sum.sum == 0.0 {
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

        } else if var.decimal_index > 0 && sum.sum != 0.0 {
            let mut num: String = "".to_string();
            let mut multiplier: String = ".".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            for _ in 0..var.var.len() - var.decimal_index as usize - 1 {
                multiplier += "0";
            }
            multiplier += "1";

            let res_num: f64 = num.to_string().parse::<f64>().unwrap();
            let res_mul: f64 = multiplier.to_string().parse::<f64>().unwrap();
            let res = res_num * res_mul;

            match op.last_op {
                1 => {
                    sum.sum += res;
                },
                2 => {
                    sum.sum -= res;
                },
                3 => {
                    sum.sum *= res;
                },
                4 => {
                    sum.sum /= res;
                },
                _ => {}, // Handle invalid index
            }
            

        } else if var.decimal_index == 0 && sum.sum != 0.0 {
            let mut num: String = "".to_string();
            for i in 0..var.var.len() {
                num += &var.var[i].to_string();
            }
            let res: f64 = if var.var.len() == 0 {
                0.0
            } else {
                num.to_string().parse::<f64>().unwrap()
            };

            match op.last_op {
                1 => {
                    sum.sum += res;
                },
                2 => {
                    sum.sum -= res;
                },
                3 => {
                    sum.sum *= res;
                },
                4 => {
                    sum.sum /= res;
                },
                _ => {}, // Handle invalid index
            }        
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