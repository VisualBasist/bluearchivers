#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

struct Name {}

struct FullName {
    last_name: Name,
    first_name: Name,
}

struct BirthDay {
    month: u8,
    day: u8,
}

enum School {}
enum Club {}

struct Profile {
    school: School,
    club: Club,
    name: FullName,

    // 永遠の・・・?
    age: u32,

    birthday: BirthDay,
    height: f64,
    hobby: &'static str,
}

struct BasicInfo {}

pub struct Student {
    profile: Profile,
    basic_info: BasicInfo,
}
