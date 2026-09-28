<script lang="ts">
  let running = true;
  let query = "";

  interface Category { name: string; count: number; }
  let categories: Category[] = [
    { name: "docs", count: 12 },
    { name: "audio", count: 14 },
    { name: "img", count: 17 },
    { name: "code", count: 13 },
    { name: "zip", count: 20 },
    { name: "vids", count: 18 },
    { name: "apps", count: 16 },
  ];
  $: maxCount = Math.max(...categories.map(c => c.count));

  interface RecentFile { name: string; category: string; time: string; }
  let recentFiles: RecentFile[] = [
    { name: "class_x_marks.pdf", category: "docs", time: "2 min" },
    { name: "imagePNT200.png", category: "img", time: "5 min" },
    { name: "argonvs-dlor.zip", category: "zip", time: "1 day" },
    { name: "flatpak_app_final_v12.2.5", category: "apps", time: "2 mo" },
  ];

  function togglePause() { running = !running; }

  import { goto } from '$app/navigation';

  function handleSearch(e: KeyboardEvent) {
    if (e.key === "Enter" && query.trim()) {
      goto(`/search?q=${encodeURIComponent(query)}`);
    }
  }

  function organizeNow() { /* TODO: invoke Tauri command */ }
</script>

<div class="home">
  <div class="header">
    <div class="app-info">
      <img src="logo.png" alt="dlorg" class="icon" />
      <div>
        <h1>dlorg</h1>
        <span class="version">v1.0.0-beta</span>
      </div>
    </div>
    <div class="controls">
      <button class="pause-btn" title="{running ? "Pause" : "Start"} background service" on:click={togglePause}>
        <span class="material-symbols-outlined">
          {running ? "pause" : "play_arrow"}
        </span>
      </button>
      <span class="status-pill" title="Background process" class:running>
        <span class="status-dot"></span>
        {running ? "Running" : "Paused"}
      </span>
    </div>
  </div>

  <div class="search-bar">
    <span class="material-symbols-outlined search-icon">search</span>
    <input
      type="text"
      placeholder="Class 12th Physics Chapter 2 Electrostatics"
      bind:value={query}
      on:keydown={handleSearch}
    />
  </div>

  <div class="chips-row">
    {#each categories as cat}
      <div class="chip">
        <div class="bar-track">
          <div class="bar-fill" style="height: {(cat.count / maxCount) * 100}%"></div>
          <span class="chip-count">{cat.count}</span>
        </div>
        <span class="chip-label">{cat.name}</span>
      </div>
    {/each}
  </div>

  <div class="recent-files">
    <h2>Recent files</h2>
    <ul>
      {#each recentFiles as file}
        <li>
          <span class="time">{file.time}</span>
          <span class="filename">{file.name}</span>
          <span class="category">/{file.category}</span>
        </li>
      {/each}
    </ul>
  </div>

  <a href="/settings" class="settings-btn">
    <span class="material-symbols-outlined">settings</span>
  </a>

  <button class="organize-btn" title="Organize files now" on:click={organizeNow}>
    <span class="material-symbols-outlined">sync</span>
    Organize Now
  </button>
</div>

<style>
  .home { position: relative; min-height: 80vh; }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 24px;
  }
  .app-info { display: flex; align-items: center; gap: 12px; }
  .icon {
    width: 48px; height: 48px;
    border-radius: 12px;
    object-fit: cover;
  }
  h1 { margin: 0; font-size: 20px; color: #e5e5e5; }
  .version { color: #888; font-size: 12px; }

  .controls { display: flex; align-items: center; gap: 12px; }
  .pause-btn {
    background: #2a2a2a; border: none; color: #ffffff;
    width: 40px; height: 40px; border-radius: 50%;
    cursor: pointer;
    display: flex; align-items: center; justify-content: center;
  }
  .status-pill {
    display: flex; align-items: center; gap: 6px;
    height: 40px;
    padding: 0 16px;
    box-sizing: border-box;
    border-radius: 20px;
    background: #50472c; font-size: 13px; color: #fffbb4;
  }
  .status-pill.running { background: #4d5e1f; color: #c5e04a; }
  .status-dot {
    width: 8px; height: 8px; border-radius: 50%;
    background: currentColor;
  }

  .search-bar {
    display: flex; align-items: center; gap: 12px;
    background: #1e1e1e;
    border-radius: 24px;
    padding: 14px 20px;
    margin-bottom: 24px;
  }
  .search-icon { color: #777; }
  .search-bar input {
    background: transparent; border: none; outline: none;
    color: #e5e5e5; font-size: 15px; width: 100%;
  }
  .search-bar input::placeholder { color: #777; }

  .chips-row {
    display: flex;
    gap: 16px;
    overflow-x: auto;
    background: #202020;
    border-radius: 20px;
    padding: 24px 20px;
    margin-bottom: 24px;
  }
  .chip {
    display: flex; flex-direction: column; align-items: center; gap: 10px;
    min-width: 56px;
    flex-shrink: 0;
  }
  .bar-track {
    width: 60px;
    height: 120px;
    background: none;
    border-radius: 14px;
    display: flex;
    align-items: flex-end;
    justify-content: center;
    position: relative;
    overflow: hidden;
  }
  .bar-fill {
    width: 100%;
    background: #333333;
    border-radius: 18px;
    position: absolute;
    bottom: 0;
  }
  .chip-count {
    position: relative;
    background: #FFBDF7; color: #1a1a1a;
    font-weight: 700; font-size: 13px;
    padding: 3px 9px; border-radius: 12px;
    margin-bottom: 8px;
    z-index: 1;
  }
  .chip-label { 
    font-size: 13px; 
    color: #ccc; 
    font-weight: 600;
  }

  .recent-files {
    background: #1c1c1c;
    border-radius: 20px;
    padding: 20px;
    margin-bottom: 80px;
  }
  .recent-files h2 {
    font-size: 16px; 
    margin: 0 0 25px 0;
    color: #e5e5e5;
    text-decoration: underline wavy #FFBDF7;
  }
  .recent-files ul { list-style: none; padding: 0; margin: 0; }
  .recent-files li {
    display: flex; align-items: center; gap: 14px;
    background: #262626;
    border-radius: 10px;
    padding: 12px 16px;
    margin-bottom: 8px;
    font-size: 14px;
  }
  .time { color: #777; font-size: 12px; width: 40px; flex-shrink: 0; }
  .filename { color: #ddd; flex: 1; }
  .category { color: #FFBDF7; font-weight: 600; }

  .settings-btn {
    position: fixed;
    bottom: 24px; left: 24px;
    background: #2a2a2a;
    color: #aaa;
    width: 48px; height: 48px;
    border-radius: 50%;
    display: flex; align-items: center; justify-content: center;
    text-decoration: none;
  }


  .organize-btn {
    position: fixed;
    bottom: 24px; right: 24px;
    background: #5f003b;
    color:#FFBDF7;
    border: none;
    border-radius: 24px;
    padding: 14px 24px;
    font-weight: 600;
    display: flex; align-items: center; gap: 8px;
    cursor: pointer;
  }

  .organize-btn:hover {
    background: #6e0044;
  }

  .organize-btn:active {
    background: #540034;
  }

</style>