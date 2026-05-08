use std::fs;
use std::path::Path;

#[test]
fn skill_files_have_required_shape() {
    let skill_files = skill_files(Path::new("skills"));
    assert!(!skill_files.is_empty(), "expected at least one skill file");

    for skill_file in skill_files {
        let text = fs::read_to_string(&skill_file).expect("skill file should be readable");
        assert!(
            text.starts_with("---\n"),
            "{} missing YAML frontmatter",
            skill_file.display()
        );
        assert!(
            text.contains("\nname: "),
            "{} missing name frontmatter",
            skill_file.display()
        );
        assert!(
            text.contains("\ndescription: "),
            "{} missing description frontmatter",
            skill_file.display()
        );
        for section in [
            "## Purpose",
            "## When to use",
            "## Key behaviors",
            "## Discovering flags",
            "## Errors",
        ] {
            assert!(
                text.contains(section),
                "{} missing {section}",
                skill_file.display()
            );
        }
    }
}

fn skill_files(directory: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(directory).expect("skills directory should exist") {
        let entry = entry.expect("directory entry should be readable");
        let path = entry.path();
        if path.is_dir() {
            files.extend(skill_files(&path));
        } else if path.file_name().and_then(|name| name.to_str()) == Some("SKILL.md") {
            files.push(path);
        }
    }
    files
}
