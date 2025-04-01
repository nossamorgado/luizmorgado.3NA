1- Explicação do Código
1️⃣ Cálculo da Média
A função mean calcula a média de um vetor de valores:

fn mean(data: &Vec<f64>) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

Exemplo: Para vec![1.0, 2.0, 3.0], a média será 2.0.

------------------------------------------------------------------------------------------------


2- Cálculo da Inclinação da Reta (b1)
A função calculate_slope calcula o coeficiente angular da regressão:

fn calculate_slope(x: &Vec<f64>, y: &Vec<f64>, x_mean: f64, y_mean: f64) -> f64 {
    let numerator: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum();
    let denominator: f64 = x.iter()
        .map(|xi| (xi - x_mean).powi(2))
        .sum();
    numerator / denominator
}

 Numerador: Soma das diferenças de x e y em relação à média multiplicadas.

 Denominador: Soma das diferenças de x em relação à média ao quadrado.

 Resultado: A inclinação da reta de regressão 
𝑏
1
b1
------------------------------------------------------------------------------------------------
3- Cálculo do Intercepto (b0)
A função calculate_intercept calcula o coeficiente linear da reta:

fn calculate_intercept(x_mean: f64, y_mean: f64, b1: f64) -> f64 {
    y_mean - b1 * x_mean
}

A fórmula é b0=y_mean−b1×x_mean.

------------------------------------------------------------------------------------------------
4- Previsão dos Valores 
𝑦
y
A função predict retorna os valores previstos:

fn predict(x: &Vec<f64>, b0: f64, b1: f64) -> Vec<f64> {
    x.iter().map(|xi| b0 + b1 * xi).collect()
}

Aplica a equação da reta: y_pred=b0+b1×x

------------------------------------------------------------------------------------------------
5- Cálculo do Coeficiente de Determinação R2
A função calculate_r_squared avalia a qualidade do modelo:

fn calculate_r_squared(y: &Vec<f64>, y_pred: &Vec<f64>, y_mean: f64) -> f64 {
    let total_sum_of_squares: f64 = y.iter()
        .map(|yi| (yi - y_mean).powi(2))
        .sum();
    let residual_sum_of_squares: f64 = y.iter()
        .zip(y_pred.iter())
        .map(|(yi, y_pred)| (yi - y_pred).powi(2))
        .sum();
    1.0 - (residual_sum_of_squares / total_sum_of_squares)
}

R2 mede o quanto a regressão explica a variabilidade dos dados.
 Quanto mais próximo de 1.0, melhor o modelo.

------------------------------------------------------------------------------------------------
6- Erro Quadrático Médio (MSE)
A função calculate_mse calcula o erro médio das previsões:

fn calculate_mse(y: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    let n = y.len() as f64;
    let error_sum: f64 = y.iter()
        .zip(y_pred.iter())
        .map(|(yi, y_pred)| (yi - y_pred).powi(2))
        .sum();
    error_sum / n
}

Mede o erro médio das previsões.

------------------------------------------------------------------------------------------------
7- Execução do Código (main)
A função main executa todo o fluxo:

fn main() {
    let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
    let y = vec![2.0, 3.0, 4.0, 5.0, 6.0];

    let x_mean = mean(&x);
    let y_mean = mean(&y);

    let b1 = calculate_slope(&x, &y, x_mean, y_mean);
    let b0 = calculate_intercept(x_mean, y_mean, b1);

    println!("Coeficientes da regressão linear:");
    println!("Intercepto (b0): {}", b0);
    println!("Inclinação (b1): {}", b1);

    let y_pred = predict(&x, b0, b1);
    
    println!("\nPrevisões:");
    for (xi, yi_pred) in x.iter().zip(y_pred.iter()) {
        println!("x = {} -> y_pred = {}", xi, yi_pred);
    }

    let r_squared = calculate_r_squared(&y, &y_pred, mean(&y));
    let mse = calculate_mse(&y, &y_pred);

    println!("\nMétricas:");
    println!("R²: {}", r_squared);
    println!("MSE: {}", mse);
}

O que acontece?

Define os valores de entrada (x e y).

Calcula média, coeficientes da reta e previsões.

Exibe os coeficientes 
𝑏
0
,
𝑏
1
b0,b1.

Gera previsões para os valores de x.

Calcula as métricas 
𝑅
2
R 
2
  e MSE para avaliar a regressão.

------------------------------------------------------------------------------------------------
Saída Esperada

Coeficientes da regressão linear:
Intercepto (b0): 1.0
Inclinação (b1): 1.0

Previsões:
x = 1.0 -> y_pred = 2.0
x = 2.0 -> y_pred = 3.0
x = 3.0 -> y_pred = 4.0
x = 4.0 -> y_pred = 5.0
x = 5.0 -> y_pred = 6.0

Métricas:
R²: 1.0
MSE: 0.0

 Como os pontos seguem uma reta perfeita, 
𝑅
2
=
1.0
R 
2
 =1.0 e MSE = 0.0.

------------------------------------------------------------------------------------------------
Resumo
Este código implementa Regressão Linear Simples: ✔ Calcula média, inclinação (b1) e intercepto (b0).
✔ Usa a equação da reta 
𝑦
=
𝑏
0
+
𝑏
1
×
𝑥
y=b0+b1×x para prever valores.
✔ Avalia o modelo com 
𝑅
2
R 
2
  e MSE.


