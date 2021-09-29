#[path = "../src/sorts.rs"]
mod sorts;

#[cfg(test)]
mod tests {
    fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
        let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
        matching == a.len() && matching == b.len()
    }

    #[test]
    fn bubble_sort_test1() {
        let mut testing   = vec![4, 3, 2, 1];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::bubble_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn bubble_sort_test2() {
        let mut testing   = vec![1, 2, 3, 4];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::bubble_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn bubble_sort_test3() {
        let mut testing   = vec![1, 3, 2, 4];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::bubble_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn bubble_sort_test4() {
        let mut testing   = vec![-3, 200, -99, -774, 99, -265, 894, -886, -188, -667, -12, 130, 499, 52, 370, 404, 472, 93, -113, 67, -465, 516, 729, -202, -476, -317, 255, 684, -478, -584, 672, 465, 176, -835, -591, -558, -150, 414, 420, -156, -472, -853, -949, -712, 683, -718, -867, 774, -264, -108, -676, 376, -259, 505, 254, -373, 210, 91, 826, -662, 587, 130, 961, -380, -674, -844, 519, -743, -379, -503, -646, -397, 755, 685, 39, -349, -618, 420, 487, 719, -360, 974, -198, 162, 668, 287, -784, 499, -71, -56, 983, -122, -873, -369, -28, -117, -236, -362, 974, 687];

        let true_test = vec![-949, -886, -873, -867, -853, -844, -835, -784, -774, -743, -718, -712, -676, -674, -667, -662, -646, -618, -591, -584, -558, -503, -478, -476, -472, -465, -397, -380, -379, -373, -369, -362, -360, -349, -317, -265, -264, -259, -236, -202, -198, -188, -156, -150, -122, -117, -113, -108, -99, -71, -56, -28, -12, -3, 39, 52, 67, 91, 93, 99, 130, 130, 162, 176, 200, 210, 254, 255, 287, 370, 376, 404, 414, 420, 420, 465, 472, 487, 499, 499, 505, 516, 519, 587, 668, 672, 683, 684, 685, 687, 719, 729, 755, 774, 826, 894, 961, 974, 974, 983];

        crate::sorts::bubble_sort(& mut testing);

        for i in testing.iter() {
            print!("{}, ", *i);
        }

        println!();

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn insertion_sort_test1() {
        let mut testing   = vec![4, 3, 2, 1];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::insertion_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn insertion_sort_test2() {
        let mut testing   = vec![1, 2, 3, 4];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::insertion_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn insertion_sort_test3() {
        let mut testing   = vec![1, 3, 2, 4];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::insertion_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn selection_sort_test1() {
        let mut testing   = vec![4, 3, 2, 1];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::selection_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn selection_sort_test2() {
        let mut testing   = vec![1, 2, 3, 4];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::selection_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }

    #[test]
    fn selection_sort_test3() {
        let mut testing   = vec![1, 3, 2, 4];
        let true_test = vec![1, 2, 3, 4];

        crate::sorts::selection_sort(& mut testing);

        assert_eq!(true, do_vecs_match(&testing, &true_test));
    }
}
