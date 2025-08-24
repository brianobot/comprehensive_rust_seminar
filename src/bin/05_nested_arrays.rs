fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    // transpose of a matrix is a matrix where the rows and colums are switched from the original
    // item in row 1 column 1 go to row 1 column 1
    // item in row 1 column 2 go to row 2 column 1
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[i][j] = matrix[j][i]
        }
    }

    result
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    // dbg!(matrix);
    let transposed = transpose(matrix);
    dbg!(transposed);
}