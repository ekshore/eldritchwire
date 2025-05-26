mod commands;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedPointDecimal {
    raw_val: i16,
}

impl FixedPointDecimal {
    pub fn get_real_val(self) -> f32 {
        println!("raw_val: {}", self.raw_val);
        f32::from(self.raw_val) / 2_f32.powi(11)
    }

    pub fn get_rounded_val(self) -> f32 {
        (self.get_real_val() * 100.0).round() / 100.0
    }

    pub fn from_data(data: &[u8; 2]) -> Self {
        assert!(data.len() == 2);
        Self {
            raw_val: u16::from_le_bytes(*data) as i16,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operation {
    Assign,
    Increment,
    Toggle,
}
#[cfg(test)]
mod type_tests {
    use super::*;

    #[test]
    fn fpd_fifteen_percent() {
        let fifteen_percent = FixedPointDecimal { raw_val: 0x0133 };
        let float_value = fifteen_percent.get_rounded_val();
        assert_eq!(float_value, 0.15);
    }

    #[test]
    fn fpd_minus_point_three() {
        let minus_point_three = FixedPointDecimal {
            raw_val: 0xfd9au16 as i16,
        };
        let float_value = minus_point_three.get_rounded_val();
        assert_eq!(float_value, -0.3_f32);
    }
}
