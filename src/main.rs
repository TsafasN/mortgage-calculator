use serde::{Serialize, Deserialize};
use serde_json::{to_string_pretty, from_str};

// #[serde(rename_all="PascalCase")]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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

// #[serde(rename_all="PascalCase")]
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
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
    serialize_test();
    deserialize_raw_string();
}

fn serialize_test() {
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

    let payment_calculator_serialized = to_string_pretty(&payment_calculator);

    if payment_calculator_serialized.is_ok() {
        println!("{}", payment_calculator_serialized.ok().unwrap());
    } else {
        println!("{:#?}", payment_calculator_serialized.err());
    }

    // for payment in payment_calculator.payments {
    //     println!("{:?}\n", payment);
    // }
}

fn deserialize_raw_string() {
    let raw_string = r#"
    {
    "asking_price": 165000.0,
    "down_payment": 20000.0,
    "years_duration": 30,
    "installment": 717.5649,
    "rate": 4.3,
    "principal": 145000.0,
    "payments": [
        {
        "year": 0,
        "start_balance": 145000.0,
        "installment_total": 0.0,
        "interest": 0.0,
        "principal": 0.0,
        "end_balance": 145000.0
        },
        {
        "year": 1,
        "start_balance": 145000.0,
        "installment_total": 8610.778,
        "interest": 6235.0,
        "principal": 2375.7783,
        "end_balance": 142624.22
        },
        {
        "year": 2,
        "start_balance": 142624.22,
        "installment_total": 8610.778,
        "interest": 6132.842,
        "principal": 2477.9365,
        "end_balance": 140146.28
        },
        {
        "year": 3,
        "start_balance": 140146.28,
        "installment_total": 8610.778,
        "interest": 6026.2905,
        "principal": 2584.4878,
        "end_balance": 137561.8
        },
        {
        "year": 4,
        "start_balance": 137561.8,
        "installment_total": 8610.778,
        "interest": 5915.1577,
        "principal": 2695.6206,
        "end_balance": 134866.17
        },
        {
        "year": 5,
        "start_balance": 134866.17,
        "installment_total": 8610.778,
        "interest": 5799.2456,
        "principal": 2811.5327,
        "end_balance": 132054.64
        },
        {
        "year": 6,
        "start_balance": 132054.64,
        "installment_total": 8610.778,
        "interest": 5678.35,
        "principal": 2932.4282,
        "end_balance": 129122.21
        },
        {
        "year": 7,
        "start_balance": 129122.21,
        "installment_total": 8610.778,
        "interest": 5552.256,
        "principal": 3058.5225,
        "end_balance": 126063.69
        },
        {
        "year": 8,
        "start_balance": 126063.69,
        "installment_total": 8610.778,
        "interest": 5420.739,
        "principal": 3190.0396,
        "end_balance": 122873.65
        },
        {
        "year": 9,
        "start_balance": 122873.65,
        "installment_total": 8610.778,
        "interest": 5283.567,
        "principal": 3327.2114,
        "end_balance": 119546.44
        },
        {
        "year": 10,
        "start_balance": 119546.44,
        "installment_total": 8610.778,
        "interest": 5140.497,
        "principal": 3470.2812,
        "end_balance": 116076.16
        },
        {
        "year": 11,
        "start_balance": 116076.16,
        "installment_total": 8610.778,
        "interest": 4991.275,
        "principal": 3619.5034,
        "end_balance": 112456.65
        },
        {
        "year": 12,
        "start_balance": 112456.65,
        "installment_total": 8610.778,
        "interest": 4835.636,
        "principal": 3775.142,
        "end_balance": 108681.5
        },
        {
        "year": 13,
        "start_balance": 108681.5,
        "installment_total": 8610.778,
        "interest": 4673.3047,
        "principal": 3937.4736,
        "end_balance": 104744.02
        },
        {
        "year": 14,
        "start_balance": 104744.02,
        "installment_total": 8610.778,
        "interest": 4503.993,
        "principal": 4106.785,
        "end_balance": 100637.234
        },
        {
        "year": 15,
        "start_balance": 100637.234,
        "installment_total": 8610.778,
        "interest": 4327.4014,
        "principal": 4283.377,
        "end_balance": 96353.85
        },
        {
        "year": 16,
        "start_balance": 96353.85,
        "installment_total": 8610.778,
        "interest": 4143.216,
        "principal": 4467.5625,
        "end_balance": 91886.29
        },
        {
        "year": 17,
        "start_balance": 91886.29,
        "installment_total": 8610.778,
        "interest": 3951.1106,
        "principal": 4659.668,
        "end_balance": 87226.62
        },
        {
        "year": 18,
        "start_balance": 87226.62,
        "installment_total": 8610.778,
        "interest": 3750.7446,
        "principal": 4860.0337,
        "end_balance": 82366.58
        },
        {
        "year": 19,
        "start_balance": 82366.58,
        "installment_total": 8610.778,
        "interest": 3541.7632,
        "principal": 5069.015,
        "end_balance": 77297.56
        },
        {
        "year": 20,
        "start_balance": 77297.56,
        "installment_total": 8610.778,
        "interest": 3323.7954,
        "principal": 5286.983,
        "end_balance": 72010.58
        },
        {
        "year": 21,
        "start_balance": 72010.58,
        "installment_total": 8610.778,
        "interest": 3096.455,
        "principal": 5514.323,
        "end_balance": 66496.26
        },
        {
        "year": 22,
        "start_balance": 66496.26,
        "installment_total": 8610.778,
        "interest": 2859.339,
        "principal": 5751.4395,
        "end_balance": 60744.82
        },
        {
        "year": 23,
        "start_balance": 60744.82,
        "installment_total": 8610.778,
        "interest": 2612.0273,
        "principal": 5998.751,
        "end_balance": 54746.07
        },
        {
        "year": 24,
        "start_balance": 54746.07,
        "installment_total": 8610.778,
        "interest": 2354.081,
        "principal": 6256.6973,
        "end_balance": 48489.375
        },
        {
        "year": 25,
        "start_balance": 48489.375,
        "installment_total": 8610.778,
        "interest": 2085.0432,
        "principal": 6525.7354,
        "end_balance": 41963.64
        },
        {
        "year": 26,
        "start_balance": 41963.64,
        "installment_total": 8610.778,
        "interest": 1804.4365,
        "principal": 6806.342,
        "end_balance": 35157.3
        },
        {
        "year": 27,
        "start_balance": 35157.3,
        "installment_total": 8610.778,
        "interest": 1511.764,
        "principal": 7099.014,
        "end_balance": 28058.287
        },
        {
        "year": 28,
        "start_balance": 28058.287,
        "installment_total": 8610.778,
        "interest": 1206.5063,
        "principal": 7404.272,
        "end_balance": 20654.014
        },
        {
        "year": 29,
        "start_balance": 20654.014,
        "installment_total": 8610.778,
        "interest": 888.1227,
        "principal": 7722.656,
        "end_balance": 12931.358
        },
        {
        "year": 30,
        "start_balance": 12931.358,
        "installment_total": 8610.778,
        "interest": 556.04846,
        "principal": 8054.73,
        "end_balance": 4876.6284
        }
    ]
    }"#;

    let payment_calculator_deserialized = from_str::<Calculator>(raw_string);

    if payment_calculator_deserialized.is_ok() {
        println!("{:#?}", payment_calculator_deserialized.ok().unwrap());
    } else {
        println!("{:#?}", payment_calculator_deserialized.err());
    }

}
