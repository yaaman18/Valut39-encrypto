<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let inputSeed = "";
  let password = "";
  let cipher = "";
  let passwordWarning = "";
  let showPassword = false;
  let seedWarning = "";
  let seedWordCount = 0;
  let minimumWordCount = 12;

  $: seedWordCount = inputSeed
    .split(" ")
    .filter((word) => word.length > 0).length;

  function togglePasswordVisibility() {
    showPassword = !showPassword;
  }

  async function handleFormSubmit() {
    seedWarning = "";
    cipher = "";

    const inputWords = inputSeed.trim().split(/\s+/);
    seedWordCount = inputWords.length;

    if (seedWordCount < 12 || seedWordCount > 24) {
        seedWarning = "Please enter a seed phrase between 12 and 24 words.";
        return;
    }
    try {
      const wordlist_en: string = await invoke("read_wordlist_file");
      const wordSet = new Set(wordlist_en.split(/\r?\n/));
      const inputWords = inputSeed.trim().split(/\s+/).filter(word => word.length > 0);

      seedWordCount = inputWords.length;


      const uniqueWordsSet = new Set(inputWords);
      if (uniqueWordsSet.size !== inputWords.length) {
        seedWarning = "Do not enter the same word twice in your seed phrase.";
        return;
      }

      for (let word of inputWords) {
        if (inputWords.length < minimumWordCount || inputWords.some(word => !wordSet.has(word))) {
  seedWarning = "The words in your seed phrase are incorrect";
  return;
}
      }
    } catch (error) {
      console.error("Error:", error);
    }

    try {
      cipher = await invoke("wrap_handle_data", {
        inputSeed: inputSeed,
        password: password,
      });
    } catch (error) {
      console.error("Error invoking wrap_handle_data:", error);
    }
  }

  function copyToClipboard(text: string) {
    navigator.clipboard.writeText(text).then(
      function () {
        console.log("Copy successful");
      },
      function (err) {
        console.error("Copy failed", err);
      },
    );
  }

  const handlePaste = (event: ClipboardEvent) => {
    event.preventDefault();
    const clipboardData = event.clipboardData;
    if (clipboardData) {
        const pastedData = clipboardData.getData('text');
        inputSeed = pastedData.replace(/\r\n/g, '\n').replace(/\n/g, ' ');
    }
  };

</script>

<main>
  <h1>Valut39</h1>

  <h2>Generate encryption</h2>
  <p>
    Please enter your wallet's seed phrase and your chosen password here.
  </p>
  <p>
    Never forget your password, as it is required to restore your seed phrase from the encrypted text.
  </p>
  <p>
    Please enter your password using only alphanumeric characters, without spaces.
  </p>
  <p>We recommend using this application in an offline environment to reduce the risk of hacking.</p>

  <form on:submit|preventDefault={handleFormSubmit}>
    <label>
      seed phrase:
      <textarea bind:value={inputSeed} style="width: 100%; white-space: pre-wrap;" rows="2" on:paste={handlePaste}></textarea>
    </label>
    <p>Number of words entered: {seedWordCount}</p>
    {#if seedWarning}
      <p style="color: red;">{seedWarning}</p>
    {/if}

    <label>
      password:
      {#if showPassword}
        <input
          type="text"
          bind:value={password}
          style="width: 100%;"
          minlength="4"
          maxlength="80"
          class="password-input"
        />
        <button type="button" on:click={() => copyToClipboard(password)}
          >copy password</button
        >
      {:else}
        <input
          type="password"
          bind:value={password}
          style="width: 100%;"
          minlength="8"
          class="password-input"
        />
        <button type="button" on:click={() => copyToClipboard(password)}
          >copy password</button
        >
      {/if}
      <button
        type="button"
        on:click={togglePasswordVisibility}
        class="toggle-button"
        >{showPassword ? "disabled " : "enabled"}</button
      >
    </label>
    {#if passwordWarning}
      <p style="color: red;">{passwordWarning}</p>
    {/if}
    <button type="submit">Generate encrypted text</button>
  </form>

  {#if cipher}
    <p>encrypted text: {cipher}</p>
    <button type="button" on:click={() => copyToClipboard(cipher)}
      >copy encrypted text</button
    >
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



  form {
    margin: 6px 0;
  }
  p {
    font-size: 16px;
    margin-bottom: 10px;
  }

  label {
    display: block;
    margin-bottom: 10px;
  }
  textarea{
    padding: 5px;
    font-size: 16px;
    width: 48vw;
    height: 12vw;
  }
  input {
    padding: 5px;
    font-size: 16px;
    width: 48vw;
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