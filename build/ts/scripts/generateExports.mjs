import { readdirSync, writeFileSync } from "fs";
import { resolve } from "path";

const files = {}

readdirSync(resolve('./pb/_generated')).forEach((file) => {
    if (!file.endsWith('.js') && !file.endsWith('.ts')) return;

    const category = file.split('.')[0];
    files[category] ||= [];
    files[category].push(file);
});

Object.entries(files).forEach(([category, categoryFiles]) => {
    const fileContent = categoryFiles.map(file => `export * from "./_generated/${file.replace('.ts', '.js')}";`).join('\n');
    writeFileSync(`./pb/${category}.ts`, `${fileContent}\n`);
});
