<script>
  import { open } from '@tauri-apps/plugin-dialog';
  import { invoke } from '@tauri-apps/api/core';
  import { t } from '$lib/i18n';
  import { get } from 'svelte/store';

  let rtfPath = $state("");
  let imgPath = $state("");
  let mode = $state("generate"); // generate, preserve, overwrite
  let logs = $state([]);
  let isRunning = $state(false);

  // Modal state
  let showRtfHelp = $state(false);
  let showImgHelp = $state(false);

  // Progress
  let progress = $state(0);
  let totalFaces = $state(0);
  let mappedFaces = $state(0);

  // Validation
  let canGenerate = $derived(rtfPath && rtfPath.toLowerCase().endsWith('.rtf') && imgPath);

  function addLog(msg) {
      console.log('Adding log:', msg);
      logs.push(msg);
      logs = logs; // Force reactivity update
  }

  async function selectRtf() {
      const file = await open({
          multiple: false,
          filters: [{ name: 'RTF Files', extensions: ['rtf'] }]
      });
      if (file) rtfPath = file;
  }

  async function selectImgDir() {
      const dir = await open({
          directory: true,
          multiple: false
      });
      if (dir) imgPath = dir;
  }

  async function runNewgan() {
      const translate = get(t);
      
      // Check for missing inputs
      if (!rtfPath && !imgPath) {
          addLog(translate('ui.error_both'));
          return;
      }
      if (!rtfPath) {
          addLog(translate('ui.error_rtf'));
          return;
      }
      if (!imgPath) {
          addLog(translate('ui.error_img'));
          return;
      }
      
      // Validate RTF extension
      if (!rtfPath.toLowerCase().endsWith('.rtf')) {
          addLog(translate('ui.error_not_rtf'));
          return;
      }
      
      isRunning = true;
      addLog("Starting process in " + mode + " mode...");
      
      try {
          const res = await invoke('greet', { name: `Mode: ${mode}` });
          addLog(res);
          addLog("Process finished successfully.");
      } catch (err) {
          addLog("Error: " + err);
      } finally {
          isRunning = false;
      }
  }
</script>

