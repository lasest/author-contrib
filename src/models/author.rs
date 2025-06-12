use crate::models::credit_format::CreditFormat;
use crate::models::roles::Roles;

#[derive(Default, Clone, PartialEq)]
pub struct Author {
    pub name: String,
    pub roles: Vec<Roles>,
}

impl Author {
    pub fn update_role(&mut self, role: &Roles, is_present: bool) {
        if is_present {
            if !self.roles.contains(&role) {
                self.roles.push(role.clone());
            }
        } else {
            self.roles.retain(|r| r != role);
        }
    }

    pub fn short_name(&self) -> Option<String> {
        let words: Vec<&str> = self
            .name
            .split_whitespace()
            .filter(|w| !w.is_empty())
            .collect();

        if let (Some(first), Some(last)) = (words.first(), words.last()) {
            let first_char = first.chars().next()?;
            let last_char = last.chars().next()?;
            Some(format!("{}{}", first_char, last_char))
        } else {
            None
        }
    }

    pub fn roles_string(&self) -> String {
        let mut roles_vec: Vec<String> = vec![];

        for role in &self.roles {
            roles_vec.push(role.to_string());
        }

        roles_vec.sort();

        roles_vec.join(", ")
    }
}

pub fn render_credit_statement(authors: Vec<Author>, format: &CreditFormat) -> String {
    let mut result = String::new();

    for author in authors {
        if author.name.trim().is_empty() || author.roles_string().trim().is_empty() {
            continue;
        }

        match format {
            CreditFormat::Long => {
                result.push_str(&format!("{}: {}", &author.name, &author.roles_string()));
            }
            CreditFormat::Short => {
                let short_name = &author.short_name().unwrap_or(author.name.clone());
                result.push_str(&format!("{}: {}", short_name, &author.roles_string()));
            }
        };

        result.push_str("; ");
    }

    if !result.is_empty() {
        result
    } else {
        // Return single character to maitain element height
        " ".to_string()
    }
}
