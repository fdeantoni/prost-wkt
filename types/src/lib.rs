mod pbtime;
pub use crate::pbtime::*;

mod pbstruct;
pub use crate::pbstruct::*;

#[cfg(test)]
mod tests {
    use crate::*;
    use chrono::{DateTime, Utc, NaiveDateTime};
    use std::convert::TryInto;

    #[test]
    fn timestamp_test() {
        let ts = Timestamp::new(10, 10);
        let datetime_utc: DateTime<Utc> = ts.try_into().expect("conversion failed!");
        let datetime_naive: NaiveDateTime = ts.try_into().expect("conversion failed!");

        println!("{:?}", datetime_utc);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
