pub struct Tokenizer {
    expr: String,
}

impl Tokenizer {
    pub fn from_string(expr: String) -> Self {
        Tokenizer { expr }
    }

    fn first(self: &Self) -> Option<String> {
        if let Some(ch) = self.expr.chars().next() {
            Some(ch.into())
        } else {
            None
        }
    }
}

impl Iterator for Tokenizer {
    type Item = String;

    /// get the next step in iteration
    fn next(&mut self) -> Option<Self::Item> {
        self.expr = self.expr.trim().to_string();

        let delimiters = [
            "(", ")", "{", "}", "[", "]", "\"", "'", " ", "\t", "\n", ",",
        ];

        if let Some(first) = self.first() {
            let token = if first == "\"" {
                format!("~{}~", self.expr.split("\"").nth(1).unwrap()).to_string()
            } else if delimiters.contains(&first.as_str()) {
                first
            } else {
                self.expr
                    .chars()
                    .take_while(|c| !delimiters.contains(&c.to_string().as_str()))
                    .collect::<String>()
            };

            self.expr = self.expr[token.len()..].to_string();
            Some(token)
        } else {
            None
        }
    }
}
