pub fn cosine_similarity(vec1: &[f64], vec2: &[f64]) -> f64 {
    if vec1.len() != vec2.len() {
        return 0.0; 
    }
    let dot_product: f64 = vec1.iter().zip(vec2.iter()).map(|(x, y)| x * y).sum();
    let magnitude1: f64 = vec1.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    let magnitude2: f64 = vec2.iter().map(|x| x.powi(2)).sum::<f64>().sqrt();
    if magnitude1 == 0.0 || magnitude2 == 0.0 {
        return 0.0;
    }
    dot_product / (magnitude1 * magnitude2)
}