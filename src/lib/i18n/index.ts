import { writable, derived } from 'svelte/store';
import en from './en.json';
import fr from './fr.json';
import de from './de.json';
import es from './es.json';
import it from './it.json';
import ko from './ko.json';
import zh from './zh.json';
import uk from './uk.json';
import pt from './pt.json';
import da from './da.json';
import el from './el.json';
import nl from './nl.json';
import pl from './pl.json';
import sv from './sv.json';

const translations: Record<string, Record<string, any>> = {
    en, fr, de, es, it, ko, zh, uk, pt, da, el, nl, pl, sv
};

function getBrowserLocale(): string {
    if (typeof window === 'undefined') return 'en';
    const lang = window.navigator.language.split('-')[0];
    return translations[lang] ? lang : 'en';
}

export const locale = writable<string>(getBrowserLocale());
export const locales = Object.keys(translations);

// isLoading is always false since we load translations statically
export const isLoading = writable<boolean>(false);

export function setLocale(newLocale: string) {
    if (translations[newLocale]) {
        locale.set(newLocale);
    }
}

export const t = derived(locale, ($locale) => {
    return (key: string): string => {
        const keys = key.split('.');
        let value: any = translations[$locale];
        for (const k of keys) {
            value = value?.[k];
        }
        return value ?? key;
    };
});
