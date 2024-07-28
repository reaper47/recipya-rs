pub const INSERT_USER: &str = "
    INSERT INTO users (email, hashed_password)
	VALUES ($1, $2)
	RETURNING id
";
