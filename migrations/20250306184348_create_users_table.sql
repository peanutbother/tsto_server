CREATE TABLE IF NOT EXISTS users 
    (
        mayhem_id INT UNIQUE,
        user_id INT UNIQUE,
        user_access_token STRING,
        user_access_code STRING,
        user_verification_code INT,
        user_refresh_token STRING,
        user_email STRING UNIQUE,
        user_name String UNIQUE,
        session_id STRING UNIQUE,
        session_key STRING UNIQUE,
        whole_land_token STRING,
        creation_date DATETIME DEFAULT CURRENT_TIMESTAMP
    )
;
