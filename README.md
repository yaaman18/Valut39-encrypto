# Tauri Encrypto Application

## Overview
This Tauri application, written in Rust, offers secure data handling by encrypting seed phrases using the ChaCha20 cipher algorithm. The application is designed to minimalize seed phrases and encrypt them, providing an added layer of security.

## Features
- **Minimalize Seed Phrases**: Compresses standard seed phrases into a minimal format using a custom algorithm.
- **Encrypt Data**: Utilizes the ChaCha20 cipher to encrypt the minimalized seed phrases.
- **Secure Hashing**: Implements SHA-256 for secure password hashing.

## Usage
- `handle_data`: Encrypts the provided seed phrase with the given password.
- `minimalize_seeds`: Compresses a standard seed phrase into a minimal format.
- `read_wordlist_file`: Reads the wordlist file necessary for the minimalization process.

## How to Run
1. Install Rust and the Tauri dependencies.
2. Clone the repository.
3. Run `cargo build` to compile the application.
4. Execute the application.

## Dependencies
- ChaCha20 for encryption.
- SHA-256 for hashing.
- Tokio for asynchronous programming.
- Serde for serialization and deserialization.

## License
Copyright © 2024 Solidity Materials Co., Ltd. All Rights Reserved.

## Disclaimer
This software is provided "as is", without warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose and noninfringement. In no event shall the authors or copyright holders be liable for any claim, damages or other liability, whether in an action of contract, tort or otherwise, arising from, out of or in connection with the software or the use or other dealings in the software.

---

# Tauri Encrypto アプリケーション

## 概要
このTauriアプリケーションはRustで書かれており、ChaCha20暗号アルゴリズムを使用してシードフレーズを安全に処理します。アプリケーションはシードフレーズを簡略化し、暗号化することでセキュリティを強化します。

## 特徴
- **シードフレーズの簡略化**: 標準的なシードフレーズをカスタムアルゴリズムを使用して簡略化します。
- **データの暗号化**: ChaCha20暗号を利用して簡略化されたシードフレーズを暗号化します。
- **セキュアなハッシング**: パスワードの安全なハッシュ化のためにSHA-256を実装します。

## 使い方
- `handle_data`: 提供されたシードフレーズを指定されたパスワードで暗号化します。
- `minimalize_seeds`: 標準的なシードフレーズを簡略化した形式に圧縮します。
- `read_wordlist_file`: 簡略化プロセスに必要な単語リストファイルを読み込みます。

## 実行方法
1. RustおよびTauriの依存関係をインストールします。
2. リポジトリをクローンします。
3. `cargo build`を実行してアプリケーションをコンパイルします。
4. アプリケーションを実行します。

## 依存関係
- 暗号化のためのChaCha20。
- ハッシュ化のためのSHA-256。
- 非同期プログラミングのためのTokio。
- シリアライズとデシリアライズのためのSerde。

## ライセンス
Copyright © 2024 Solidity Materials Co., Ltd. All Rights Reserved.

## 免責事項
このソフトウェアは「現状のまま」提供されており、商品性、特定目的への適合性、および権利侵害を含め、明示的または暗黙的ないかなる保証も伴いません。作者または著作権所有者は、契約、不法行為、またはその他の形態にかかわらず、このソフトウェアの使用またはその他の取引から生じるいかなるクレーム、損害、その他の責任についても責任を負いません。
