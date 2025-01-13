fn main() {}
#[cfg(test)]
mod tests {

    use std::rc::Rc;

    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }
    #[test]
    fn json_ser_der() {
        let point = Point { x: 1, y: 2 };
        let serialized = serde_json::to_string(&point).unwrap();
        let point_from_json: Point = serde_json::from_str(&serialized).unwrap();
        assert_eq!(point, point_from_json);
    }
    #[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
    struct PointWithRc {
        rcx: Rc<i32>,
        y: i32,
    }
    #[test]
    fn json_ser_der_rc() {
        let point = PointWithRc {
            rcx: Rc::new(1),
            y: 2,
        };
        let serialized = serde_json::to_string(&point).unwrap();
        println!("{serialized}");
        let point_from_json: PointWithRc = serde_json::from_str(&serialized).unwrap();
        assert_eq!(point, point_from_json);
    }
}
