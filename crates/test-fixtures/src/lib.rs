use std::{
    fs::File,
    io::{Cursor, Read},
    sync::OnceLock,
};

use uuid::Uuid;

use auth::token::generate_access_token;
use models::user::{User, UserForCreate};
use repository::ModelManager;

/// The email address of the default user in the test database.
pub const TEST_USER_EMAIL: &str = "test@test.com";

/// The password of the default user in the test database.
pub const TEST_USER_PASSWORD: &str = "12345678";

/// The email address of the demo user in the test database.
pub const TEST_DEMO_EMAIL: &str = "demo@demo.com";

pub static TEST_PASSWORD_SALT: OnceLock<Uuid> = OnceLock::new();

pub const TEST_PASSWORD_HASH: &str = "#02#$argon2id$v=19$m=19456,t=2,p=1$bpxlx+8RTH2isz8k1XIyXw$aknwhS/gQtL5taRD5jSgHOqUjxfrI4kifQWbvV0WY/U";

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

pub struct RecipeImages {
    pub main: Uuid,
    pub additional: Uuid,
    pub video: Uuid,
}

impl Default for RecipeImages {
    fn default() -> Self {
        Self {
            main: Uuid::new_v4(),
            additional: Uuid::new_v4(),
            video: Uuid::new_v4(),
        }
    }
}

/// Gets the password salt.
///
/// The variable holding the value will be initialised the first time the function is called.
///
/// # Panics
///
/// Panics when the UUID is invalid.
pub fn get_password_salt() -> Uuid {
    *TEST_PASSWORD_SALT
        .get_or_init(|| Uuid::parse_str("6e9c65c7-ef11-4c7d-a2b3-3f24d572325f").unwrap())
}

/// Generates a token for a newly created test user.
///
/// # Panics
///
/// Panics if the user cannot be found after creation.
pub async fn get_token(mm: ModelManager) -> Result<String> {
    let email = "confirm@test.com";

    User::new_with_hash(
        &mm,
        UserForCreate {
            email: email.to_string(),
            password_clear: "12345678".to_string(),
        },
        get_password_salt(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?;

    let user = User::get_user_by_email(&mm, email)
        .await?
        .expect("user not found");

    Ok(generate_access_token(&user.id)?)
}

/// Inserts a test user in the database.
pub async fn insert_user(mm: &ModelManager) -> Result<User> {
    insert_user_helper(mm, TEST_USER_EMAIL).await
}

/// Inserts another test user in the database.
pub async fn insert_other_user(mm: &ModelManager, email: &str) -> Result<User> {
    insert_user_helper(mm, email).await
}

async fn insert_user_helper(mm: &ModelManager, email: &str) -> Result<User> {
    Ok(User::new_with_hash(
        mm,
        UserForCreate {
            email: email.into(),
            password_clear: TEST_USER_PASSWORD.into(),
        },
        get_password_salt(),
        TEST_PASSWORD_HASH.to_string(),
    )
    .await?)
}

/// Opens a data test file and returns its contents as a `Cursor`.
///
/// # Panics
///
/// Panics if the file does not exist or cannot be read.
pub fn open_test_file(filename: &str) -> Cursor<Vec<u8>> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("integrations")
        .join("tests")
        .join("data")
        .join(filename);

    let mut file = File::open(path).expect("file to exist");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("failed to read file");
    Cursor::new(buf)
}
