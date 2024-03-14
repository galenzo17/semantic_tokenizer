use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use serde_json::json;
use anyhow::{Context, Result}; 
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModel;

/// Convierte un archivo .txt en un archivo .jsonl, donde cada línea del .txt se convierte en un objeto JSON.
fn txt_to_jsonl(txt_path: &str, jsonl_path: &str) -> std::io::Result<()> {
    let file = File::open(txt_path)?;
    let reader = BufReader::new(file);
    let mut jsonl_file = File::create(jsonl_path)?;

    for line in reader.lines() {
        let line = line?;
        // Crea un objeto JSON con la línea de texto
        let json_obj = json!({ "text": line });
        // Escribe el objeto JSON en el archivo .jsonl
        writeln!(jsonl_file, "{}", json_obj.to_string())?;
    }

    Ok(())
}

use std::collections::HashMap;

use qdrant_client::{
    prelude::*,
    qdrant::{VectorParams, VectorsConfig},
};
use rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv::dotenv().ok(); 

    let model_and_client = tokio::task::spawn_blocking(|| -> Result<(SentenceEmbeddingsModel, QdrantClient)> {
        let model = SentenceEmbeddingsBuilder::remote(
            rust_bert::pipelines::sentence_embeddings::SentenceEmbeddingsModelType::AllMiniLmL12V2,
        )
        .create_model()
        .context("Failed to create model")?; // Usa context para añadir contexto al error

        let qdrant_url = std::env::var("QDRANT_URL").context("QDRANT_URL environment variable not set")?;
        let qdrant_api_key = std::env::var("QDRANT_API_KEY").context("QDRANT_API_KEY environment variable not set")?;

        let client = QdrantClient::from_url(&qdrant_url)
            .with_api_key(qdrant_api_key)
            .build()
            .context("Failed to build QdrantClient")?; // Usa context aquí también

        Ok((model, client))
    }).await??; 
    let (model, client) = model_and_client;
    let mut args = std::env::args();
    match args.nth(1).as_deref() {
        Some("insert") => {
            let _ = client.delete_collection("points").await;
            client
                .create_collection(&CreateCollection {
                    collection_name: "points".into(),
                    vectors_config: Some(VectorsConfig {
                        config: Some(qdrant_client::qdrant::vectors_config::Config::Params(
                            VectorParams {
                                size: 384,
                                distance: Distance::Cosine as i32,
                                ..Default::default()
                            },
                        )),
                    }),
                    ..Default::default()
                })
                .await?;
            let Some(input_file) = args.next() else {
                eprintln!("usage: semantic insert <path/to/file>");
                return Ok(());
            };
            let contents = std::fs::read_to_string(input_file)?;
            let mut id = 0;
            let points = contents
                .lines()
                .map(|line| {
                    let payload: HashMap<String, Value> = serde_json::from_str(line).unwrap();
                    let text = payload.get("text").unwrap().to_string();
                    let embeddings = model
                        .encode(&[text])
                        .unwrap()
                        .into_iter()
                        .next()
                        .unwrap()
                        .into();
                    id += 1;
                    PointStruct {
                        id: Some(id.into()),
                        payload,
                        vectors: Some(embeddings),
                        ..Default::default()
                    }
                })
                .collect::<Vec<_>>();
            for batch in points.chunks(100) {
                let _ = client
                    .upsert_points("points", None,batch.to_owned(), None)
                    .await?;
                print!(".");
            }
            println!()
        }
        Some("find") => {
            let Some(text) = args.next() else {
                eprintln!("usage: semantic insert <path/to/file>");
                return Ok(());
            };
            let vector = model.encode(&[text])?.into_iter().next().unwrap();
            let result = client
                .search_points(&SearchPoints {
                    collection_name: "points".into(),
                    vector,
                    limit: 3,
                    with_payload: Some(true.into()),
                    ..Default::default()
                })
                .await?;
            println!("{result:#?}");
        }
        Some("convert") => {
            let txt_path = args.next().expect("Expected a path to the txt file");
            let jsonl_path = "output.jsonl"; // O permite al usuario especificarlo
            txt_to_jsonl(&txt_path, jsonl_path).expect("Failed to convert txt to jsonl");
            println!("Converted {} to {}", txt_path, jsonl_path);
        }
        _ => {
            eprintln!("usage: semantic insert <path/to/file>");
            return Ok(());
        }
    }
    
    Ok(())
}
