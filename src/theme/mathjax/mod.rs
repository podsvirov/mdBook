// An array of (file_name, file_contents) pairs
pub static FILES: [(&str, &[u8]); 29] = [
    ("LICENSE", include_bytes!("LICENSE")),
    (
        "es5/tex-mml-chtml.js",
        include_bytes!("es5/tex-mml-chtml.js"),
    ),
    ("es5/output/chtml.js", include_bytes!("es5/output/chtml.js")),
    ("es5/output/svg.js", include_bytes!("es5/output/svg.js")),
    (
        "es5/output/chtml/fonts/tex.js",
        include_bytes!("es5/output/chtml/fonts/tex.js"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_AMS-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_AMS-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Calligraphic-Bold.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Calligraphic-Bold.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Calligraphic-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Calligraphic-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Fraktur-Bold.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Fraktur-Bold.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Fraktur-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Fraktur-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Main-Bold.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Main-Bold.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Main-Italic.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Main-Italic.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Main-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Main-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Math-BoldItalic.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Math-BoldItalic.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Math-Italic.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Math-Italic.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Math-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Math-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_SansSerif-Bold.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_SansSerif-Bold.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_SansSerif-Italic.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_SansSerif-Italic.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_SansSerif-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_SansSerif-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Script-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Script-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Size1-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Size1-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Size2-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Size2-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Size3-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Size3-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Size4-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Size4-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Typewriter-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Typewriter-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Vector-Bold.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Vector-Bold.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Vector-Regular.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Vector-Regular.woff"),
    ),
    (
        "es5/output/chtml/fonts/woff-v2/MathJax_Zero.woff",
        include_bytes!("es5/output/chtml/fonts/woff-v2/MathJax_Zero.woff"),
    ),
    (
        "es5/output/svg/fonts/tex.js",
        include_bytes!("es5/output/svg/fonts/tex.js"),
    ),
];