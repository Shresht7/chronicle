use std::path::Path;

use super::models::ApiNode;
use crate::models;

pub fn build_file_tree(files: Vec<models::FileMetadata>) -> ApiNode {
    let mut root_node = ApiNode {
        name: ".".to_string(),
        children: Vec::new(),
        size: None,
    };

    fn insert_into_tree(
        current_node: &mut ApiNode,
        path_components: &[&Path],
        file_bytes: Option<u64>,
    ) {
        if path_components.is_empty() {
            return;
        }

        let component = path_components[0];
        let is_last_component = path_components.len() == 1;
        let component_name = component.to_string_lossy().to_string();

        let mut found_child_index = None;
        for (i, child) in current_node.children.iter_mut().enumerate() {
            if child.name == component_name {
                found_child_index = Some(i);
                break;
            }
        }

        match found_child_index {
            Some(idx) => {
                let child = &mut current_node.children[idx];
                if is_last_component && file_bytes.is_some() {
                    child.size = file_bytes;
                }
                insert_into_tree(child, &path_components[1..], file_bytes);
            }
            None => {
                let mut new_child = ApiNode {
                    name: component_name,
                    children: Vec::new(),
                    size: if is_last_component { file_bytes } else { None },
                };
                insert_into_tree(&mut new_child, &path_components[1..], file_bytes);
                current_node.children.push(new_child);
            }
        }
    }

    let mut sorted_files = files;
    sorted_files.sort_by(|a, b| a.path.cmp(&b.path));

    for file_meta in sorted_files {
        let path_components: Vec<&Path> = file_meta.path.iter().map(Path::new).collect();
        insert_into_tree(&mut root_node, &path_components, Some(file_meta.bytes));
    }

    root_node
}
