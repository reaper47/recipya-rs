use axum::{
    extract::{Form, FromRequest, rejection::FormRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;
use thiserror::Error;
use validator::Validate;

/// Extractor and validates `application/x-www-form-urlencoded` data.
///
/// # Errors
///
/// Returns a [`ServerError`] if the body cannot be parsed.
///
/// # Example
///
/// Use a form directly:
///
/// ```ignore
/// #[derive(Debug, Validate, Deserialize, Serialize)]
/// pub struct LoginForm {
///     pub email: Email,
///     pub password: Password,
///     pub remember_me: Option<bool>,
/// }
///
/// pub async fn login_post_handler(
///     RequestLanguage(messages): RequestLanguage<Messages>,
///     State(state): State<AppState>,
///     cookies: Cookies,
///     ValidatedForm(form): ValidatedForm<LoginForm>,
/// ) -> impl IntoResponse {
///     // `form` is parsed and validated, ready to use
/// }
/// ```
///
/// Send a toast to the user on form validation failure:
///
/// ```ignore
/// #[derive(Debug, Validate, Deserialize, Serialize)]
/// pub struct LoginForm {
///     pub email: Email,
///     pub password: Password,
///     pub remember_me: Option<bool>,
/// }
///
/// pub async fn login_post_handler(
///     RequestLanguage(messages): RequestLanguage<Messages>,
///     State(state): State<AppState>,
///     cookies: Cookies,
///     form: Result<ValidatedForm<LoginForm>, ServerError>,
/// ) -> impl IntoResponse {
///     let toast = ToastCtx::<UnauthenticatedState>::new(&state, &messages);
///
///     let ValidatedForm(form) = match form {
///         Ok(form) => form,
///         Err(_) => {
///             return toast.error(
///                 Error::Form.into_response(),
///                 &messages.toast_auth_credentials_invalid(),
///             );
///         }
///     };
///
///     // Rest of the handler
/// }
/// ```
pub struct ValidatedForm<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedForm<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Form<T>: FromRequest<S, Rejection = FormRejection>,
{
    type Rejection = ServerError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedForm(value))
    }
}

/// Errors that can occur while handling a request.
///
/// # Variants
///
/// - [`ValidationError`](Self::ValidationError): the request parsed, but failed the [`validator`] rules.
/// - [`AxumFormRejection`](Self::AxumFormRejection): the request body could not be parsed as a form.
#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] FormRejection),
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = format!("Input validation error: [{self}]").replace('\n', ", ");
                (StatusCode::BAD_REQUEST, message)
            }
            ServerError::AxumFormRejection(_) => (StatusCode::BAD_REQUEST, self.to_string()),
        }
        .into_response()
    }
}
