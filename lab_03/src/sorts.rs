pub fn bubble_sort<T: std::cmp::Ord>(v: &mut [T]) {
    for i in 0..v.len() {
        for j in i..v.len() {
            if v[i] > v[j] {
                v.swap(i, j);
            }
        }
    }
}

pub fn insertion_sort<T: std::cmp::Ord>(v: &mut [T]) {
    for i in 1..v.len() {
        let mut j = i;
        while j > 0 && v[j] < v[j - 1] {
            v.swap(j, j - 1);
            j = j - 1;
        }
    }
}

pub fn selection_sort<T: std::cmp::Ord>(v: &mut [T]) {
    let mut min;

    for i in 0..v.len() {
        min = i;
        for j in (i + 1)..v.len() {
            if v[j] < v[min] {
                min = j;
            }
        }

        v.swap(i, min);
    }
}
