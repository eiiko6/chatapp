import { FluentBundle, FluentResource } from '@fluent/bundle';
import { createFluentVue } from 'fluent-vue';

import enMessages from './locales/en.ftl?raw';
import frMessages from './locales/fr.ftl?raw';

const MESSAGES: Record<string, string> = {
  en: enMessages,
  fr: frMessages,
  // es: esMessages,
};

const BUNDLES: Record<string, FluentBundle> = {};

// Helper to create and cache bundles
function getBundle(locale: string): FluentBundle {
  if (BUNDLES[locale]) return BUNDLES[locale];

  const bundle = new FluentBundle(locale);
  const resource = new FluentResource(MESSAGES[locale] || MESSAGES['en']);
  bundle.addResource(resource);
  BUNDLES[locale] = bundle;
  return bundle;
}

const fallbackBundle = getBundle('en');

export const fluent = createFluentVue({
  bundles: [fallbackBundle]
});

/**
 * Matches input (e.g., 'fr-FR' or 'fr') against supported keys.
 */
export function setLanguage(localeCode: string) {
  // Extract base language (e.g., 'en' from 'en-US')
  const base = localeCode.split('-')[0].toLowerCase();

  // Check if supported
  const target = MESSAGES[localeCode] ? localeCode : (MESSAGES[base] ? base : 'en');

  const mainBundle = getBundle(target);
  fluent.bundles = [mainBundle, fallbackBundle];

  return target;
}

export const supportedLocales = Object.keys(MESSAGES);

/**
 * Iterates through supported locales and gets the value of the 'language' key
 * from each specific bundle.
 */
export function getSupportedLanguagesMetadata() {
  return supportedLocales.map(code => {
    const bundle = getBundle(code);
    const message = bundle.getMessage('language');

    let nativeName = code.toUpperCase();
    if (message && message.value) {
      nativeName = bundle.formatPattern(message.value);
    }

    return { code, name: nativeName };
  });
}