<!-- Help Modal for RTF -->
{#if showRtfHelp}
<div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50" onclick={() => showRtfHelp = false} onkeydown={(e) => e.key === 'Escape' && (showRtfHelp = false)} role="dialog" aria-modal="true" tabindex="-1">
    <div class="bg-slate-900 border border-cyan-700 rounded-lg p-6 max-w-md mx-4 shadow-xl shadow-cyan-900/30">
        <h3 class="text-xl font-bold text-cyan-400 mb-4">{$t('help.rtf_title')}</h3>
        <p class="text-gray-300 leading-relaxed">{$t('help.rtf_text')}</p>
        <button onclick={() => showRtfHelp = false} class="mt-4 px-4 py-2 bg-cyan-700 hover:bg-cyan-600 rounded text-white text-sm transition-colors">
            {$t('ui.close')}
        </button>
    </div>
</div>
{/if}

<!-- Help Modal for Image Directory -->
{#if showImgHelp}
<div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50" onclick={() => showImgHelp = false} onkeydown={(e) => e.key === 'Escape' && (showImgHelp = false)} role="dialog" aria-modal="true" tabindex="-1">
    <div class="bg-slate-900 border border-cyan-700 rounded-lg p-6 max-w-md mx-4 shadow-xl shadow-cyan-900/30">
        <h3 class="text-xl font-bold text-cyan-400 mb-4">{$t('help.img_title')}</h3>
        <p class="text-gray-300 leading-relaxed">{$t('help.img_text')}</p>
        <button onclick={() => showImgHelp = false} class="mt-4 px-4 py-2 bg-cyan-700 hover:bg-cyan-600 rounded text-white text-sm transition-colors">
            {$t('ui.close')}
        </button>
    </div>
</div>
{/if}

<div class="flex flex-col gap-6 h-full">

  <!-- Inputs Section -->
  <div class="flex flex-col gap-4">
      <!-- RTF Input -->
      <div class="flex gap-4 items-center">
          <button onclick={() => showRtfHelp = true} class="w-8 h-8 flex items-center justify-center rounded-full bg-cyan-900/50 border border-cyan-700 text-cyan-400 hover:bg-cyan-800/50 transition-colors text-sm font-bold" title="Help">
              i
          </button>
          <input 
              type="text" 
              bind:value={rtfPath} 
              placeholder="C:/Users/FM/export.rtf"
              readonly
              class="flex-1 bg-[#0f0f18] border border-cyan-800 rounded px-4 py-3 text-cyan-300 placeholder-cyan-900/50 focus:outline-none focus:border-cyan-500 transition-colors shadow-inner"
          />
          <button onclick={selectRtf} class="neon-btn text-cyan-300 w-48 rounded font-semibold text-sm uppercase tracking-wider hover:text-white">
              {$t('ui.select_file')}
          </button>
      </div>

      <!-- Image Dir Input -->
      <div class="flex gap-4 items-center">
          <button onclick={() => showImgHelp = true} class="w-8 h-8 flex items-center justify-center rounded-full bg-cyan-900/50 border border-cyan-700 text-cyan-400 hover:bg-cyan-800/50 transition-colors text-sm font-bold" title="Help">
              i
          </button>
          <input 
              type="text" 
              bind:value={imgPath} 
              placeholder="D:/Faces/Sorted/"
              readonly
              class="flex-1 bg-[#0f0f18] border border-cyan-800 rounded px-4 py-3 text-cyan-300 placeholder-cyan-900/50 focus:outline-none focus:border-cyan-500 transition-colors shadow-inner"
          />
          <button onclick={selectImgDir} class="neon-btn text-cyan-300 w-48 rounded font-semibold text-sm uppercase tracking-wider hover:text-white">
              {$t('ui.select_folder')}
          </button>
      </div>
  </div>

  <!-- Mode Selection -->
  <div class="grid grid-cols-3 gap-2">
      <div class="flex flex-col">
          <button 
              class="py-3 rounded-t transition-all font-semibold uppercase tracking-widest text-sm {mode === 'generate' ? 'bg-purple-900/50 text-white shadow-[0_0_10px_rgba(168,85,247,0.5)] border border-purple-500' : 'bg-[#0f0f18] text-gray-500 hover:text-cyan-400 border border-cyan-900/30'}"
              onclick={() => mode = 'generate'}
          >
              [ {$t('mode.generate')} ]
          </button>
          <div class="bg-[#0a0a12] border border-t-0 border-cyan-900/30 rounded-b px-2 py-2 text-[10px] text-gray-500 leading-tight">
              {$t('mode.generate_desc')}
          </div>
      </div>
      <div class="flex flex-col">
          <button 
              class="py-3 rounded-t transition-all font-semibold uppercase tracking-widest text-sm {mode === 'preserve' ? 'bg-purple-900/50 text-white shadow-[0_0_10px_rgba(168,85,247,0.5)] border border-purple-500' : 'bg-[#0f0f18] text-gray-500 hover:text-cyan-400 border border-cyan-900/30'}"
              onclick={() => mode = 'preserve'}
          >
              [ {$t('mode.preserve')} ]
          </button>
          <div class="bg-[#0a0a12] border border-t-0 border-cyan-900/30 rounded-b px-2 py-2 text-[10px] text-gray-500 leading-tight">
              {$t('mode.preserve_desc')}
          </div>
      </div>
      <div class="flex flex-col">
          <button 
              class="py-3 rounded-t transition-all font-semibold uppercase tracking-widest text-sm {mode === 'overwrite' ? 'bg-purple-900/50 text-white shadow-[0_0_10px_rgba(168,85,247,0.5)] border border-purple-500' : 'bg-[#0f0f18] text-gray-500 hover:text-cyan-400 border border-cyan-900/30'}"
              onclick={() => mode = 'overwrite'}
          >
              [ {$t('mode.overwrite')} ]
          </button>
          <div class="bg-[#0a0a12] border border-t-0 border-cyan-900/30 rounded-b px-2 py-2 text-[10px] text-gray-500 leading-tight">
              {$t('mode.overwrite_desc')}
          </div>
      </div>
  </div>

  <!-- Generate Button -->
  <button 
      onclick={runNewgan} 
      disabled={isRunning || !canGenerate}
      class="neon-btn-primary w-full py-4 rounded font-bold text-xl uppercase tracking-[0.2em] transform active:scale-[0.99] transition-transform disabled:opacity-50 disabled:cursor-not-allowed"
  >
      {isRunning ? $t('ui.processing') : $t('ui.generate_btn')}
  </button>

  <!-- Progress Section -->
  <div class="flex flex-col gap-2">
      <div class="flex justify-end text-cyan-400 text-sm font-mono mb-1">
          {$t('logs.mapped')} {mappedFaces.toLocaleString()} / {totalFaces > 0 ? totalFaces.toLocaleString() : '50,000'}
      </div>
      <div class="h-2 bg-gray-800 rounded-full overflow-hidden border border-gray-700">
          <div 
              class="h-full bg-cyan-400 shadow-[0_0_10px_#0ff] transition-all duration-300" 
              style="width: {progress}%"
          ></div>
      </div>
  </div>

  <!-- Logs Console -->
  <div class="flex-1 bg-[#050510] border border-gray-800 rounded p-4 font-mono text-sm overflow-hidden flex flex-col">
      <div class="overflow-y-auto h-full space-y-1 pr-2">
          <div class="text-gray-500">[INFO] {$t('logs.ready')}</div>
          {#each logs as log}
              <div class="text-cyan-100 border-l-2 border-cyan-500 pl-2">
                  <span class="text-cyan-600 mr-2">[{new Date().toLocaleTimeString()}]</span>
                  {log}
              </div>
          {/each}
      </div>
  </div>

</div>
