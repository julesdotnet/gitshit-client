use std::fs;

pub struct Post {
    pub message: String,
    pub code: String,
    pub creator_id: i32,
    pub pg_lang: String,
}

impl Post {
    pub fn from(
        message: String,
        filepath: String,
        line_start: i32,
        line_end: i32,
        creator_id: i32,
    ) -> std::io::Result<Self> {
        let code = fs::read_to_string(&filepath)?;
        let lines: Vec<&str> = code.lines().collect();
        let start: usize = line_start.try_into().unwrap();
        let end: usize = line_end.try_into().unwrap();
        let end = end.min(lines.len().saturating_sub(1));
        let code_output = if start >= lines.len() || start > end {
            String::new()
        } else {
            lines[start..=end].join("\n")
        };

        let suffix = filepath.rsplit_once('.').map(|(_, ext)| ext).unwrap_or("");

        let pg_lang = match suffix {
            "java" => "Java",
            "rs"   => "Rust",
            "js"   => "JavaScript",
            "py"   => "Python",
            "c"    => "C",
            "cpp"  => "C++",
            "ts"   => "TypeScript",
            "zig"  => "Zig",
            "rb"   => "Ruby",
            "cs"   => "C#",
            _      => "Text",
        };

        Ok(Post {
            message,
            code: code_output,
            creator_id,
            pg_lang: pg_lang.to_string(),
        })
    }
}