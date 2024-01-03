use lettre::AsyncTransport;

#[tracing::instrument(
    name = "Generic e-mail sending function.",
    skip(
        subject,
        html_content,
        text_content
    ),
    fields(
        recipient_email = %user.email,
        recipient_first_name = %user.first_name,
        recipient_last_name = %user.last_name
    )
)]
pub async fn send_email(
    user: crate::models::UserVisible,
    subject: impl Into<String>,
    html_content: impl Into<String>,
    text_content: impl Into<String>,
) -> Result<(), String> {
    let settings = crate::settings::get_settings().expect("Failed to read settings.");

    let email = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                "StackOverflow clone with axum and SvelteKit",
                settings.email.host_user.clone()
            )
            .parse()
            .map_err(|e| {
                tracing::error!("Could not parse 'from' email address: {:#?}", e);
                format!("Could not parse 'from' email address: {:#?}", e)
            })?,
        )
        .to(format!(
            "{} <{}>",
            [user.first_name, user.last_name].join(" "),
            user.email
        )
        .parse()
        .map_err(|e| {
            tracing::error!("Could not parse 'to' email address: {:#?}", e);
            format!("Could not parse 'to' email address: {:#?}", e)
        })?)
        .subject(subject)
        .multipart(
            lettre::message::MultiPart::alternative()
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_PLAIN)
                        .body(text_content.into()),
                )
                .singlepart(
                    lettre::message::SinglePart::builder()
                        .header(lettre::message::header::ContentType::TEXT_HTML)
                        .body(html_content.into()),
                ),
        )
        .unwrap();

    let creds = lettre::transport::smtp::authentication::Credentials::new(
        settings.email.host_user,
        settings.email.host_user_password,
    );

    // Open a remote connection to gmail
    let mailer: lettre::AsyncSmtpTransport<lettre::Tokio1Executor> =
        lettre::AsyncSmtpTransport::<lettre::Tokio1Executor>::relay(&settings.email.host)
            .unwrap()
            .credentials(creds)
            .build();

    // Send the email
    match mailer.send(email).await {
        Ok(_) => {
            tracing::info!("Email sent successfully.");
            Ok(())
        }
        Err(e) => {
            tracing::error!("Could not send email: {:#?}", e);
            Err(format!("Could not send email: {:#?}", e))
        }
    }
}

#[tracing::instrument(
    name = "Generic multipart e-mail sending function.",
    skip(
        user,
        state,
        template_name
    ),
    fields(
        recipient_user_id = %user.id,
        recipient_email = %user.email,
        recipient_first_name = %user.first_name,
        recipient_last_name = %user.last_name
    )
)]
pub async fn send_multipart_email(
    subject: String,
    user: crate::models::UserVisible,
    state: crate::startup::AppState,
    template_name: &str,
    issued_token: String,
) -> Result<(), String> {
    let settings = crate::settings::get_settings().expect("Unable to load settings.");
    let title = subject.clone();

    let now = time::OffsetDateTime::now_utc();
    let expiration_time = now + time::Duration::minutes(settings.secret.token_expiration);
    let exact_time = expiration_time
        .format(&time::format_description::well_known::Rfc2822)
        .unwrap();

    let template = state.env.get_template(template_name).unwrap();
    let ctx = minijinja::context! {
        title => &title,
        user_id => &user.id,
        domain => &settings.frontend_url,
        token => &issued_token,
        expiration_time => &settings.secret.token_expiration,
        exact_time => &exact_time,
    };
    let html_text = template.render(ctx).unwrap();

    let text = format!(
        r#"
        Thanks for signing up for a StackOverflow clone with Rust (axum) and SvelteKit. We're excited to have you on board! 

        For future reference, your user ID number is {}.

        Please visit {}/auth/activate/{} and input the token below to activate your account:

        {}


        Please note that this is a one-time use token and it will expire in {} minutes ({}).


        Thanks,

        StackOverflow clone with Rust (axum) and SvelteKit Team 
        "#,
        user.id,
        settings.frontend_url,
        user.id,
        issued_token,
        settings.secret.token_expiration,
        exact_time
    );

    tokio::spawn(send_email(user, subject, html_text, text));
    Ok(())
}
