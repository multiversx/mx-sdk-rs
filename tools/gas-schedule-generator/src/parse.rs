pub(crate) fn parse_toml_sections(content: &str) -> Vec<(String, Vec<(String, String)>)> {
    let mut sections: Vec<(String, Vec<(String, String)>)> = Vec::new();
    let mut current_section = String::new();
    let mut current_entries: Vec<(String, String)> = Vec::new();

    for line in content.lines() {
        let trimmed_line = line.trim();

        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        if trimmed_line.starts_with('[') && trimmed_line.ends_with(']') {
            // previous section with entries, add it to sections list
            if !current_section.is_empty() && current_section != "WASMOpcodeCost" {
                sections.push((current_section.clone(), current_entries));
                current_entries = Vec::new();
            }

            current_section = trimmed_line[1..trimmed_line.len() - 1].to_string();
            println!("Found section {current_section:?}");

            // skip WASMOpcodeCost section
            if current_section == "WASMOpcodeCost" {
                current_section = String::new(); // reset to avoid collecting entries
            }
        } else if !current_section.is_empty() && trimmed_line.contains('=') {
            let parts: Vec<&str> = trimmed_line.splitn(2, '=').collect();
            if parts.len() == 2 {
                let key = parts[0].trim().to_string();
                let value = parts[1].trim().to_string();
                current_entries.push((key, value));
            }
        }
    }

    // last section
    if !current_section.is_empty() && current_section != "WASMOpcodeCost" {
        sections.push((current_section, current_entries));
    }

    sections
}
