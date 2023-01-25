use super::RelevantDirectory;
use std::{
    collections::BTreeMap,
    path::{Component, Path},
};

const PIPE_L: &str = " └─";
const PIPE_T: &str = " ├─";
const INDENT_PIPE: &str = " │ ";
const INDENT_SPACE: &str = "   ";

struct PrettyPrintTreeNode {
    name: String,
    dir: Option<RelevantDirectory>,
    children: BTreeMap<String, PrettyPrintTreeNode>,
}

impl PrettyPrintTreeNode {
    fn new(name: String) -> Self {
        PrettyPrintTreeNode {
            name,
            dir: None,
            children: BTreeMap::new(),
        }
    }

    fn add_path(&mut self, path: &Path, dir: &RelevantDirectory) {
        let components: Vec<Component> = path
            .components()
            .filter(|component| component.as_os_str().to_string_lossy() != "/")
            .collect();
        self.add_components(&components[..], dir);
    }

    fn add_components(&mut self, components: &[Component], dir: &RelevantDirectory) {
        if components.is_empty() {
            return;
        }

        let first = components[0].as_os_str().to_string_lossy().into_owned();
        let child = self
            .children
            .entry(first.to_string())
            .or_insert_with(|| PrettyPrintTreeNode::new(first));

        let remaining_components = &components[1..];
        if remaining_components.is_empty() {
            child.dir = Some(dir.clone());
        } else {
            child.add_components(remaining_components, dir);
        }
    }

    fn coalesce_single_children(&mut self) {
        for child in self.children.values_mut() {
            child.coalesce_single_children();
        }

        if self.children.len() == 1 {
            let only_child = self.children.first_entry().unwrap().remove();
            self.name = format!("{}/{}", &self.name, &only_child.name);
            self.children = only_child.children;
        }
    }

    fn print<PrintName>(&self, prefix: &str, child_prefix: &str, print_name: &PrintName)
    where
        PrintName: Fn(&RelevantDirectory),
    {
        let num_children = self.children.len();
        print!("{prefix} {}", &self.name);
        if let Some(dir) = &self.dir {
            print_name(dir);
        }
        println!();

        for (i, child) in self.children.values().enumerate() {
            let (l_pipe, vertical_pipe) = if i == num_children - 1 {
                (PIPE_L, INDENT_SPACE)
            } else {
                (PIPE_T, INDENT_PIPE)
            };

            let next_prefix = format!("{child_prefix}{l_pipe}");
            let next_child_prefix = format!("{child_prefix}{vertical_pipe}"); // or grandchild prefix
            child.print(next_prefix.as_str(), next_child_prefix.as_str(), print_name);
        }
    }
}

pub fn dir_pretty_print<'a, I, PrintName>(dir_iter: I, prefix: &str, print_name: &PrintName)
where
    I: Iterator<Item = &'a RelevantDirectory>,
    PrintName: Fn(&RelevantDirectory),
{
    let mut root = PrettyPrintTreeNode::new("".to_string());
    for dir in dir_iter {
        root.add_path(dir.path.as_ref(), dir);
    }
    root.coalesce_single_children();

    root.print(prefix, prefix, print_name);
}
