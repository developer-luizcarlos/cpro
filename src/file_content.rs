pub fn get_html_content<'a>() -> &'a str {
    r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <meta charset="UTF-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1.0" />
        <title>Document</title>
        <link rel="stylesheet" href="./assets/styles/main.css" />
        <script type="module" src="./assets/scripts/main.js"></script>
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
