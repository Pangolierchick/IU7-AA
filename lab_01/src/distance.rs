fn _levenstein_mem_rec(s1: &Vec<char>, s2: &Vec<char>, i: usize, j: usize, matrix: & mut Vec<Vec<usize>>) -> usize {
    if matrix[i][j] != usize::MAX {
        return matrix[i][j];
    }

    if i == 0 {
        matrix[i][j] = j;
        return matrix[i][j];
    }

    if j == 0 && i > 0 {
        matrix[i][j] = i;
        return matrix[i][j];
    }

    let mut eq = 1;

    if s1[i - 1] == s2[j - 1] {
        eq = 0;
    }

    matrix[i][j] = std::cmp::min(
        _levenstein_mem_rec(s1, s2, i, j - 1, matrix) + 1,
        std::cmp::min(
            _levenstein_mem_rec(s1, s2, i - 1, j, matrix) + 1,
            _levenstein_mem_rec(s1, s2, i - 1, j - 1, matrix) + eq
        )
    );

    return matrix[i][j];
}

pub fn levenstein_mem_rec(s1: &str, s2: &str) -> usize {
    let w1 = s1.chars().collect::<Vec<_>>();
    let w2 = s2.chars().collect::<Vec<_>>();

    let word1_length = w1.len();
    let word2_length = w2.len();

    let mut matrix = vec![vec![0; word1_length + 1]; word2_length + 1];

    for i in 0..word1_length + 1 {
        for j in 0..word2_length + 1 {
            matrix[i][j] = usize::MAX;
        }
    }

    _levenstein_mem_rec(&w1, &w2, word1_length, word2_length, &mut matrix);

    return matrix[word1_length][word2_length];
}

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
