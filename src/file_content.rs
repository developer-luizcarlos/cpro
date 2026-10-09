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
