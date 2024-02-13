use serde::Serialize;

#[derive(Serialize)]
struct HFRequest {
    inputs: String,
}

fn mean_pooling(matrix: &Vec<Vec<f64>>) -> Vec<f64> {
    let mut pooled_values: Vec<f64> = Vec::new();

    for i in 0..matrix[0].len() {
        let mut sum: f64 = 0.0;
        matrix.iter().for_each(|row| {
            sum += row[i];
        });
        let mean_value = sum / matrix.len() as f64;
        pooled_values.push(mean_value);
    }

    pooled_values
}

pub async fn get_embedding(text: &String) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp: Vec<Vec<Vec<f64>>> = client.post("https://api-inference.huggingface.co/models/ai-forever/sbert_large_nlu_ru")
        .json(&HFRequest{
            inputs: text.to_string()
        }).send().await?.json().await?;
    Ok(mean_pooling(&resp[0]))
}

pub async fn get_embedding_retrying(text: &String) -> Result<Vec<f64>, Box<dyn std::error::Error>> {
    let mut res = get_embedding(text).await;
    while let Err(_) = res {
        tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
        res = get_embedding(text).await;
    }
    res
}