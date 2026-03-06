use std::collections::HashMap;

use crate::models::credit_format::CreditFormat;
use crate::models::grouping_type::GroupingType;
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

fn render_credit_by_author(authors: Vec<Author>, format: &CreditFormat) -> String {
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

fn render_credit_by_contribution(authors: Vec<Author>, format: &CreditFormat) -> String {
    let mut role_to_authors: HashMap<Roles, Vec<String>> = HashMap::new();

    for author in authors {
        if author.name.trim().is_empty() {
            continue;
        }

        let author_name = match format {
            CreditFormat::Long => author.name.clone(),
            CreditFormat::Short => author.short_name().unwrap_or(author.name.clone()),
        };

        for role in &author.roles {
            role_to_authors
                .entry(role.clone())
                .or_insert_with(Vec::new)
                .push(author_name.clone());
        }
    }

    if role_to_authors.is_empty() {
        return " ".to_string();
    }

    let mut roles_vec: Vec<(Roles, Vec<String>)> = role_to_authors.into_iter().collect();
    roles_vec.sort_by_key(|(role, _)| role.to_string());

    let mut result = String::new();

    for (role, author_names) in roles_vec {
        if author_names.is_empty() {
            continue;
        }

        let names_str = match author_names.len() {
            1 => author_names[0].clone(),
            2 => format!("{} and {}", author_names[0], author_names[1]),
            _ => {
                let last_idx = author_names.len() - 1;
                let before_last = &author_names[..last_idx];
                let last = &author_names[last_idx];
                format!("{} and {}", before_last.join(", "), last)
            }
        };

        result.push_str(&format!("{}: {}; ", role.to_string(), names_str));
    }

    if !result.is_empty() {
        result
    } else {
        " ".to_string()
    }
}

pub fn render_credit_statement(
    authors: Vec<Author>,
    format: &CreditFormat,
    grouping_type: &GroupingType,
) -> String {
    match grouping_type {
        GroupingType::ByAuthor => render_credit_by_author(authors, format),
        GroupingType::ByContrib => render_credit_by_contribution(authors, format),
    }
}
