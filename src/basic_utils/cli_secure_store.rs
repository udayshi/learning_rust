use aes_gcm::{
    aead::{Aead, Generate, KeyInit},
    Aes256Gcm, Key, Nonce,
};
use rusqlite::{Connection, Result};
use std::io::{self, Write};

pub fn run() -> Result<()> {
    let author = "Uday Shiwakoti";
    println!("--- Secure CLI Store for {} (uday@uday.com.np) ---", author);
    println!("Commands: 'add', 'list', 'get:<key>', or 'exit'");

    let conn = Connection::open("rust_cli_secure.db")?;

    // Create secure table with key-value support
    conn.execute(
        "CREATE TABLE IF NOT EXISTS kv_store (
            key TEXT PRIMARY KEY,
            ciphertext BLOB NOT NULL,
            nonce BLOB NOT NULL
        )",
        [],
    )?;

    // Generate session key and cipher
    let key = Key::<Aes256Gcm>::generate();
    let cipher = Aes256Gcm::new(&key);

    loop {
        print!("\nEnter command > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        if command == "exit" {
            println!("Exiting secure store. Goodbye!");
            break;
        } else if command == "add" {
            add_record(&conn, &cipher)?;
        } else if command == "list" {
            list_records(&conn, &cipher)?;
        } else if let Some(target_key) = command.strip_prefix("get:") {
            get_record(&conn, &cipher, target_key)?;
        } else {
            println!("Unknown command. Use 'add', 'list', 'get:<key>', or 'exit'.");
        }
    }

    Ok(())
}

fn add_record(conn: &Connection, cipher: &Aes256Gcm) -> Result<()> {
    print!("Enter key: ");
    io::stdout().flush().unwrap();
    let mut key_input = String::new();
    io::stdin().read_line(&mut key_input).unwrap();
    let k = key_input.trim();

    print!("Enter value to encrypt: ");
    io::stdout().flush().unwrap();
    let mut val_input = String::new();
    io::stdin().read_line(&mut val_input).unwrap();
    let v = val_input.trim();

    let nonce = Nonce::generate();
    let ciphertext = cipher
        .encrypt(&nonce, v.as_bytes())
        .expect("Encryption failed!");

    conn.execute(
        "INSERT OR REPLACE INTO kv_store (key, ciphertext, nonce) VALUES (?1, ?2, ?3)",
        rusqlite::params![k, ciphertext, nonce.as_slice()],
    )?;

    println!("Successfully encrypted and saved key: '{}'", k);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypts_and_decrypts_a_value() {
        let key = Key::<Aes256Gcm>::generate();
        let cipher = Aes256Gcm::new(&key);
        let nonce = Nonce::generate();
        let plaintext = b"secure value";

        let ciphertext = cipher
            .encrypt(&nonce, plaintext.as_ref())
            .expect("encryption should succeed");
        let decrypted = cipher
            .decrypt(&nonce, ciphertext.as_ref())
            .expect("decryption should succeed");

        assert_eq!(decrypted, plaintext);
    }
}

fn list_records(conn: &Connection, cipher: &Aes256Gcm) -> Result<()> {
    let mut stmt = conn.prepare("SELECT key, ciphertext, nonce FROM kv_store")?;
    let rows = stmt.query_map([], |row| {
        let k: String = row.get(0)?;
        let c: Vec<u8> = row.get(1)?;
        let n: Vec<u8> = row.get(2)?;
        Ok((k, c, n))
    })?;

    println!("--- Stored Keys & Decrypted Values ---");
    for row in rows {
        let (k, ciphertext, nonce_bytes) = row?;
        let Ok(nonce) = Nonce::try_from(nonce_bytes.as_slice()) else {
            println!("Key: {} => [Invalid nonce]", k);
            continue;
        };

        match cipher.decrypt(&nonce, ciphertext.as_ref()) {
            Ok(decrypted) => {
                let val = String::from_utf8_lossy(&decrypted);
                println!("Key: {} => Value: {}", k, val);
            }
            Err(_) => println!("Key: {} => [Decryption Error]", k),
        }
    }
    Ok(())
}

fn get_record(conn: &Connection, cipher: &Aes256Gcm, target_key: &str) -> Result<()> {
    let mut stmt = conn.prepare("SELECT ciphertext, nonce FROM kv_store WHERE key = ?1")?;
    let mut rows = stmt.query(rusqlite::params![target_key])?;

    if let Some(row) = rows.next()? {
        let ciphertext: Vec<u8> = row.get(0)?;
        let nonce_bytes: Vec<u8> = row.get(1)?;
        let Ok(nonce) = Nonce::try_from(nonce_bytes.as_slice()) else {
            println!("Stored nonce is invalid for key: '{}'", target_key);
            return Ok(());
        };

        match cipher.decrypt(&nonce, ciphertext.as_ref()) {
            Ok(decrypted) => {
                let val = String::from_utf8_lossy(&decrypted);
                println!("Result for '{}': {}", target_key, val);
            }
            Err(_) => println!("Decryption failed for key: '{}'", target_key),
        }
    } else {
        println!("Key '{}' not found.", target_key);
    }
    Ok(())
}
