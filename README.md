# ⚡ Gauss Solver 3x3

[![Crates.io](https://img.shields.io/crates/v/gauss_solver_3x3.svg)](https://crates.io/crates/gauss_solver_3x3)
[![Documentation](https://docs.rs/gauss_solver_3x3/badge.svg)](https://docs.rs/gauss_solver_3x3)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Um solucionador determinístico de Sistemas de Equações Lineares (3x3) de alto desempenho escrito em Rust puro. 

Projetado para ambientes de computação de baixo nível, motores gráficos, simulações físicas e algoritmos base de inteligência artificial. Esta *crate* implementa a **Eliminação de Gauss com Pivoteamento Parcial**, oferecendo segurança de memória estrita e mitigação de erros de ponto flutuante.

## 🧠 Características Principais

* **Zero Heap Allocation:** Matrizes representadas inteiramente como *arrays* bidimensionais nativos (`[[f64; 4]; 3]`) na *stack*. Maximiza a localidade de cache (*cache locality*) e garante latência zero de alocação dinâmica.
* **Estabilidade Numérica (Pivoteamento Parcial):** O algoritmo identifica e reposiciona iterativamente o elemento de maior magnitude para a posição do pivô, minimizando erros catastróficos de truncamento em divisões.
* **Sanitização IEEE 754:** Implementa um filtro $\epsilon$ customizado ($10^{-10}$) pós-processamento, removendo ruídos de ponto flutuante e dízimas residuais geradas pela FPU.
* **Segurança contra Singularidades:** Rejeita Sistemas Impossíveis (SI) e Possíveis Indeterminados (SPI) de forma segura através de um `Result`, impedindo *panics* por divisão por zero.

## 📦 Instalação

Adicione a *crate* ao seu projeto executando:

```bash
cargo add gauss_solver_3x3
```

Ou adicione manualmente no seu Cargo.toml:

```Ini, TOML
[dependencies]
gauss_solver_3x3 = "0.1.1"
```
## 🚀 Como Usar
A API foi desenhada para ser limpa e direta. O fluxo consiste em passar a matriz bruta pela redução de linhas e, em seguida, extrair a solução com o solver.

```Rust
use gauss_solver_3x3::{row_reduction, solver, SolverError};

fn main() -> Result<(), SolverError> {
    // Representação do sistema linear:
    //  1x +  1y + 1z = 6
    // -10x + 2y - 1z = -9
    //  3x -  2y + 1z = 2
    
    let mut matrix = [
        [1.0, 1.0, 1.0, 6.0],
        [-10.0, 2.0, -1.0, -9.0],
        [3.0, -2.0, 1.0, 2.0]
    ];

    // 1. Aplica a eliminação de Gauss com pivoteamento (mutação in-place)
    let reduced_matrix = row_reduction(&mut matrix);
    
    // 2. Resolve o sistema triangular superior
    let solution = solver(reduced_matrix)?;

    println!("Solução exata: {:?}", solution); 
    // Saída: Solução exata: [1.0, 2.0, 3.0]

    Ok(())
}
```

## 🛡️ Tratamento de Erros
A função solver retorna um Result<[f64; 3], SolverError>. Caso a matriz fornecida não possua solução única, o erro SolverError::SingularSystem será retornado.

```Rust
use gauss_solver_3x3::{solver, SolverError};

// ... matriz escalonada de um sistema impossível ...
match solver(bad_matrix) {
    Ok(sol) => println!("Solução: {:?}", sol),
    Err(SolverError::SingularSystem) => {
        eprintln!("Falha: O sistema é impossível ou indeterminado.");
    }
}
```

## 🧪 Testes
A biblioteca possui uma suíte robusta de testes cobrindo o "caminho feliz" (SPD), casos extremos de pivoteamento parcial e detecção de matrizes singulares.

Para executar a suíte localmente:

```Bash
cargo test
```

## 📄 Licença
Distribuído sob a licença MIT. Veja LICENSE para mais informações.
