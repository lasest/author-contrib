use dioxus::prelude::*;

#[server]
pub async fn analytics_author_added() -> Result<(), ServerFnError> {
    info!("ANALYTICS: Author add button clicked");
    Ok(())
}
