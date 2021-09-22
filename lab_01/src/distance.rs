pub fn levenstein_rec(s1: &str, s2: &str) -> usize {
    let s1_len = s1.len();
    let s2_len = s2.len();

    if s1_len == 0 {
        return s2_len;
    }

    if s2_len == 0 {
        return s1_len;
    }   

    if s1.chars().nth(0) == s2.chars().nth(0) {
        return levenstein_rec(&s1[1..], &s2[1..]);
    }

    let a = levenstein_rec(&s1[1..], &s2[1..]);
    let b = levenstein_rec(s1, &s2[1..]);
    let c = levenstein_rec(&s1[1..], &s2);

    return std::cmp::min(a, std::cmp::min(b, c)) + 1;
}

pub fn damerau_levenstein_rec(s1: &str, s2: &str) -> usize {
    let s1_len = s1.len();
    let s2_len = s2.len();

    if s1_len == 0 {
        return s2_len;
    }

    if s2_len == 0 {
        return s1_len;
    }   

    if s1.chars().nth(0) == s2.chars().nth(0) {
        return damerau_levenstein_rec(&s1[1..], &s2[1..]);
    }

    let a = damerau_levenstein_rec(&s1[1..], &s2[1..]);
    let b = damerau_levenstein_rec(s1, &s2[1..]);
    let c = damerau_levenstein_rec(&s1[1..], &s2);

    let mut d = usize::MAX;
    if s1_len > 1 && s2_len > 1 {
        d = damerau_levenstein_rec(&s1[2..], &s2[2..]);
    }

    return std::cmp::min(d, std::cmp::min(a, std::cmp::min(b, c))) + 1;
}

pub fn levenstein_iter(word1: &str, word2: &str) -> usize {
    // getting length of words
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();

    let word1_length = w1.len() + 1;
    let word2_length = w2.len() + 1;

    let mut matrix = vec![vec![0; word1_length]; word2_length];

    for i in 1..word1_length {
        matrix[0][i] = i;
    }
    for j in 1..word2_length {
        matrix[j][0] = j;
    }

    for j in 1..word2_length {
        for i in 1..word1_length {
            let x: usize = if w1[i - 1] == w2[j - 1] {
                matrix[j - 1][i - 1]
            } else {
                1 + std::cmp::min(
                    std::cmp::min(matrix[j][i - 1], matrix[j - 1][i]),
                    matrix[j - 1][i - 1],
                )
            };
            matrix[j][i] = x;
        }
    }
    return matrix[word2_length - 1][word1_length - 1];
}

pub fn damerau_levenstein_iter(word1: &str, word2: &str) -> usize {
    // getting length of words
    let w1 = word1.chars().collect::<Vec<_>>();
    let w2 = word2.chars().collect::<Vec<_>>();

    let word1_length = w1.len() + 1;
    let word2_length = w2.len() + 1;

    let mut matrix = vec![vec![0; word1_length]; word2_length];

    for i in 1..word1_length {
        matrix[0][i] = i;
    }
    for j in 1..word2_length {
        matrix[j][0] = j;
    }

    for j in 1..word2_length {
        for i in 1..word1_length {
            let x: usize = if w1[i - 1] == w2[j - 1] {
                matrix[j - 1][i - 1]
            } else {
                1 + std::cmp::min(
                    std::cmp::min(matrix[j][i - 1], matrix[j - 1][i]),
                    matrix[j - 1][i - 1],
                )
            };

            matrix[j][i] = x;

            if (j > 1) && (i > 1) && (w1[i - 1] == w2[j - 2]) && (w1[i - 2] == w2[j - 1]) {
                matrix[j][i] = std::cmp::min(x, matrix[j - 2][i - 2] + 1);
            }
        }
    }
    return matrix[word2_length - 1][word1_length - 1];
}
