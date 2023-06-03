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
    use centered_interval_tree::CenTreeNode;
    use chrono::NaiveTime;

    #[test]
    fn tree() {
        let mut root: CenTreeNode<NaiveTime, String> = CenTreeNode::new();

        root.add(
            (
                NaiveTime::from_hms_opt(11, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(12, 0, 0).unwrap(),
            ),
            String::from("first"),
        );
        root.add(
            (
                NaiveTime::from_hms_opt(9, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(10, 0, 0).unwrap(),
            ),
            String::from("second"),
        );
        root.add(
            (
                NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(22, 0, 0).unwrap(),
            ),
            String::from("third"),
        );
        root.add(
            (
                NaiveTime::from_hms_opt(17, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(18, 0, 0).unwrap(),
            ),
            String::from("fourth"),
        );
        root.add(
            (
                NaiveTime::from_hms_opt(13, 0, 0).unwrap(),
                NaiveTime::from_hms_opt(14, 0, 0).unwrap(),
            ),
            String::from("fifth"),
        );

        assert_eq!(root.height(), 4);
    }
}
