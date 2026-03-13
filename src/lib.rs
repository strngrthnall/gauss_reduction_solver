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

/// Executa a Eliminação de Gauss com pivoteamento parcial em uma matriz aumentada 3x4.
///
/// Transforma o sistema linear em uma matriz triangular superior. O algoritmo 
/// procura iterativamente pelo elemento de maior magnitude na coluna atual e realiza 
/// o reordenamento das linhas (pivoteamento). Isso garante a máxima estabilidade 
/// numérica e minimiza erros de truncamento durante as divisões.
///
/// Ao final do processo, um filtro de epsilon ($\epsilon = 10^{-10}$) é aplicado 
/// iterativamente para sanitizar a matriz e remover dízimas residuais geradas 
/// pelas limitações da arquitetura IEEE 754 de ponto flutuante.
///
/// # Argumentos
///
/// * `matrix` - Uma referência mutável para a matriz aumentada `[[f64; 4]; 3]`. 
///   A matriz original será modificada *in-place* para evitar alocações na heap.
///
/// # Retorno
///
/// Retorna uma cópia da matriz escalonada (triangular superior) e perfeitamente 
/// sanitizada, pronta para a etapa de retrosubstituição.
///
/// # Exemplos
///
/// ```
/// // Substitua "gauss_solver_3x3" pelo nome que você definiu no Cargo.toml
/// use gauss_solver_3x3::row_reduction;
///
/// let mut matrix = [
///     [10.0, 12.0, 15.0, 960.0],
///     [6.0, 8.0, 12.0, 660.0],
///     [12.0, 12.0, 18.0, 1080.0]
/// ];
///
/// let reduced = row_reduction(&mut matrix);
///
/// // A terceira linha original foi puxada para o topo devido ao pivoteamento (|12.0| > |10.0|)
/// // e as linhas subsequentes foram perfeitamente zeradas (triangularização).
/// assert_eq!(reduced[0], [12.0, 12.0, 18.0, 1080.0]);
/// assert_eq!(reduced[1], [0.0, 2.0, 3.0, 120.0]);
/// assert_eq!(reduced[2], [0.0, 0.0, -3.0, -60.0]);
/// ```
pub fn row_reduction(matrix: &mut [[f64; 4]; 3]) -> [[f64; 4]; 3] {
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

#[derive(Debug, PartialEq)]
pub enum SolverError {
    SingularSystem
}
/// Resolve um sistema linear de 3 variáveis previamente escalonado.
///
/// Retorna um array `[x, y, z]` em caso de sucesso, ou `SolverError::SingularSystem`
/// caso o sistema seja impossível ou indeterminado.
///
/// # Exemplos
///
/// ```
/// // O nome da sua crate vai aqui (ex: gauss_solver)
/// use gauss_solver::{row_reduction, solver}; 
///
/// let mut matrix = [
///     [1.0, 1.0, 1.0, 6.0],
///     [-10.0, 2.0, -1.0, -9.0],
///     [3.0, -2.0, 1.0, 2.0]
/// ];
/// 
/// let reduced = row_reduction(&mut matrix);
/// let result = solver(reduced).unwrap();
/// 
/// assert_eq!(result, [1.0, 2.0, 3.0]);
/// ```
pub fn solver(matrix: [[f64; 4]; 3]) -> Result<[f64; 3], SolverError> {

    let mut solutions = [0.0; 3];

    let n = matrix.len();

    let epsilon = 1e-10;

    for i in (0..n).rev() {
        let mut acc = matrix[i][n];

        for j in i+1..n {
            acc -= matrix[i][j] * solutions[j];
        }

        if matrix[i][i].abs() < epsilon {
            return Err(SolverError::SingularSystem)
        }

        solutions[i] = acc / matrix[i][i];
    }

    for i in 0..n {
        solutions[i] = rounder(solutions[i])
    }

    Ok(solutions)
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

#[cfg(test)]
mod tests {
    #[test]
    fn solution_on_simple_matrix() {
        let mut matrix = [
            [1.0, 1.0, 1.0, 6.0],
            [-10.0, 2.0, -1.0, -9.0],
            [3.0, -2.0, 1.0, 2.0]
        ];

        let reduction = crate::row_reduction(&mut matrix);
        let expected = [1.0, 2.0, 3.0];

        let solution = crate::solver(reduction).unwrap();
        assert_eq!(solution, expected);

    }

    #[test]
    fn impossible_system() {
        let mut matrix: [[f64; 4]; 3] = [
            [1.0, 1.0, 1.0, 1.0],
            [2.0, 2.0, 2.0, 5.0],
            [1.0, -1.0, 2.0, 3.0]
        ];

        let new_matrix = crate::row_reduction(&mut matrix);

        assert!(crate::solver(new_matrix).is_err())
    }

    #[test]
    fn underteminated_system() {
        let mut matrix: [[f64; 4]; 3] = [
        [1.0, 1.0, 1.0, 2.0],
        [2.0, -1.0, 1.0, 3.0],
        [3.0, 0.0, 2.0, 5.0]
    ];

        let new_matrix = crate::row_reduction(&mut matrix);

        assert!(crate::solver(new_matrix).is_err())
    }
}

