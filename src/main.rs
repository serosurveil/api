use serosurveil_api::telemetry::{get_subscriber, init_subscriber};
use serosurveil_api::{configuration::get_configuration, startup::Application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    let subscriber = get_subscriber(
        "serosurveil".into(),
        "info".into(),
        std::io::stdout,
        &configuration.application.honeycomb_url,
    );
    init_subscriber(subscriber);

    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;
    Ok(())
}
