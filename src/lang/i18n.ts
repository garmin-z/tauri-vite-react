import i18next, { Resource } from 'i18next';
import { initReactI18next } from 'react-i18next';

export const defaultNS = 'common';

const lfs = import.meta.glob('./locales/**/*.json', { eager: true });

const resources: Resource = {}
for (let path in lfs) {
    const [, , lang, file] = path.split("/");
    const [name] = file.split(".");
    if (!(lang in resources)) {
        resources[lang] = {}
    }
    Object.assign(resources[lang], { [name]: lfs[path] })
}

i18next.use(initReactI18next).init({
    lng: 'cn',
    debug: true,
    resources,
    defaultNS,
});