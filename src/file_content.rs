pub fn get_html_content<'a>() -> &'a str {
    r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Document</title>
        <link rel="stylesheet" href="./src/styles/main.css" />
        <script type="module" src="./dist/bundle.js"></script>
    </head>
    <body></body>
</html>
    "#
}

pub fn get_css_content<'a>() -> &'a str {
    r#"* {
    box-sizing: border-box;
    margin: 0;
    padding: 0;
}
    
body,
html {
    font-family: Arial, Helvetica, sans-serif;
    height: 100vh;
    width: 100vw;
}
        "#
}

pub fn get_js_content<'a>() -> &'a str {
    r#"console.log("Rust made all this!");"#
}

pub fn get_tsconfig_content<'a>() -> &'a str {
    r#"{
"compilerOptions": {
    "allowArbitraryExtensions": true,
    "allowImportingTsExtensions": true,
    "allowJs": true,
    "allowSyntheticDefaultImports": true,
    "allowUnreachableCode": false,
    "allowUnusedLabels": false,
    "alwaysStrict": true,
    "checkJs": true,
    "declaration": true,
    "declarationMap": true,
    "emitDeclarationOnly": true,
    "esModuleInterop": true,
    "exactOptionalPropertyTypes": true,
    "extendedDiagnostics": true,
    "forceConsistentCasingInFileNames": true,
    "inlineSourceMap": true,
    "inlineSources": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "noFallthroughCasesInSwitch": true,
    "noImplicitAny": true,
    "noImplicitOverride": true,
    "noImplicitReturns": true,
    "noImplicitThis": true,
    "noPropertyAccessFromIndexSignature": true,
    "noUncheckedSideEffectImports": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "outDir": "./dist/",
    "removeComments": true,
    "resolveJsonModule": true,
    "rootDir": "./src/scripts",
    "skipLibCheck": true,
    "strictBindCallApply": true,
    "strictFunctionTypes": true,
    "strictNullChecks": true,
    "strictPropertyInitialization": true,
    "target": "esnext"
},
"include": [
    "src/**/*"
    ]
}"#
}

pub fn get_webpack_content_ts<'a>() -> &'a str {
    r#"// @ts-ignore
import path from "node:path";
// @ts-ignore
import { fileURLToPath } from "node:url";
import webpack from "webpack";
    
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
    
const config: webpack.Configuration = {
    mode: "production",
    entry: "./src/scripts/main.ts",
    output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
    },
};
    
export default config;"#
}

pub fn get_webpack_content_js<'a>() -> &'a str {
    r#"import path from "node:path";
import { fileURLToPath } from "node:url";
    
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
    
export default {
    mode: "development",
    entry: "./src/scripts/main.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bundle.js",
    },
};"#
}
