use std::fs;
use std::path::Path;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use keyword_extraction::rake::{Rake, RakeParams};
use stop_words::{get, LANGUAGE};

const OLLAMA_URL: &str = "http://localhost:11434";
const MODEL: &str = "qwen2.5:3b-instruct";

#[derive(Serialize)]
struct OllamaRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub struct ClassificationResult {
    pub category: String,
    pub suggested_filename: String,
    pub used_llm: bool,
}

pub struct Organizer {
    magika: magika::Session,
    http: reqwest::blocking::Client,
    categories: Vec<String>,
    ollama_available: bool,
}

impl Organizer {
    pub fn new(categories: Vec<String>) -> Result<Self, magika::Error> {
        let magika = magika::Session::new()?;
        let http = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("failed to build http client");

        let ollama_available = Self::check_ollama(&http);

        Ok(Self { magika, http, categories, ollama_available })
    }

    fn check_ollama(http: &reqwest::blocking::Client) -> bool {
        http.get(format!("{}/api/tags", OLLAMA_URL))
            .timeout(Duration::from_secs(2))
            .send()
            .map(|res| res.status().is_success())
            .unwrap_or(false)
    }

    pub fn detect_type(&mut self, path: &Path) -> Result<String, magika::Error> {
        let result = self.magika.identify_file_sync(path)?;
        Ok(result.info().label.to_string())
    }

    fn extract_text(&self, path: &Path) -> String {
        match path.extension().and_then(|e| e.to_str()) {
            Some("pdf") => pdf_extract::extract_text(path).unwrap_or_default(),
            Some("txt") => fs::read_to_string(path).unwrap_or_default(),
            _ => String::new(),
        }
    }

    pub fn classify(&self, path: &Path) -> ClassificationResult {
        let text = self.extract_text(path);

        if text.trim().is_empty() {
            return ClassificationResult {
                category: "uncategorized".to_string(),
                suggested_filename: path.file_stem()
                    .map(|s| s.to_string_lossy().to_string())
                    .unwrap_or_else(|| "untitled".to_string()),
                used_llm: false,
            };
        }

        if self.ollama_available {
            if let Some(result) = self.classify_with_llm(&text) {
                return result;
            }
        }

        self.classify_with_rake(&text)
    }

    fn classify_with_llm(&self, text: &str) -> Option<ClassificationResult> {
        let prompt = format!(
            "Classify this document into exactly one category: {}.\n\
            Then suggest a short filename (3-5 words, lowercase, underscores instead of spaces, no extension).\n\
            Respond in exactly this format:\n\
            category: <category>\n\
            filename: <filename>\n\n\
            Document text:\n{}",
            self.categories.join(", "),
            &text.chars().take(800).collect::<String>()
        );

        let req = OllamaRequest {
            model: MODEL.to_string(),
            prompt,
            stream: false,
        };

        let res = self.http
            .post(format!("{}/api/generate", OLLAMA_URL))
            .json(&req)
            .send()
            .ok()?
            .json::<OllamaResponse>()
            .ok()?;

        let response = res.response.trim();

        let category = response.lines()
            .find(|l| l.starts_with("category:"))
            .map(|l| l.trim_start_matches("category:").trim().to_string())?;

        let filename = response.lines()
            .find(|l| l.starts_with("filename:"))
            .map(|l| l.trim_start_matches("filename:").trim().to_string())
            .unwrap_or_else(|| "untitled".to_string());

        Some(ClassificationResult { category, suggested_filename: filename, used_llm: true })
    }

    fn classify_with_rake(&self, text: &str) -> ClassificationResult {
        let stop_words = get(LANGUAGE::English);
        let rake = Rake::new(RakeParams::WithDefaults(text, &stop_words));
        let phrases: Vec<String> = rake.get_ranked_phrases(2);

        let category = "uncategorized".to_string();
        let filename = if phrases.is_empty() {
            format!("{}_untitled", category)
        } else {
            let joined = phrases.join("_")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join("_")
                .to_lowercase();
            format!("{}_{}", category, joined)
        };

        ClassificationResult { category, suggested_filename: filename, used_llm: false }
    }

    pub fn move_file(&self, src: &Path, dest_dir: &Path, new_stem: Option<&str>) -> std::io::Result<()> {
        fs::create_dir_all(dest_dir)?;

        let ext = src.extension().map(|e| e.to_string_lossy().to_string()).unwrap_or_default();
        let base_name = new_stem
            .map(|s| s.to_string())
            .unwrap_or_else(|| src.file_stem().unwrap().to_string_lossy().to_string());

        let file_name = if ext.is_empty() { base_name.clone() } else { format!("{}.{}", base_name, ext) };
        let mut dest = dest_dir.join(&file_name);

        let mut counter = 1;
        while dest.exists() {
            let candidate = if ext.is_empty() {
                format!("{}_{}", base_name, counter)
            } else {
                format!("{}_{}.{}", base_name, counter, ext)
            };
            dest = dest_dir.join(candidate);
            counter += 1;
        }

        fs::rename(src, dest)?;
        Ok(())
    }
}