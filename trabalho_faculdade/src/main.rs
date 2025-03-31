fn mean(data: &Vec<f64>) -> f64 {
    let sum: f64 = data.iter().sum();
    sum / data.len() as f64
}

fn calculate_slope(x: &Vec<f64>, y: &Vec<f64>, x_mean: f64, y_mean: f64) -> f64 {
    let numerator: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (xi - x_mean) * (yi - y_mean))
        .sum();
    let denominator: f64 = x.iter()
        .map(|xi| (xi - x_mean).powi(2))
        .sum();
    numerator / denominator
}

fn calculate_intercept(x_mean: f64, y_mean: f64, b1: f64) -> f64 {
    y_mean - b1 * x_mean
}

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

fn calculate_mse(y: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
    let n = y.len() as f64;
    let error_sum: f64 = y.iter()
        .zip(y_pred.iter())
        .map(|(yi, y_pred)| (yi - y_pred).powi(2))
        .sum();
    error_sum / n
}

fn predict(x: &Vec<f64>, b0: f64, b1: f64) -> Vec<f64> {
    x.iter().map(|xi| b0 + b1 * xi).collect()
}

// Função principal para executar o código
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
    println!("\nPrevisões para os dados de entrada:");
    for (xi, yi_pred) in x.iter().zip(y_pred.iter()) {
        println!("x = {} -> y_pred = {}", xi, yi_pred);
    }

    let r_squared = calculate_r_squared(&y, &y_pred, mean(&y));
    let mse = calculate_mse(&y, &y_pred);

    println!("\nMétricas de avaliação:");
    println!("R²: {}", r_squared);
    println!("MSE: {}", mse);
}

