extern crate numeric_utils;

use numeric_utils::parse_f64;

fn test_f64(string: &str) {
    let tst_num = parse_f64(string);
    let ref_num = string.parse::<f64>();

    println!("{:?} {:?}", tst_num, ref_num);

    // Our internal representation of a float will almost certainly not
    // match the parse::<64> internal representation, and our
    // ParseFloatError isn't the same as core::num::ParseFloatError (and
    // we couldn't use that becuase it's kind field is private).
    //
    // So all around, it seemed easier to just compare the debug strings.
    assert_eq!(format!("{:?}", tst_num), format!("{:?}", ref_num));
}

#[test]
fn test_parse_f64() {
    let tests = vec!["1", "1.2", "12.34", "-1", "-1.2", "-12.34", "1E6", "1e6", "1e+6",
                     "1e-6", "", ".1", "-.1", "0.1", "-0.1", "+3", "12.34e3", "12.34e-3"];

    for test in tests {
        test_f64(test);
    }
}
