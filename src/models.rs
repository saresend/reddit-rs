#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    token_type: String,
    expires_in: u32,
    scope: String,
}
