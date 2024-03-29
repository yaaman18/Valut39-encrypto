<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let inputSeed = "";
  let password = "";
  let cipher = "";
  let passwordWarning = "";
  let showPassword = false;

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  async function handleFormSubmit() {

    try {
        cipher = await invoke('wrap_handle_data', { inputSeed: inputSeed, password: password });
    } catch (error) {
        console.error("Error invoking wrap_handle_data:", error);
    }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text).then(
    function () {
      console.log("コピー成功");
    },
    function (err) {
      console.error("コピー失敗", err);
    }
  );
}

</script>


<main>
  <h1>Nil Wallet: あなたのデジタル資産を安全に管理</h1>
  <h2>シンプルで安全なウォレット、手の中に。</h2>
  <p>Nil Walletは、デジタル通貨の管理をよりシンプルで安全にするための革新的なウォレットアプリケーションです。ユーザーフレンドリーなインターフェイスと先進的なセキュリティ機能を組み合わせて、初心者から上級者まで、誰でも簡単にデジタル資産を管理できます。</p>
  <h3>主な機能:</h3>
  <ul>
    <li>セキュアな暗号化</li>
    <li>マルチプラットフォーム対応</li>
    <li>直感的な操作</li>
    <li>カスタマーサポート</li>
  </ul>
  <p>Nil Walletで、デジタル通貨の新しい時代を体験しましょう。今すぐダウンロードして、安全で便利なデジタル資産管理の世界へ飛び込みましょう。</p>
  <h3>暗号生成</h3>
  <p>こちらにお手持ちのwalletのシードフレーズと復号する時に必要なパスワード（最低８文字）を入力してください。</p>
  <p>パスワードは暗号文からシードフレーズを復元する時に必要なので絶対に忘れないでください。</p>
  <p>入力するパスワードは半角英数字のみでスペースを入れずに入力してください。</p>
  <p>尚オフラン環境でのみ利用可能になっています。</p>
  <p>また、パソコンの中にシードフレーズもパスワードも保存しないので安心してご利用いただけます</p>
  <form on:submit|preventDefault={handleFormSubmit}>
  <label>
    シードフレーズ:
      <input type="text" bind:value={inputSeed} style="width: 100%;" />
  </label>
  <label>
  パスワード:
  {#if showPassword}
    <input type="text" bind:value={password} style="width: 100%;" minlength="4" maxlength="80" class="password-input" />
    <button type="button" on:click={() => copyToClipboard(password)}>パスワードをコピー</button>
  {:else}
    <input type="password" bind:value={password} style="width: 100%;" minlength="8" class="password-input" />
    <button type="button" on:click={() => copyToClipboard(password)}>パスワードをコピー</button>
  {/if}
  <button type="button" on:click={togglePasswordVisibility} class="toggle-button">{showPassword ? '非表示にする' : 'パスワードを表示する'}</button>
  </label>
  {#if passwordWarning}
    <p style="color: red;">{passwordWarning}</p>
  {/if}
  <button type="submit">暗号文を生成する</button>
</form>

  {#if cipher}
    <p>暗号文: {cipher}</p>
    <button type="button" on:click={() => copyToClipboard(cipher)}>暗号文をコピー</button>
  {/if}


</main>

<style>
  :global(body) {
    margin: 0;
    background: linear-gradient(to bottom right, #afeeee 33%, #f0e68c 66%);
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 100vh;
  }
  main {
    padding: 20px;
    font-family: sans-serif;
    color: #696969;
    width: 66%;
    box-sizing: border-box;
  }
  h1 {
    font-size: 24px;
    margin-bottom: 20px;
  }
  h2 {
    font-size: 20px;
    margin-bottom: 15px;
  }

   h3 {
    font-size: 18px;
    margin-top: 30px;
    margin-bottom: 20px;
  }

  form {
    margin: 6px 0;
  }
  p {
    font-size: 16px;
    margin-bottom: 10px;
  }
  ul {
    font-size: 16px;
    margin-bottom: 10px;
  }
  label {
    display: block;
    margin-bottom: 10px;
  }
  input {
    padding: 5px;
    font-size: 16px;
    width: 30vw;
  }

  .toggle-button {
    margin-top: 20px;
  }
  button {
    padding: 5px 10px;
    font-size: 16px;
    cursor: pointer;
  }

</style>