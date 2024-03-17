//! # Module for retrieving the lastest auction results.
//!
//!
pub mod auctioned;

// Re-Export
pub use self::auctioned::Latest;

#[cfg(test)]
mod test {
    use chrono::{prelude::*, Months};

    use crate::tenor::Tenor;

    #[test]
    fn it_should_go_back_in_time() {
        let timerange = Tenor::parse("6m");
        let Ok(tenor) = timerange else {
           panic!("Could not parse range");
        };

        let now = Utc::now();
        let num = tenor.security();

        let at_that_time = now.checked_sub_months(Months::new(num));
        let diff = now.with_timezone(&Utc) - at_that_time.unwrap().with_timezone(&Utc);
        // let duration = Duration::num_days(6*30);
        println!("{:?}", diff.num_days());
    }
}