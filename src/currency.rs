struct AP {
    value: u32,
    // 1AP/6min
    last_recoverd_time: std::time::Instant,
}

impl AP {
    pub fn new(value: u32, last_recoverd_time: std::time::Instant) -> AP {
        AP {
            value,
            last_recoverd_time,
        }
    }
    pub fn get(&self) -> u32 {
        self.value
    }
    pub fn buy_120(
        &mut self,
        blue_pyroxene: &mut BluePyroxene,
    ) -> Result<(), NotEnoughBluePyroxene> {
        const AP_AMOUNT: u32 = 120;
        const AP_PER_BLUEPYROXENE: u32 = 4;
        blue_pyroxene.consume(AP_AMOUNT / AP_PER_BLUEPYROXENE)?;
        self.value += AP_AMOUNT;
        Ok(())
    }
}

struct Credit(u32);
struct BluePyroxene(u32);
#[derive(Debug)]
struct NotEnoughBluePyroxene;

impl BluePyroxene {
    fn new(value: u32) -> BluePyroxene {
        BluePyroxene(value)
    }
    fn get(&self) -> u32 {
        self.0
    }
    fn consume(&mut self, amount: u32) -> Result<(), NotEnoughBluePyroxene> {
        self.0 = self.0.checked_sub(amount).ok_or(NotEnoughBluePyroxene)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::currency;

    #[test]
    fn buy_AP() {
        let mut ap = currency::AP::new(0, std::time::Instant::now());
        let mut blue_pyroxene = currency::BluePyroxene::new(30);
        ap.buy_120(&mut blue_pyroxene).unwrap();
        assert_eq!(ap.get(), 120);
        assert_eq!(blue_pyroxene.get(), 0);
    }
}