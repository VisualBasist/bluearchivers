#[derive(Debug, PartialEq, Eq)]
struct AP {
    // TODO 最大 999
    value: u32,
    // 1AP/6min
    last_recoverd_time: std::time::Instant,
}

#[derive(Debug, PartialEq, Eq)]
enum APError {
    // TODO: 足りない個数
    NotEnoughBluePyroxene,
    APUpperLimit,
}

impl std::fmt::Display for APError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            APError::NotEnoughBluePyroxene => write!(f, "青輝石が足りません"),
            // TODO: 本家のメッセージに合わせる
            APError::APUpperLimit => write!(f, "これ以上APを持てません"),
        }
    }
}
impl std::error::Error for APError {}

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
    pub fn buy_120(&mut self, blue_pyroxene: &mut BluePyroxene) -> Result<(), APError> {
        const AP_AMOUNT: u32 = 120;
        const AP_UPPER_LIMIT: u32 = 999;
        if self.value + AP_AMOUNT > AP_UPPER_LIMIT {
            return Err(APError::APUpperLimit);
        }
        const AP_PER_BLUEPYROXENE: u32 = 4;
        blue_pyroxene
            .consume(AP_AMOUNT / AP_PER_BLUEPYROXENE)
            .or(Err(APError::NotEnoughBluePyroxene))?;
        self.value += AP_AMOUNT;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Credit(u32);
#[derive(Debug, PartialEq, Eq)]
struct BluePyroxene(u32);
#[derive(Debug, PartialEq, Eq)]
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
    use crate::currency::{self, APError, NotEnoughBluePyroxene};

    #[test]
    fn buy_AP() {
        let mut ap = currency::AP::new(0, std::time::Instant::now());
        let mut blue_pyroxene = currency::BluePyroxene::new(30);
        assert_eq!(ap.buy_120(&mut blue_pyroxene), Ok(()));
        assert_eq!(ap.get(), 120);
        assert_eq!(blue_pyroxene.get(), 0);
    }

    #[test]
    fn buy_AP_with_notenough_blueproxene() {
        let mut ap = currency::AP::new(0, std::time::Instant::now());
        let mut blue_pyroxene = currency::BluePyroxene::new(29);
        assert_eq!(
            ap.buy_120(&mut blue_pyroxene),
            Err(APError::NotEnoughBluePyroxene)
        );
        assert_eq!(ap.get(), 0);
        assert_eq!(blue_pyroxene.get(), 29);
    }

    #[test]
    fn AP_exceeds_upper_limit() {
        let mut ap = currency::AP::new(880, std::time::Instant::now());
        let mut blue_pyroxene = currency::BluePyroxene::new(30);
        assert_eq!(ap.buy_120(&mut blue_pyroxene), Err(APError::APUpperLimit));
        assert_eq!(ap.get(), 880);
        assert_eq!(blue_pyroxene.get(), 30);
    }
}
