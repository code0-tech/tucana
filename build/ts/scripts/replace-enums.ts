import { Project } from "ts-morph";

const project = new Project({
    tsConfigFilePath: "tsconfig.json",
});

const SOURCE_DIR = "pb/_generated";

project.addSourceFilesAtPaths(`${SOURCE_DIR}/**/*.ts`);

for (const sourceFile of project.getSourceFiles()) {
    const isDts = sourceFile.isDeclarationFile();
    const enums = sourceFile.getEnums();

    for (const enumDecl of enums) {
        const name = enumDecl.getName();
        const isExported = enumDecl.isExported();

        const members = enumDecl.getMembers().map((member) => {
            const memberName = member.getName();
            const value = member.getValue();

            return {
                name: memberName,
                value: value ?? `"${memberName}"`,
            };
        });

        let replacement = "";

        if (isDts) {
            replacement += `
${isExported ? "export " : ""}declare const ${name}: {
${members.map(m => `  readonly ${m.name}: ${m.value};`).join("\n")}
};
`;

            replacement += `
${isExported ? "export " : ""}type ${name} = typeof ${name}[keyof typeof ${name}];
`;
        } else {
            replacement += `
${isExported ? "export " : ""}const ${name} = {
${members.map(m => `  ${m.name}: ${m.value}`).join(",\n")}
} as const;
`;

            replacement += `
${isExported ? "export " : ""}type ${name} = typeof ${name}[keyof typeof ${name}];
`;
        }

        enumDecl.replaceWithText(replacement);
    }
}

project.saveSync();
