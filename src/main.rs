#[derive(Debug)]
struct YearBalance {
    start_balance:  f32,
    end_balance:    f32,
    interest:       f32,
    principal:      f32,
}

#[derive(Debug)]
struct Calculator {
    asking_price:   f32,
    down_payment:   f32,
    years_duration: u16,
    installment:    f32,
    rate:           f32,
    principal:      f32,
    payments:       Vec<YearBalance>,
}

impl Calculator {
    pub fn new() -> Self {
        
        let mut calculator = Calculator::default();

        // TODO - Populate from gui values at initial rendering positions

        calculator.principal    = calculator.calculate_principal();
        calculator.installment  = calculator.calculate_installment();

        calculator
    }

    fn default() -> Self {
        Self { 
            asking_price:   Default::default(), 
            down_payment:   Default::default(), 
            years_duration: Default::default(), 
            installment:    Default::default(), 
            rate:           Default::default(), 
            principal:      Default::default(), 
            payments:       Vec::<YearBalance>::default(),
        }
    }

    fn calculate_principal(&self) -> f32 {
        self.asking_price - self.down_payment
    }

    fn calculate_installment(&self) -> f32 {
        let rate_1: f32 = (self.rate / 12.0) / 100.0;
        let rate_2: f32 = (1.0 + rate_1).powi((self.years_duration * 12) as i32);
        
        self.principal / ((rate_2 - 1.0) / (rate_1 * rate_2))
    }
}

fn main() {
    
    let mut payment_calculator = Calculator::new();

    println!("Default installment: {}", payment_calculator.installment);

    payment_calculator.down_payment = 20000.0;
    payment_calculator.asking_price = 165000.0;
    payment_calculator.rate = 4.3;
    payment_calculator.years_duration = 30;

    payment_calculator.principal    = payment_calculator.calculate_principal();
    payment_calculator.installment  = payment_calculator.calculate_installment();

    println!("Calculator: {:?}", payment_calculator);

    println!("Calculated installment: {}", payment_calculator.installment);

}
