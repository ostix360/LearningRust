#[derive(PartialEq, Debug)]
struct Chaussure {
    pointure: u32,
    style: String,
}

fn chaussures_a_la_pointure(chaussures: Vec<Chaussure>, pointure_chaussure: u32) -> Vec<Chaussure> {
    chaussures.into_iter()
              .filter(|s| s.pointure == pointure_chaussure)
              .collect()
}


#[cfg(test)]
mod test{
    use super::*;


    #[test]
    fn iterator(){
        let mut v = vec![1, 2, 3];

        let mut v_iter = v.iter_mut();

        assert_eq!(v_iter.next(), Some(&mut 1));
        assert_eq!(v_iter.next(), Some(&mut 2));
        assert_eq!(v_iter.next(), Some(&mut 3));
        assert_eq!(v_iter.next(), None);
    }

    #[test]
    fn iterator_sum(){
        let v = vec![1, 2, 3];

        let v_iter = v.iter();

        let total: u32 = v_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map(){
        let v = vec![1, 2, 3];

        let v2: Vec<_> = v.iter().map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);

    }

     #[test]
    fn filtres_par_pointure() {
        let chaussures = vec![
            Chaussure {
                pointure: 10,
                style: String::from("baskets"),
            },
            Chaussure {
                pointure: 13,
                style: String::from("sandale"),
            },
            Chaussure {
                pointure: 10,
                style: String::from("bottes"),
            },
        ];

        let a_ma_pointure = chaussures_a_la_pointure(chaussures, 10);

        assert_eq!(
            a_ma_pointure,
            vec![
                Chaussure {
                    pointure: 10,
                    style: String::from("baskets")
                },
                Chaussure {
                    pointure: 10,
                    style: String::from("bottes")
                },
            ]
        );
    }
}