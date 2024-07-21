#[derive(Debug, Default, Clone, PartialEq)]
struct YearBalance {
    year:               u8,
    start_balance:      f32,
    installment_total:  f32,
    interest:           f32,
    principal:          f32,
    end_balance:        f32,
}

impl YearBalance {
    fn new(principal: f32) -> Self {
        Self { 
            installment_total:  Default::default(), 
            interest:           Default::default(), 
            principal:          Default::default(), 
            year:               Default::default(),
            end_balance:        principal, 
            start_balance:      principal,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Calculator {
    asking_price:   f32,
    down_payment:   f32,
    years_duration: u8,
    installment:    f32,
    rate:           f32,
    principal:      f32,
    payments:       Vec<YearBalance>,
}

impl Calculator {    
    pub fn new() -> Self {
        Self::default().init_values().to_owned()
    }

    pub fn update(&mut self) {
        self.principal = self.calculate_principal().unwrap();
        self.installment = self.calculate_installment().unwrap();
        self.calculate_payments();
    }

    pub fn calculate_principal(&self) -> Result<f32, &str> {
        if self.asking_price >= self.down_payment {
            Ok(self.principal_equation())
        } else {
            Err("Invalid calculation")
        }
    }

    pub fn calculate_installment(&self) -> Result<f32,&str> {
        if self.rate > 0.0 {
            Ok(self.installment_equation())
        } else {
            Err("Invalid calculation")
        }
    }

    pub fn calculate_payments(&mut self) {

        for year in 1..=self.years_duration {

            let installment_total = self.installment * 12.0;
            let start_balance = self.payments.last().unwrap().end_balance;
            let end_balance = self.payments.last().unwrap().end_balance - self.installment * 12.0 + (self.payments.last().unwrap().end_balance * self.rate) / 100.0;
            let interest = (self.payments.last().unwrap().end_balance * self.rate) / 100.0;
            let principal = self.installment * 12.0 - ((self.payments.last().unwrap().end_balance * self.rate) / 100.0);
            
            self.payments.push(YearBalance {installment_total, start_balance, end_balance, interest, principal, year});
        }
    }

    fn init_values(&mut self) -> &Self{
        self.down_payment = 20000.0;
        self.asking_price = 165000.0;
        self.rate = 4.3;
        self.years_duration = 30;

        self.principal = self.calculate_principal().unwrap();
        self.installment = self.calculate_installment().unwrap();
        self.payments.push(YearBalance::new(self.principal,));

        self
    }
    
    fn principal_equation(&self) -> f32 {
        self.asking_price - self.down_payment
    }
    
    fn installment_equation(&self) -> f32 {
        let rate_1: f32 = (self.rate / 12.0) / 100.0;
        let rate_2: f32 = (1.0 + rate_1).powi(((self.years_duration as u16) * 12) as i32);
        self.principal / ((rate_2 - 1.0) / (rate_1 * rate_2))
    }

}

fn main() {
    
    let mut payment_calculator = Calculator::new();

    assert_eq!( payment_calculator, 
                Calculator { 
                    asking_price: 165000.0, 
                    down_payment: 20000.0, 
                    years_duration: 30, 
                    installment: 717.5649, 
                    rate: 4.3, 
                    principal: 145000.0, 
                    payments: [YearBalance { 
                        year: 0, 
                        start_balance: 145000.0, 
                        installment_total: 0.0, 
                        interest: 0.0, 
                        principal: 0.0, 
                        end_balance: 145000.0 }].to_vec() });


    payment_calculator.update();

    for payment in payment_calculator.payments {
        println!("{:?}\n", payment);
    }
}
