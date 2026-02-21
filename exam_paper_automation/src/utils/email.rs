use lettre::{
    message::header::ContentType,
    transport::smtp::authentication::Credentials,
    Message, AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

pub struct EmailService {
    smtp_host: String,
    smtp_port: u16,
    smtp_username: String,
    smtp_password: String,
    from_email: String,
}

impl EmailService {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
    ) -> Self {
        Self {
            smtp_host,
            smtp_port,
            smtp_username,
            smtp_password,
            from_email,
        }
    }

    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        reset_token: &str,
    ) -> Result<(), String> {
        let email_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .token-box {{ 
                        background-color: #f0f0f0; 
                        padding: 15px; 
                        margin: 20px 0;
                        border-radius: 5px;
                        font-size: 18px;
                        font-weight: bold;
                        text-align: center;
                        letter-spacing: 2px;
                    }}
                    .warning {{ color: #d9534f; font-weight: bold; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <h2>Password Reset Request</h2>
                    <p>You have requested to reset your password for the Exam Paper Automation System.</p>
                    <p>Please use the following token to reset your password:</p>
                    <div class="token-box">{}</div>
                    <p>This token will expire in 1 hour.</p>
                    <p class="warning">If you did not request this password reset, please ignore this email.</p>
                    <hr>
                    <p style="font-size: 12px; color: #777;">
                        This is an automated email. Please do not reply.
                    </p>
                </div>
            </body>
            </html>
            "#,
            reset_token
        );

        let email = Message::builder()
            .from(self.from_email.parse().map_err(|e| format!("Invalid from email: {}", e))?)
            .to(to_email.parse().map_err(|e| format!("Invalid to email: {}", e))?)
            .subject("Password Reset Token - Exam Paper Automation")
            .header(ContentType::TEXT_HTML)
            .body(email_body)
            .map_err(|e| format!("Failed to build email: {}", e))?;

        let credentials = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host)
            .map_err(|e| format!("Failed to create SMTP transport: {}", e))?
            .port(self.smtp_port)
            .credentials(credentials)
            .build();

        mailer
            .send(email)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }

    pub async fn send_first_login_email(
        &self,
        to_email: &str,
        temporary_password: &str,
    ) -> Result<(), String> {
        let email_body = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <style>
                    body {{ font-family: Arial, sans-serif; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .credentials {{ 
                        background-color: #f0f0f0; 
                        padding: 15px; 
                        margin: 20px 0;
                        border-radius: 5px;
                    }}
                    .warning {{ color: #d9534f; font-weight: bold; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <h2>Welcome to Exam Paper Automation System</h2>
                    <p>Your account has been created. Please use the following credentials to log in:</p>
                    <div class="credentials">
                        <p><strong>Email:</strong> {}</p>
                        <p><strong>Temporary Password:</strong> {}</p>
                    </div>
                    <p class="warning">You will be required to change your password on first login.</p>
                    <hr>
                    <p style="font-size: 12px; color: #777;">
                        This is an automated email. Please do not reply.
                    </p>
                </div>
            </body>
            </html>
            "#,
            to_email, temporary_password
        );

        let email = Message::builder()
            .from(self.from_email.parse().map_err(|e| format!("Invalid from email: {}", e))?)
            .to(to_email.parse().map_err(|e| format!("Invalid to email: {}", e))?)
            .subject("Account Created - Exam Paper Automation")
            .header(ContentType::TEXT_HTML)
            .body(email_body)
            .map_err(|e| format!("Failed to build email: {}", e))?;

        let credentials = Credentials::new(
            self.smtp_username.clone(),
            self.smtp_password.clone(),
        );

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&self.smtp_host)
            .map_err(|e| format!("Failed to create SMTP transport: {}", e))?
            .port(self.smtp_port)
            .credentials(credentials)
            .build();

        mailer
            .send(email)
            .await
            .map_err(|e| format!("Failed to send email: {}", e))?;

        Ok(())
    }
}
