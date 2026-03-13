fn rounder(mut num: f64) -> f64 {
    let epsilon = 1e-10;

    let rounded = num.round();

    if (num - rounded).abs() < epsilon {
        num = rounded;
    }

    if num.abs() < epsilon {
        num = 0.0;
    }

    num
}

fn row_reduction(matrix: &mut [[f64; 4]; 3]) -> [[f64; 4]; 3] {
    let n = matrix.len();
    
    for k in 0..n {

        let mut max_row = k;

        for i in k+1..n {
            if matrix[i][k].abs() > matrix[max_row][k].abs() {
                max_row = i
            }
        }

        if max_row != k {
            matrix.swap(k, max_row);
        }

        for i in k+1..n {
            let m = matrix[i][k] / matrix[k][k];

            for j in k..=n {
                matrix[i][j] -= m * matrix[k][j];
            }
        }

    }

    for i in 0..n {
        for j in 0..=n {
            matrix[i][j] = rounder(matrix[i][j])
        }
    }

    *matrix
}

fn solver(matrix: [[f64; 4]; 3]) -> Result<[f64; 3], &'static str> {

    let mut solutions = [0.0; 3];

    let n = matrix.len();

    let epsilon = 1e-10;

    for i in (0..n).rev() {
        let mut acc = matrix[i][n];

        for j in i+1..n {
            acc -= matrix[i][j] * solutions[j];
        }

        if matrix[i][i].abs() < epsilon {
            return Err("Sistema Singular: O sistema é Impossível (SI) ou Indeterminado (SPI)")
        }

        solutions[i] = acc / matrix[i][i];
    }

    for i in 0..n {
        solutions[i] = rounder(solutions[i])
    }

    Ok(solutions)
}

fn main() {    

    let mut matrix_si: [[f64; 4]; 3] = [
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 5.0],
        [1.0, -1.0, 2.0, 3.0]
    ];

    let new_matrix = row_reduction(&mut matrix_si);
    
    for line in new_matrix {
        println!("{:?}", line)
    }

    match solver(new_matrix) {
        Ok(solution) => println!("Soluções - {:?}", solution),
        Err(e) => println!("Falha ao resolver: {}", e)
    }

}

#[test]
fn reduction_on_simple_matrix() {
    let mut matrix = [
        [10.0, 12.0, 15.0, 960.0], 
        [6.0, 8.0, 12.0, 660.0], 
        [12.0, 12.0, 18.0, 1080.0]
    ];

    let answer = [
        [12.0, 12.0, 18.0, 1080.0],
        [0.0, 2.0, 3.0, 120.0],
        [0.0, 0.0, -3.0, -60.0]
    ];

    assert_eq!(row_reduction(&mut matrix), answer)
}

#[test]
fn solution_on_simple_matrix() {
    let mut matrix = [
        [1.0, 1.0, 1.0, 6.0],
        [-10.0, 2.0, -1.0, -9.0],
        [3.0, -2.0, 1.0, 2.0]
    ];

    let reduction = row_reduction(&mut matrix);
    let expected = [1.0, 2.0, 3.0];

    let solution = solver(reduction).unwrap();
    assert_eq!(solution, expected);

}

#[test]
fn impossible_system() {
    let mut matrix: [[f64; 4]; 3] = [
        [1.0, 1.0, 1.0, 1.0],
        [2.0, 2.0, 2.0, 5.0],
        [1.0, -1.0, 2.0, 3.0]
    ];

    let new_matrix = row_reduction(&mut matrix);

    assert!(solver(new_matrix).is_err())
}

#[test]
fn underteminated_system() {
    let mut matrix: [[f64; 4]; 3] = [
    [1.0, 1.0, 1.0, 2.0],
    [2.0, -1.0, 1.0, 3.0],
    [3.0, 0.0, 2.0, 5.0]
];

    let new_matrix = row_reduction(&mut matrix);

    assert!(solver(new_matrix).is_err())
}

