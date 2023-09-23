use std::collections::HashMap;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use std::error::Error;
use generic_array::GenericArray;
use typenum::{U32, U12};
use std::env;
use std::path::PathBuf;
use tokio::fs::read_to_string;
use sha2::{Digest, Sha256};


#[derive(serde::Deserialize)]
pub struct MinimalizeSeedsArgs {
    input_seed_phrase: String,
}


#[tauri::command]
pub async fn handle_data(input_seed: String, password: String) -> tauri::Result<String> {
    println!("Received input_seed: {}", input_seed);
    println!("Received password: {}", password);

    // minimalize_seeds関数を呼び出して、短縮されたシードフレーズを取得
    let minimalized_seeds = match minimalize_seeds(MinimalizeSeedsArgs { input_seed_phrase: input_seed }).await {
        Ok(result) => result,
        Err(e) => {
            return Err(e.into());
        }
    };

    // generate_cipher関数を呼び出して、暗号文を生成
    let cipher_text = match generate_cipher(&minimalized_seeds, &password).await {
        Ok(result) => result,
        Err(e) => {
            return Err(tauri::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())));
        }
    };

    Ok(cipher_text)
}

#[tauri::command]
pub async fn minimalize_seeds(args: MinimalizeSeedsArgs) -> tauri::Result<String> {

    let mut path_en = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path_en.push("src");
    path_en.push("resources");
    path_en.push("wordlist_en.txt");

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
        .concat();

    Ok(minimalized_seeds)
}


async fn string_to_32_byte_array(str: &str) -> Result<[u8; 32], Box<dyn Error>> {
    let mut hasher = Sha256::new();
    hasher.update(str);
    let result = hasher.finalize();
    let byte_array: [u8; 32] = result.into();
    Ok(byte_array)
}

async fn generate_cipher(input_seed: &str, password: &str) -> Result<String, Box<dyn Error>> {
    // パスワードをSHA-256でハッシュ化
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();

    let secret: GenericArray<u8, U32> = GenericArray::clone_from_slice(&result);
    let nonce_buff = [0u8; 12];
    let nonce: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce_buff);

    // 元のinput_seedをバイト配列に変換
    let input_seed_bytes = input_seed.as_bytes();

    let mut cipher = ChaCha20::new(&secret, &nonce);

    let mut encrypted_bytes = input_seed_bytes.to_vec();
    cipher.apply_keystream(&mut encrypted_bytes);
    let encrypted = bs58::encode(encrypted_bytes).into_string();

    Ok(encrypted)
}
