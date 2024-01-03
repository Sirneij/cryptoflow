mod crypto;
mod email;
mod errors;
mod middleware;
mod password;
mod qa;
mod query_constants;
mod responses;
mod user;

pub use crypto::{get_crypto_prices, CryptoPrices};
pub use email::send_multipart_email;
pub use errors::{CustomAppError, ErrorContext};
pub use middleware::validate_authentication_session;
pub use password::{hash_password, verify_password};
pub use qa::{convert_markdown_to_html, slugify};
pub use query_constants::{
    ANSWER_AUTHOR_QUERY, ANSWER_AUTHOR_QUERY_VIA_QUESTION_ID, QUESTION_AUTHOR_WITH_TAGS_QUERY,
    QUESTION_AUTHOR_WITH_TAGS_QUERY_ALL,
};
pub use responses::{CustomAppJson, SuccessResponse};
pub use user::get_user_id_from_session;
