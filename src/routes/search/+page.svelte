<script lang="ts">
  import { page } from '$app/stores';

  $: query = $page.url.searchParams.get('q') ?? '';

  interface SearchResult {
    title: string;
    path: string;
    ext: string;
  }

  // placeholder — will be replaced by invoke('semantic_search', { query })
  let results: SearchResult[] = [
    { title: "Class X Marksheet", path: "/home/a/Download/Documents/Exams/class_x_marksheet.pdf", ext: "PDF" },
    { title: "Student marks", path: "/home/a/Download/Documents/Exams/student_marks.xlsx", ext: "XLS" },
    { title: "Class XII Marksheet", path: "/home/a/Download/Documents/Exams/class_12.doc", ext: "DOC" },
  ];

  function extIcon(ext: string): string {
    const map: Record<string, string> = {
      PDF: "picture_as_pdf",
      XLS: "table_chart",
      DOC: "description",
      ZIP: "folder_zip",
      PNG: "image", JPG: "image",
      MP4: "movie",
    };
    return map[ext] ?? "draft";
  }
</script>

<div class="search-page">
  <a href="/" class="back-btn">
    <span class="material-symbols-outlined">arrow_back</span>
  </a>

  <div class="search-bar">
    <span class="material-symbols-outlined search-icon">search</span>
    <input type="text" value={query} placeholder="Class 12th Marksheet" readonly />
  </div>

  {#if results.length}
    <div class="results">
      {#each results as result}
        <div class="result">
          <div class="result-icon">
            <span class="material-symbols-outlined">{extIcon(result.ext)}</span>
          </div>
          <div class="result-text">
            <div class="result-title">{result.title}</div>
            <div class="result-path">{result.path}</div>
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <span class="material-symbols-outlined">search_off</span>
      <p>No matches found</p>
    </div>
  {/if}
</div>

<style>
  .search-page { padding-bottom: 40px; }

  .back-btn {
    display: inline-flex; align-items: center; justify-content: center;
    width: 40px; height: 40px;
    background: #2a2a2a;
    border-radius: 50%;
    color: #ccc;
    text-decoration: none;
    margin-bottom: 20px;
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

  .results { display: flex; flex-direction: column; gap: 12px; }
  .result {
    display: flex; align-items: center; gap: 16px;
    background: #1c1c1c;
    border-radius: 16px;
    padding: 16px 20px;
  }
  .result-icon {
    width: 44px; height: 44px;
    border-radius: 10px;
    background: #2a2a2a;
    display: flex; align-items: center; justify-content: center;
    color: #d94fb0;
    flex-shrink: 0;
  }
  .result-title { font-weight: 600; margin-bottom: 4px; color: #e5e5e5; }
  .result-path { color: #888; font-size: 13px; }

  .empty-state {
    display: flex; flex-direction: column; align-items: center; gap: 12px;
    padding: 60px 0;
    color: #666;
  }
  .empty-state .material-symbols-outlined { font-size: 40px; }
</style>