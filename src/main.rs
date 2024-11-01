use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
  name: String,
  
}

#[derive(Serialize, Deserialize)]
struct Article {
  article: String,
  author: String,
  paragraphs: Vec<Paragraph>
}

fn main() {
  let json = r#"
    {
      "article": "How to work with JSON in Rust",
      "author": "Sergey",
      "paragraphs": [
        {
          "name": "paragraph 1"
        },
        {
          "name": "paragraph 2"
        }
      ]
    }
  "#;
  let deserialized: Article = read_json_typed(&json);
  println!("The name of the first paragraph is : {}", deserialized.paragraphs[0].name);
}

fn read_json_typed(txt: &str) -> Article {
  let parsed: Article = serde_json::from_str(txt).unwrap();
  parsed
}