pub async fn enviar_email_resend(para: &str, assunto: &str, html_body: &str) -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("RESEND_API_KEY").unwrap_or_else(|_| "dummy_api_key".to_string());
    
    // Se for dummy, apenas loga e não envia
    if api_key == "dummy_api_key" {
        println!("Dummy API KEY: fingindo enviar email para {} com assunto '{}'", para, assunto);
        return Ok(());
    }

    let client = reqwest::Client::new();
    
    let payload = serde_json::json!({
        "from": "onboarding@resend.dev",
        "to": [para],
        "subject": assunto,
        "html": html_body
    });

    let res = client.post("https://api.resend.com/emails")
        .bearer_auth(api_key)
        .json(&payload)
        .send()
        .await?;

    if !res.status().is_success() {
        let status = res.status();
        let text = res.text().await?;
        return Err(format!("Erro ao enviar email via Resend: {} - {}", status, text).into());
    }

    Ok(())
}
