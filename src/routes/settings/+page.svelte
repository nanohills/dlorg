<script lang="ts">
  import { invoke } from '@tauri-apps/api/core';
  import { onMount } from 'svelte';

  let downloadsPath = "/home/a/Downloads";
  let renameFiles = false;

  interface CategoryRule { name: string; keywords: string; }
  let rules: CategoryRule[] = [
    { name: "office", keywords: "invoice, meeting, project, salary, employee, report" },
    { name: "health", keywords: "diagnosis, prescription, clinic, hospital, patient, doctor" },
    { name: "bank", keywords: "account, balance, transaction, statement, ifsc" },
    { name: "academics", keywords: "marksheet, semester, university, grade, exam, syllabus" },
    { name: "gov_ids", keywords: "passport, aadhaar, license, identity, nationality" },
  ];

  let saved = false;

  onMount(async () => {
    try {
      const settings = await invoke<{
        downloads_path: string;
        rename_files: boolean;
        rules: CategoryRule[];
      }>('load_settings');
      downloadsPath = settings.downloads_path;
      renameFiles = settings.rename_files;
      if (settings.rules?.length) rules = settings.rules;
    } catch (e) {
      console.error('failed to load settings:', e);
    }
  });

  function addRule() {
    rules = [...rules, { name: "", keywords: "" }];
  }

  function removeRule(i: number) {
    rules = rules.filter((_, idx) => idx !== i);
  }

  async function browseFolder() {
    try {
      const path = await invoke<string | null>('pick_folder');
      if (path) downloadsPath = path;
    } catch (e) {
      console.error('folder picker failed:', e);
    }
  }

  async function saveSettings() {
    try {
      await invoke('save_settings', {
        settings: {
          downloads_path: downloadsPath,
          rename_files: renameFiles,
          rules,
        },
      });
      saved = true;
      setTimeout(() => (saved = false), 1500);
    } catch (e) {
      console.error('failed to save settings:', e);
    }
  }
</script>

<div class="settings-page">
  <div class="page-header">
    <a href="/" class="back-btn">
      <span class="material-symbols-outlined">arrow_back</span>
    </a>
    <h1>Settings</h1>
  </div>

  <section>
    <h2>Watched folder</h2>
    <div class="field-row">
      <input type="text" bind:value={downloadsPath} />
      <button on:click={browseFolder}>
        <span class="material-symbols-outlined">folder_open</span>
        Browse
      </button>
    </div>
  </section>

  <section>
    <h2>Categorization</h2>
    <p class="hint">Categories used for classification. Keywords help as a fallback signal.</p>

    {#each rules as rule, i}
      <div class="rule-row">
        <input type="text" placeholder="Category name" bind:value={rule.name} />
        <input type="text" placeholder="Keywords (comma separated)" bind:value={rule.keywords} />
        <button class="icon-btn" on:click={() => removeRule(i)}>
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
    {/each}
    <button class="add-btn" on:click={addRule}>
      <span class="material-symbols-outlined">add</span>
      Add category
    </button>
  </section>

  <section>
    <div class="section-header">
      <h2>File handling</h2>
    </div>
    <label class="toggle-row">
      <span>Rename files using suggested name</span>
      <label class="switch">
        <input type="checkbox" bind:checked={renameFiles} />
        <span class="slider"></span>
      </label>
    </label>
  </section>

  <button class="save-btn" on:click={saveSettings}>
    <span class="material-symbols-outlined">{saved ? "check" : "save"}</span>
    {saved ? "Saved" : "Save settings"}
  </button>

  <section class="about">
    <h2>About</h2>
    <p>dlorg v1.0.0-beta</p>
    <p>Built by Alt</p>
  </section>
</div>

<style>
  .settings-page { padding-bottom: 40px; }

  .page-header {
    display: flex; align-items: center; gap: 16px;
    margin-bottom: 24px;
  }
  .back-btn {
    display: inline-flex; align-items: center; justify-content: center;
    width: 40px; height: 40px;
    background: #2a2a2a;
    border-radius: 50%;
    color: #ccc;
    text-decoration: none;
    flex-shrink: 0;
  }
  h1 { margin: 0; color: #e5e5e5; }

  section {
    background: #1c1c1c;
    border-radius: 16px;
    padding: 20px;
    margin-bottom: 16px;
  }
  .section-header {
    display: flex; justify-content: space-between; align-items: center;
    margin-bottom: 4px;
  }
  section h2 { font-size: 15px; margin: 0 0 12px 0; color: #ccc; }
  .hint { color: #777; font-size: 13px; margin: 0 0 16px 0; }

  .field-row { display: flex; gap: 8px; }
  .field-row input {
    flex: 1;
    background: #262626; border: none; border-radius: 8px;
    padding: 10px 14px; color: #e5e5e5;
  }
  .field-row button, .add-btn {
    background: #2a2a2a; border: none; color: #ccc;
    border-radius: 8px; padding: 10px 16px; cursor: pointer;
    display: flex; align-items: center; gap: 6px;
  }
  .add-btn { margin-top: 4px; }

  .rule-row { display: flex; gap: 8px; margin-bottom: 8px; align-items: center; }
  .rule-row input {
    background: #262626; border: none; border-radius: 8px;
    padding: 10px 14px; color: #e5e5e5; flex: 1;
  }
  .icon-btn {
    background: transparent; border: none; color: #666;
    cursor: pointer; display: flex; padding: 6px;
  }

  .toggle-row {
    display: flex; justify-content: space-between; align-items: center;
    font-size: 14px; color: #ddd;
  }

  .switch {
    position: relative; display: inline-block;
    width: 40px; height: 22px;
  }
  .switch input { opacity: 0; width: 0; height: 0; }
  .slider {
    position: absolute; cursor: pointer;
    top: 0; left: 0; right: 0; bottom: 0;
    background: #333; border-radius: 22px;
    transition: 0.2s;
  }
  .slider::before {
    position: absolute; content: "";
    height: 16px; width: 16px;
    left: 3px; bottom: 3px;
    background: #ccc; border-radius: 50%;
    transition: 0.2s;
  }
  .switch input:checked + .slider { background: #8a1e5c; }
  .switch input:checked + .slider::before {
    transform: translateX(18px);
    background: #ff6ec7;
  }

  .save-btn {
    display: flex; align-items: center; gap: 8px;
    background: #8a1e5c; color: #ff6ec7;
    border: none; border-radius: 12px;
    padding: 12px 20px;
    font-weight: 600;
    cursor: pointer;
    margin-bottom: 16px;
  }

  .about p { color: #888; font-size: 13px; margin: 4px 0; }
</style>