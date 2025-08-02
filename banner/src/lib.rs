use std::{ collections::HashMap, num::ParseFloatError };

pub struct Flag {
    pub short_hand: String,
    pub long_hand: String,
    pub desc: String,
}

impl Flag {
    pub fn opt_flag(name: &str, d: &str) -> Self {
        Flag {
            short_hand: name
                .chars()
                .nth(0)
                .map(|c| format!("-{}", c))
                .unwrap_or_default(),
            long_hand: format!("--{}", name),
            desc: d.to_string(),
        }

        // None("");
    }
}

//  associated type
pub type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

pub struct FlagsHandler {
    pub flags: HashMap<String, Callback>,
}

impl FlagsHandler {
    pub fn add_flag(&mut self, flag: Flag, func: Callback) {
        self.flags.insert(flag.short_hand, func);
        self.flags.insert(flag.long_hand, func);
    }

    pub fn exec_func(&self, input: &str, argv: &[&str]) -> Result<String, String> {
        if let Some(f) = self.flags.get(input) {
            if argv.len() < 2 {
                return Err("".to_string());
            } else {
                f(argv[0], argv[1]).map_err(|e| e.to_string())
            }
        } else {
            Err(format!(" {}", input))
        }
    }
}

pub fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.to_string().parse()?;

    if y == 0.0 {
        return Ok("inf".to_string());
    } else {
        let res = x / y;
        Ok(res.to_string())
    }
}

pub fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let x: f64 = a.parse()?;
    let y: f64 = b.to_string().parse()?;

    if y == 0.0 {
        return Ok("NaN".to_string());
    } else {
        let res = x % y;
        Ok(res.to_string())
    }
}
