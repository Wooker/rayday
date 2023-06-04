use std::ops::Bound;

use chrono::NaiveDate;

const REFORM_YEAR: u32 = 1099;
//const MONTHS: usize = 12;
//const WEEKDAYS: u32 = 7;

pub fn is_leap_year(year: u32) -> bool {
    if year <= REFORM_YEAR {
        return year % 4 == 0;
    }
    (year % 4 == 0) ^ (year % 100 == 0) ^ (year % 400 == 0)
}

/*
fn days_by_year(mut year: u32) -> u32 {
    let mut count: u32 = 0;

    while year > 1 {
        year -= 1;
        if is_leap_year(year) {
            count += 366
        } else {
            count += 365
        }
    }
    count
}
*/

pub fn get_days_from_month(year: i32, month: u32) -> i64 {
    NaiveDate::from_ymd(
        match month {
            12 => year + 1,
            _ => year,
        },
        match month {
            12 => 1,
            _ => month + 1,
        },
        1,
    )
    .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
    .num_days()
}

#[cfg(test)]
mod tests {

    use chrono::NaiveTime;
    use std::ops::Bound::*;
    use store_interval_tree::Interval;
    use store_interval_tree::IntervalTree;

    #[test]
    fn interval_tree() {
        // initialize an interval tree with end points of type usize
        let mut interval_tree = IntervalTree::<usize, ()>::new();

        // insert interval into the tree
        interval_tree.insert(Interval::new(Included(0), Excluded(3)), ());
        interval_tree.insert(Interval::new(Included(6), Included(10)), ());
        interval_tree.insert(Interval::new(Excluded(8), Included(9)), ());
        interval_tree.insert(Interval::new(Excluded(15), Excluded(23)), ());
        interval_tree.insert(Interval::new(Included(16), Excluded(21)), ());
        interval_tree.insert(Interval::new(Included(17), Excluded(19)), ());
        interval_tree.insert(Interval::new(Excluded(19), Included(20)), ());
        interval_tree.insert(Interval::new(Excluded(25), Included(30)), ());
        interval_tree.insert(Interval::new(Included(26), Included(26)), ());

        let interval1 = Interval::new(Excluded(23), Included(26));

        // interval (25, 30] is overlapped with interval (23,26]
        assert_eq!(
            interval_tree.find_overlap(&interval1).unwrap(),
            Interval::new(Excluded(25), Included(30))
        );

        // there is no interval in the tree that has interval with (10,15)
        assert!(interval_tree
            .find_overlap(&Interval::new(Excluded(10), Excluded(15)))
            .is_none());

        // find all overlaps with an interval
        let interval = Interval::new(Included(8), Included(26));
        // intervals are: (8,9], [6,10],(19,20], [16,21), (15,23), [17,19), (25,30], [26,26]
        let intervals = interval_tree.find_overlaps(&interval);

        // delete interval
        let interval = Interval::new(Included(15), Included(18));
        let overlapped_interval = interval_tree.find_overlap(&interval).unwrap();
        interval_tree.delete(&overlapped_interval);

        // find all intervals between two intervals/points
        let low = Interval::point(14);
        let high = Interval::point(24);
        // intervals are: (15,23), [16,21), [17,19), (19,20]
        let intervals = interval_tree.intervals_between(&low, &high);
    }

    #[test]
    fn interval_tree_time() {
        let mut interval_tree = IntervalTree::<NaiveTime, String>::new();

        interval_tree.insert(
            Interval::new(
                Included(NaiveTime::from_hms_opt(12, 0, 0).unwrap()),
                Excluded(NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
            ),
            String::from("1"),
        );
        assert_eq!(interval_tree.height(), 0);

        interval_tree.insert(
            Interval::new(
                Included(NaiveTime::from_hms_opt(13, 0, 0).unwrap()),
                Excluded(NaiveTime::from_hms_opt(14, 0, 0).unwrap()),
            ),
            String::from("1"),
        );
        assert_eq!(interval_tree.height(), 1);

        interval_tree.insert(
            Interval::new(
                Included(NaiveTime::from_hms_opt(12, 30, 0).unwrap()),
                Excluded(NaiveTime::from_hms_opt(13, 30, 0).unwrap()),
            ),
            String::from("1"),
        );
        assert_eq!(interval_tree.height(), 1);

        interval_tree.insert(
            Interval::new(
                Included(NaiveTime::from_hms_opt(12, 31, 0).unwrap()),
                Excluded(NaiveTime::from_hms_opt(13, 29, 0).unwrap()),
            ),
            String::from("1"),
        );

        let search = Interval::point(NaiveTime::from_hms_opt(12, 40, 0).unwrap());

        dbg!(&interval_tree.find_overlaps(&search));
        assert_eq!(interval_tree.height(), 2);
    }
}
