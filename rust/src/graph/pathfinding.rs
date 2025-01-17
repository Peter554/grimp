use crate::errors::{GrimpError, GrimpResult};
use crate::graph::{Graph, ModuleToken, EMPTY_MODULE_TOKENS};
use indexmap::IndexMap;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use std::hash::BuildHasherDefault;

type FxIndexMap<K, V> = IndexMap<K, V, BuildHasherDefault<FxHasher>>;

pub fn find_shortest_path_bidirectional(
    graph: &Graph,
    from_modules: &FxHashSet<ModuleToken>,
    to_modules: &FxHashSet<ModuleToken>,
    excluded_modules: &FxHashSet<ModuleToken>,
    excluded_imports: &FxHashMap<ModuleToken, FxHashSet<ModuleToken>>,
) -> GrimpResult<Option<Vec<ModuleToken>>> {
    if !(from_modules & to_modules).is_empty() {
        return Err(GrimpError::SharedDescendants);
    }

    let mut predecessors: FxIndexMap<ModuleToken, Option<ModuleToken>> = from_modules
        .clone()
        .into_iter()
        .map(|m| (m, None))
        .collect();
    let mut successors: FxIndexMap<ModuleToken, Option<ModuleToken>> =
        to_modules.clone().into_iter().map(|m| (m, None)).collect();

    let mut i_forwards = 0;
    let mut i_backwards = 0;
    let middle = 'l: loop {
        for _ in 0..(predecessors.len() - i_forwards) {
            let module = *predecessors.get_index(i_forwards).unwrap().0;
            let next_modules = graph.imports.get(module).unwrap();
            for next_module in next_modules {
                if excluded_modules.contains(next_module) {
                    continue;
                }
                if excluded_imports
                    .get(&module)
                    .unwrap_or(&EMPTY_MODULE_TOKENS)
                    .contains(next_module)
                {
                    continue;
                }
                if !predecessors.contains_key(next_module) {
                    predecessors.insert(*next_module, Some(module));
                }
                if successors.contains_key(next_module) {
                    break 'l Some(*next_module);
                }
            }
            i_forwards += 1;
        }

        for _ in 0..(successors.len() - i_backwards) {
            let module = *successors.get_index(i_backwards).unwrap().0;
            let next_modules = graph.reverse_imports.get(module).unwrap();
            for next_module in next_modules {
                if excluded_modules.contains(next_module) {
                    continue;
                }
                if excluded_imports
                    .get(next_module)
                    .unwrap_or(&EMPTY_MODULE_TOKENS)
                    .contains(&module)
                {
                    continue;
                }
                if !successors.contains_key(next_module) {
                    successors.insert(*next_module, Some(module));
                }
                if predecessors.contains_key(next_module) {
                    break 'l Some(*next_module);
                }
            }
            i_backwards += 1;
        }

        if i_forwards == predecessors.len() && i_backwards == successors.len() {
            break 'l None;
        }
    };

    if middle.is_none() {
        return Ok(None);
    }
    let middle = middle.unwrap();

    // Path found!
    // Build the path.
    let mut path = vec![];
    let mut node = Some(middle);
    while let Some(n) = node {
        path.push(n);
        node = *predecessors.get(&n).unwrap();
    }
    path.reverse();
    let mut node = *successors.get(path.last().unwrap()).unwrap();
    while let Some(n) = node {
        path.push(n);
        node = *successors.get(&n).unwrap();
    }
    Ok(Some(path))
}
