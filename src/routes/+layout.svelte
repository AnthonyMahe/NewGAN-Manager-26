<script>
    import "../app.css";
    import { isLoading } from "$lib/i18n";
    import LanguageSwitcher from "$lib/components/LanguageSwitcher.svelte";
    import { t } from "$lib/i18n";

    let { children } = $props();
</script>

<div class="fixed inset-0 flex items-center justify-center p-4 bg-[#0a0a14]">
    <!-- Main App Container -->
    <div class="w-full max-w-5xl h-full max-h-[800px] flex flex-col bg-[var(--bg-panel)] rounded-2xl glow-border-cyan overflow-hidden relative">
        
        <!-- Header -->
        <header class="flex items-center justify-between p-4 border-b border-cyan-900/50 bg-[#0f0f18]">
            <div class="flex items-center gap-4">
                <div class="w-16 h-16 p-1 relative">
                     <!-- Logo Container with glow effect -->
                    <div class="absolute inset-0 bg-cyan-500/20 rounded-full blur-lg"></div>
                    <img src="/logo.png" alt="Logo" class="w-full h-full object-contain relative z-10 drop-shadow-[0_0_8px_rgba(0,255,255,0.5)]" />
                </div>
                <h1 class="text-3xl font-bold tracking-wider text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-cyan-200 glow-text-cyan uppercase">
                    NewGAN-Manager26
                </h1>
            </div>
            
            <div class="flex items-center gap-4">
                <LanguageSwitcher />
            </div>
        </header>

        <!-- Content -->
        <main class="flex-1 overflow-auto p-6 relative">
            {#if $isLoading}
            <div class="flex items-center justify-center h-full">
                <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-cyan-400"></div>
            </div>
            {:else}
                {@render children()}
            {/if}
        </main>
        
        <!-- Footer Credit -->
        <footer class="text-center py-2 text-xs text-gray-500 border-t border-gray-800/50">
            {$t('app.footer')} <a href="https://github.com/Maradonna90/NewGAN-Manager" target="_blank" class="text-cyan-600 hover:text-cyan-400 transition-colors">NewGAN-Manager</a> - by <a href="https://github.com/AnthonyMahe" target="_blank" class="text-cyan-600 hover:text-cyan-400 transition-colors">AnthonyMahe</a>
        </footer>

    </div>
</div>
