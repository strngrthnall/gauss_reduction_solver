#[derive(Debug)]
struct System {
    matrix: [[f64; 4]; 3],
}

fn gaussian_elimination(system: &mut System) -> [[f64; 4]; 3] {
    let matrix = &mut system.matrix;

    let n = matrix.len();
    
    for k in 0..n {

        if matrix[k][k] == 0.0 && k < n-2 {
            let buffer = matrix[k];

            matrix[k] = matrix[k+1];
            matrix[k+1] = buffer;
        }

        for i in k+1..n {
            let m = matrix[i][k] / matrix[k][k];

            for j in k..=n {
                matrix[i][j] -= m * matrix[k][j];
            }
        }

    }

    let new_matrix = matrix.clone();

    new_matrix

}

fn main() {    
    let mut system = System { 
        matrix: [
            [20.0, 40.0, 20.0, 1300.0], 
            [20.0, 10.0, 30.0, 1100.0], 
            [24.0, 24.0, 16.0, 1000.0]
        ]
    };

    let new_matrix = gaussian_elimination(&mut system);
    
    for line in new_matrix {
        println!("{:?}", line)
    }

}
