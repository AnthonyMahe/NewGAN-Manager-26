<script>
    import { locale, setLocale } from '$lib/i18n';

    let isOpen = $state(false);

    const languages = [
        { code: 'en', flag: 'https://flagcdn.com/24x18/gb.png', label: 'EN' },
        { code: 'fr', flag: 'https://flagcdn.com/24x18/fr.png', label: 'FR' },
        { code: 'de', flag: 'https://flagcdn.com/24x18/de.png', label: 'DE' },
        { code: 'es', flag: 'https://flagcdn.com/24x18/es.png', label: 'ES' },
        { code: 'it', flag: 'https://flagcdn.com/24x18/it.png', label: 'IT' },
        { code: 'pt', flag: 'https://flagcdn.com/24x18/pt.png', label: 'PT' },
        { code: 'ko', flag: 'https://flagcdn.com/24x18/kr.png', label: 'KO' },
        { code: 'zh', flag: 'https://flagcdn.com/24x18/cn.png', label: 'ZH' },
        { code: 'uk', flag: 'https://flagcdn.com/24x18/ua.png', label: 'UA' }
    ];

    function getCurrentLang() {
        return languages.find(l => l.code === $locale) || languages[0];
    }

    function selectLanguage(code) {
        setLocale(code);
        isOpen = false;
    }

    function toggleDropdown(event) {
        event.stopPropagation();
        isOpen = !isOpen;
    }

    // Close dropdown when clicking outside
    function handleClickOutside(event) {
        isOpen = false;
    }
</script>

<svelte:window onclick={handleClickOutside} />

<div class="relative">
    <!-- Current Selection Button -->
    <button 
        onclick={toggleDropdown}
        class="flex items-center gap-2 bg-transparent border border-cyan-800/50 rounded px-3 py-1.5 hover:border-cyan-500/50 transition-colors cursor-pointer"
    >
        <img src={getCurrentLang().flag} alt="flag" class="w-6 h-4 object-cover rounded-sm" />
        <span class="text-cyan-300 font-bold uppercase tracking-wider">{getCurrentLang().label}</span>
        <span class="text-cyan-600 text-xs ml-1">▼</span>
    </button>

    <!-- Dropdown Menu -->
    {#if isOpen}
    <div 
        class="absolute top-full right-0 mt-1 bg-slate-900 border border-cyan-800/50 rounded shadow-lg shadow-cyan-900/20 overflow-hidden z-50 min-w-[100px]"
        onkeydown={(e) => e.key === 'Escape' && (isOpen = false)}
        role="menu"
        tabindex="-1"
    >
        {#each languages as lang}
            <button 
                onclick={() => selectLanguage(lang.code)}
                class="w-full flex items-center gap-2 px-3 py-2 hover:bg-cyan-900/30 transition-colors text-left {$locale === lang.code ? 'bg-cyan-900/20' : ''}"
            >
                <img src={lang.flag} alt={lang.label} class="w-6 h-4 object-cover rounded-sm" />
                <span class="text-cyan-300 font-bold uppercase tracking-wider">{lang.label}</span>
            </button>
        {/each}
    </div>
    {/if}
</div>
