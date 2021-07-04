pub mod currency {
    struct AP {
        value: u32,
        // 1AP/6min
        last_recoverd_time: std::time::Instant,
    }

    impl AP {
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
    struct NotEnoughBluePyroxene;

    impl BluePyroxene {
        fn consume(&mut self, amount: u32) -> Result<(), NotEnoughBluePyroxene> {
            self.0 = self.0.checked_sub(amount).ok_or(NotEnoughBluePyroxene)?;
            Ok(())
        }
    }
}
