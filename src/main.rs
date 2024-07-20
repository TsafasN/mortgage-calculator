#[derive(Debug, Clone, PartialEq)]
struct YearBalance {
    _start_balance:  f32,
    _end_balance:    f32,
    _interest:       f32,
    _principal:      f32,
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Calculator {
    asking_price:   f32,
    down_payment:   f32,
    years_duration: u16,
    installment:    f32,
    rate:           f32,
    principal:      f32,
    _payments:       Vec<YearBalance>,
}

impl Calculator {    
    pub fn new() -> Self {
        let mut calculator = Self::default();

        calculator.init_values();
        calculator.update();

        calculator
    }
    
    fn init_values(&mut self) {
        self.down_payment = 20000.0;
        self.asking_price = 165000.0;
        self.rate = 4.3;
        self.years_duration = 30;
    }

    fn update(&mut self) {
        self.principal = self.calculate_principal().unwrap();
        self.installment = self.calculate_installment().unwrap();
    }

    fn calculate_principal(&self) -> Result<f32, &str> {
        if self.asking_price >= self.down_payment {
            Ok(self.principal_equation())
        } else {
            Err("Invalid calculation")
        }
    }

    fn calculate_installment(&self) -> Result<f32,&str> {
        if self.rate > 0.0 {
            Ok(self.installment_equation())
        } else {
            Err("Invalid calculation")
        }
    }

    fn principal_equation(&self) -> f32 {
        self.asking_price - self.down_payment
    }
    
    fn installment_equation(&self) -> f32 {
        let rate_1: f32 = (self.rate / 12.0) / 100.0;
        let rate_2: f32 = (1.0 + rate_1).powi((self.years_duration * 12) as i32);
        self.principal / ((rate_2 - 1.0) / (rate_1 * rate_2))
    }
}

fn main() {
    
    let mut payment_calculator = Calculator::new();

    payment_calculator.update();

    assert_eq!( payment_calculator, 
                Calculator { 
                    asking_price: 165000.0, 
                    down_payment: 20000.0, 
                    years_duration: 30, 
                    installment: 717.5649, 
                    rate: 4.3, 
                    principal: 145000.0, 
                    _payments: [].to_vec() });

}
