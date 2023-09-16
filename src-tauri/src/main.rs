// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate chacha20;
extern crate bs58;

use std::collections::HashMap;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use std::error::Error;
use tokio::fs::read_to_string;
use generic_array::GenericArray;
use typenum::{U32, U12};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;


#[derive(serde::Deserialize)]
struct HandleDataArgs {
    input_seed: String,
    password: String,
}

#[derive(serde::Deserialize)]
struct MinimalizeSeedsArgs {
    input_seed_phrase: String,
}


#[tauri::command]
async fn handle_data(args: HandleDataArgs) -> tauri::Result<String> {
    // minimalize_seeds関数を呼び出して、短縮されたシードフレーズを取得
    let minimalized_seeds = match minimalize_seeds(MinimalizeSeedsArgs { input_seed_phrase: args.input_seed }).await {
        Ok(result) => result,
        Err(e) => {
            return Err(e.into());
        }
    };

    // generate_cipher関数を呼び出して、暗号文を生成
    let cipher_text = match generate_cipher(&minimalized_seeds, &args.password).await {
        Ok(result) => result,
        Err(e) => {
           return Err(tauri::Error::from(io::Error::new(io::ErrorKind::Other, e.to_string())));
        }
    };

    // 生成された暗号文を返す
    Ok(cipher_text)
}

#[tauri::command]
async fn minimalize_seeds(args: MinimalizeSeedsArgs) -> tauri::Result<String> {

  // Construct the path to the wordlist_en.txt file
    let mut path_en = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_en.push("src");
    path_en.push("resources");
    path_en.push("wordlist_en.txt");

    // Construct the path to the wordlist_minimal.txt file
    let mut path_minimal = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_minimal.push("src");
    path_minimal.push("resources");
    path_minimal.push("wordlist_minimal.txt");

    println!("Path to wordlist_en.txt: {:?}", path_en);

    let wordlist_en = read_to_string(&path_en).await?;
    let wordlist_minimal = read_to_string(&path_minimal).await?;

    let wordlist_en: Vec<String> = wordlist_en.lines().map(|s| s.to_owned()).collect();
    let wordlist_minimal: Vec<String> = wordlist_minimal.lines().map(|s| s.to_owned()).collect();

    let en_to_index: HashMap<_, _> = wordlist_en.iter().enumerate().map(|(i, word)| (word, i)).collect();

    let input_seeds: Vec<String> = args.input_seed_phrase.split_whitespace().map(|s| s.to_owned()).collect();

    let minimalized_seeds: String = input_seeds.iter()
        .filter_map(|seed| en_to_index.get(seed).map(|&index| &wordlist_minimal[index]))
        .cloned()
        .collect::<Vec<String>>()
        .concat();  // Changed from .join(" ") to .concat()

    Ok(minimalized_seeds)
}


async fn string_to_32_byte_array(str: &str) -> Result<[u8; 32], Box<dyn Error>> {
    let mut byte_array = [0u8; 32];
    let encoded = str.as_bytes();

    if encoded.len() > 32 {
        return Err("String too long to convert to 32-byte array".into());
    }

    byte_array[0..encoded.len()].copy_from_slice(encoded);
    Ok(byte_array)
}

async fn generate_cipher(input_seed: &str, password: &str) -> Result<String, Box<dyn Error>> {
    let mut secret_buff = [0u8; 32];
    let password_bytes = password.as_bytes();
    secret_buff[(32 - password_bytes.len())..].copy_from_slice(password_bytes);  // 32に修正

    let secret: GenericArray<u8, U32> = GenericArray::clone_from_slice(&secret_buff);
    let nonce_buff = [0u8; 12];
    let nonce: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce_buff);

    let minimal_seed = string_to_32_byte_array(input_seed).await?;

    let mut cipher = ChaCha20::new(&secret, &nonce);

    let mut encrypted_bytes = minimal_seed.clone();
    cipher.apply_keystream(&mut encrypted_bytes);
    let encrypted = bs58::encode(encrypted_bytes).into_string();

    if encrypted.len() > 44 {
        return Err("Encrypted data is too long".into());
    }

    let mut decrypted_bytes = bs58::decode(&encrypted).into_vec()?;
    let original_length = input_seed.as_bytes().len();
    decrypted_bytes.truncate(original_length);

    let mut cipher = ChaCha20::new(&secret.into(), &nonce.into());
    cipher.apply_keystream(&mut decrypted_bytes);

    let decrypted = String::from_utf8(decrypted_bytes)?;

    if decrypted != input_seed {
        return Err("Decrypted data does not match minimal_seed".into());
    }

    Ok(encrypted)
}

#[tokio::main]
async fn main() {
    let mut input_seed = String::new();
    let mut password = String::new();


    print!("Please enter your seed phrase: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input_seed).unwrap();

    print!("Please enter your password: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut password).unwrap();

    // 末尾の改行を削除
    input_seed = input_seed.trim().to_string();
    password = password.trim().to_string();

    let args = MinimalizeSeedsArgs {
        input_seed_phrase: input_seed,
    };

    let minimalized_seeds = match minimalize_seeds(args).await {
         Ok(result) => {
            // minimalized_seedsの内容をターミナルに出力
            println!("minimalized_seeds: {}", result);
            result
        },
        Err(e) => {
            println!("Error in minimalize_seeds: {}", e);
            return;
        }
    };

    let cipher = match generate_cipher(&minimalized_seeds, &password).await {
        Ok(result) => result,
        Err(e) => {
            println!("Error in generate_cipher: {}", e);
            return;
        }
    };

    println!("Generated cipher text: {}", cipher);

    println!("minimalized_seeds: {}", minimalized_seeds);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_data, minimalize_seeds])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
