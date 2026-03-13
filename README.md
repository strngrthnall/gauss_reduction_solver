# ⚡ Linear Equation Solver (Rust)

Um solucionador de Sistemas de Equações Lineares (3x3) de alto desempenho construído em Rust puro. O projeto implementa o algoritmo de **Eliminação de Gauss com Pivoteamento Parcial**, focado em segurança de memória, estabilidade numérica e alocação estrita na *stack*.

## 🧠 Visão Geral

Este repositório não é apenas um script de álgebra linear, mas uma prova de conceito de engenharia de software de baixo nível. A resolução de matrizes da forma $A \cdot \mathbf{x} = \mathbf{b}$ é a espinha dorsal de motores gráficos, simulações físicas e algoritmos base de Inteligência Artificial.

Foi desenhado para ser determinístico, rápido e imune a falhas clássicas da arquitetura de ponto flutuante (IEEE 754).

## 🛠️ Decisões de Engenharia e Arquitetura

* **Zero Heap Allocation:** Matrizes representadas como *arrays* bidimensionais nativos (`[[f64; 4]; 3]`) na *stack*. Isso garante total localidade de cache (*cache locality*) para a CPU, essencial para integração em sistemas operacionais ou *engines* rodando milhares de cálculos por segundo.
* **Estabilidade Numérica (Pivoteamento Parcial):** Mitiga o erro de propagação de truncamento. O algoritmo busca ativamente o elemento de maior magnitude absoluta na coluna atual e realiza um `.swap()` seguro em memória, garantindo que o multiplicador $m$ seja sempre minimizado, evitando divisões por valores próximos a zero.
* **Sanitização de Ponto Flutuante (Filtro $\epsilon$):** Uma rotina customizada de limpeza pós-processamento atua como um coletor de lixo matemático. Diferenças menores que `1e-10` entre floats residuais e inteiros absolutos são extirpadas, corrigindo o clássico acúmulo de dízimas da FPU.
* **Tratamento de Singularidades Seguro:** Rejeição explícita de Sistemas Impossíveis (SI) ou Possíveis Indeterminados (SPI). Em vez de causar *panic* ou vazar `NaN`/`inf` na memória, a função de retrosubstituição retorna de forma idiomática um `Result<[f64; 3], &'static str>`.

## 🎯 Casos de Uso Reais

A otimização implementada torna este módulo base ideal para cenários que exigem performance computacional:
- Modelos de Regressão Linear Múltipla (Mínimos Quadrados) para predição de dados ambientais, como níveis de reservatórios e previsão de chuvas.
- Cálculos de matrizes de transformação geométrica em ambientes gráficos e gerenciadores de janelas.
- Física de colisão em desenvolvimento de *engines* customizadas.

## 🚀 Como Executar

Clone o repositório (substitua pelo seu repositório real):
```bash
git clone [https://github.com/strngrthnall/gauss-solver-rs.git](https://github.com/strngrthnall/gauss-solver-rs.git)
cd gauss-solver-rs
cargo run
```

## Exemplo de Uso
```Rust
fn main() {
    // Sistema:
    //  1x +  1y + 1z = 6
    // -10x + 2y - 1z = -9
    //  3x -  2y + 1z = 2
    
    let mut matrix = [
        [1.0, 1.0, 1.0, 6.0],
        [-10.0, 2.0, -1.0, -9.0],
        [3.0, -2.0, 1.0, 2.0]
    ];

    let reduced_matrix = row_reduction(&mut matrix);
    
    match solver(reduced_matrix) {
        Ok(solution) => println!("Solução encontrada: {:?}", solution),
        Err(e) => eprintln!("Erro na resolução: {}", e)
    }
}
```

## 🧪 Suíte de Testes
O software é validado deterministicamente contra anomalias. Execute os testes com:

```Bash
cargo test
```

Os testes unitários garantem a cobertura contra:

Caminho Feliz: Sistemas Possíveis e Determinados (SPD).

Estresse de Pivoteamento: Sistemas onde o pivô natural é fraco em comparação aos elementos inferiores.

Falha Controlada: Injeção intencional de matrizes de Sistemas Impossíveis (SI) e Indeterminados (SPI) para garantir que o tipo Result::Err seja acionado com segurança através do assert!(...is_err()).

Desenvolvido com foco em fundamentos de Computação de Baixo Nível e Álgebra Linear.


---

### Por que esse README impressiona?
1. **O uso do jargão correto:** Palavras como *Zero Heap Allocation*, *Cache Locality*, e *IEEE 754* mostram que você não está apenas resolvendo a matemática, mas entende como o processador lida com ela sob o capô.
2. **Contexto arquitetural:** Mencionar o tratamento de `Result` mostra fluência na sintaxe e filosofia do Rust.
3. **Escalabilidade mental:** Ao citar que o algoritmo serve de base para regressões lineares em predição de reservatórios, você mostra ao recrutador que consegue enxergar o quadro completo e conectar um micro-componente (um array na *stack*) a um produto complexo e real (IA/Sistemas).

Suba este arquivo junto com o seu `.rs`. É o selo de qualidade que o seu esforço merece. O que achou da estrutura? Tem algum ponto específico que gostaria de enfatizar ainda mais?