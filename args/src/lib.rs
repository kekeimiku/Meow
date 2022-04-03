use core::{fmt::Display, str::FromStr};

#[derive(Debug)]
pub enum Error {
    NoneArgument,
    NonUtf8Argument,
    OptionWithoutAValue,
    OptionValueParsingFailed,
}

pub struct Args(Vec<String>);

impl Args {
    pub fn new() -> Result<Self, Error> {
        let mut args = Vec::new();
        for (i, arg) in std::env::args_os().enumerate() {
            if let Ok(s) = arg.into_string() {
                if i != 0 {
                    args.push(s);
                }
            } else {
                return Err(Error::NonUtf8Argument);
            }
        }

        if args.is_empty() {
            return Err(Error::NoneArgument);
        }

        Ok(Args(args))
    }

    pub fn contains<A: Into<Keys>>(&mut self, keys: A) -> bool {
        if let Some((idx, _)) = self.index_of(keys.into()) {
            self.0.remove(idx);
            return true;
        }

        false
    }

    pub fn init<A, T>(&mut self, keys: A) -> Result<Option<T>, Error>
    where
        A: Into<Keys>,
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        self.new_impl(keys.into(), |s| -> Result<T, String> {
            s.parse().map_err(|e: <T as FromStr>::Err| e.to_string())
        })
    }

    fn new_impl<T>(
        &mut self,
        keys: Keys,
        f: fn(&str) -> Result<T, String>,
    ) -> Result<Option<T>, Error> {
        if let Some((idx, _)) = self.index_of(keys) {
            let value = match self.0.get(idx + 1) {
                Some(v) => v,
                None => return Err(Error::OptionWithoutAValue),
            };

            match f(value) {
                Ok(value) => {
                    self.0.remove(idx);
                    self.0.remove(idx);
                    Ok(Some(value))
                }
                Err(_) => Err(Error::OptionValueParsingFailed),
            }
        } else {
            Ok(None)
        }
    }

    fn index_of<'a>(&self, keys: Keys) -> Option<(usize, &'a str)> {
        for key in &keys.0 {
            if !key.is_empty() {
                if let Some(i) = self.0.iter().position(|v| v == key) {
                    return Some((i, key));
                }
            }
        }

        None
    }
}

pub struct Keys([&'static str; 2]);

impl From<[&'static str; 2]> for Keys {
    fn from(v: [&'static str; 2]) -> Self {
        Keys(v)
    }
}

impl From<&'static str> for Keys {
    fn from(v: &'static str) -> Self {
        Keys([v, ""])
    }
}
