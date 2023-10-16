use std::collections::HashMap;
use chacha20::ChaCha20;
use chacha20::cipher::{KeyIvInit, StreamCipher};
use std::error::Error;
use generic_array::GenericArray;
use typenum::{U32, U12};
use sha2::{Digest, Sha256};

// リソースファイルの内容をバイナリに埋め込む
const WORDLIST_EN: &str = include_str!("../../src-tauri/src/resources/wordlist_en.txt");
const WORDLIST_MINIMAL: &str = include_str!("../../src-tauri/src/resources/wordlist_minimal.txt");

#[derive(serde::Deserialize)]
pub struct MinimalizeSeedsArgs {
    input_seed_phrase: String,
}


#[tauri::command]
pub async fn handle_data(input_seed: String, password: String) -> tauri::Result<String> {

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

    let wordlist_en: Vec<String> = WORDLIST_EN.lines().map(|s| s.to_owned()).collect();
    let wordlist_minimal: Vec<String> = WORDLIST_MINIMAL.lines().map(|s| s.to_owned()).collect();


    let en_to_index: HashMap<_, _> = wordlist_en.iter().enumerate().map(|(i, word)| (word, i)).collect();

    let input_seeds: Vec<String> = args.input_seed_phrase.split_whitespace().map(|s| s.to_owned()).collect();

    let minimalized_seeds: String = input_seeds.iter()
        .filter_map(|seed| en_to_index.get(seed).map(|&index| &wordlist_minimal[index]))
        .cloned()
        .collect::<Vec<String>>()
        .concat();

    Ok(minimalized_seeds)

}

fn string_to_32_byte_array(input: &str) -> Result<Vec<u8>, &'static str> {
    let mut buffer = vec![0u8; 32]; // 32 bytes buffer initialized with zeros
    let input_bytes = input.as_bytes();

    if input_bytes.len() > buffer.len() {
        return Err("String too long to convert to 32-byte array");
    }

    buffer[..input_bytes.len()].copy_from_slice(input_bytes);

    Ok(buffer)
}





async fn generate_cipher(input_seed: &str, password: &str) -> Result<String, Box<dyn Error>> {
    // パスワードをSHA-256でハッシュ化
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();

    let secret: GenericArray<u8, U32> = GenericArray::clone_from_slice(&result);
    let nonce_buff = [0u8; 12];
    let nonce: GenericArray<u8, U12> = GenericArray::clone_from_slice(&nonce_buff);

    // 元のinput_seedを32バイト配列に変換
    let input_seed_bytes = match string_to_32_byte_array(input_seed) {
        Ok(buffer) => buffer,
        Err(e) => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidInput, e))),
    };

    let mut cipher = ChaCha20::new(&secret, &nonce);

    let mut encrypted_bytes = input_seed_bytes.to_vec();
    cipher.apply_keystream(&mut encrypted_bytes);
    let encrypted = bs58::encode(encrypted_bytes).into_string();

    Ok(encrypted)
}
