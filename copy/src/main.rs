use copy::*;

fn main() {
   // let a = "1 2 4 5 6".to_owned();
   // let b = vec![1, 2, 4, 5];
    let c = 12;

    println!("{:?}", nbr_function(c));
    /* println!("{:?}", vec_function(b));
    println!("{:?}", str_function(a)); */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ownership_nbr_test() {
        assert_eq!(
            nbr_function(12),
            (12, 162754.79141900392, 2.4849066497880004)
        );
        assert_eq!(nbr_function(1), (1, 2.718281828459045, 0.0));
        assert_eq!(nbr_function(0), (0, 1.0, std::f64::NEG_INFINITY));
    }
 
    #[test]
    fn negative_nbr_test() {
        assert_eq!(
            nbr_function(-12),
            (-12, 0.00000614421235332821, 2.4849066497880004)
        );
        assert_eq!(nbr_function(-1), (-1, 0.36787944117144233, 0.0));
        assert_eq!(nbr_function(0), (0, 1.0, std::f64::NEG_INFINITY));
    } 

   #[test]
    fn ownership_vec_test() {
        assert_eq!(
            vec_function(vec![1, 2, 4]),
            (
                vec![1, 2, 4],
                vec![0.0, 0.6931471805599453, 1.3862943611198906]
            )
        );

        assert_eq!(
            vec_function(vec![0, 1]),
            (vec![0, 1], vec![std::f64::NEG_INFINITY, 0.0])
        );

        assert_eq!(
            vec_function(vec![1, 2, 4, 5]),
            (
                vec![1, 2, 4, 5],
                vec![
                    0.0,
                    0.6931471805599453,
                    1.3862943611198906,
                    1.6094379124341003
                ]
            )
        );
    }
 
    #[test]
    fn ownership_str_test() {
        assert_eq!(
            str_function(String::from("1 2 4")),
            (
                "1 2 4".to_string(),
                "2.718281828459045 7.38905609893065 54.598150033144236".to_string()
            )
        );
        assert_eq!(
            str_function(String::from("1 0")),
            (("1 0".to_string(), "2.718281828459045 1".to_string()))
        );
        assert_eq!(str_function(
			String::from("1 2 4 5 6")),
			(("1 2 4 5 6".to_string(), "2.718281828459045 7.38905609893065 54.598150033144236 148.4131591025766 403.4287934927351".to_string())));
    }
}