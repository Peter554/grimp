use crate::errors::{GrimpError, GrimpResult};
use crate::graph::{Graph, ModuleToken, EMPTY_MODULE_TOKENS};
use pathfinding::directed::bfs::bfs_bidirectional;
use pathfinding::NodeRefs;
use rustc_hash::{FxHashMap, FxHashSet};

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

    Ok(bfs_bidirectional(
        NodeRefs::from_iter(from_modules),
        NodeRefs::from_iter(to_modules),
        |module| {
            let mut next_modules = vec![];
            for candidate_next_module in graph.imports.get(*module).unwrap() {
                if excluded_modules.contains(candidate_next_module) {
                    continue;
                }
                if excluded_imports
                    .get(&module)
                    .unwrap_or(&EMPTY_MODULE_TOKENS)
                    .contains(candidate_next_module)
                {
                    continue;
                }
                next_modules.push(*candidate_next_module)
            }
            next_modules
        },
        |module| {
            let mut next_modules = vec![];
            for candidate_next_module in graph.reverse_imports.get(*module).unwrap() {
                if excluded_modules.contains(candidate_next_module) {
                    continue;
                }
                if excluded_imports
                    .get(candidate_next_module)
                    .unwrap_or(&EMPTY_MODULE_TOKENS)
                    .contains(module)
                {
                    continue;
                }
                next_modules.push(*candidate_next_module)
            }
            next_modules
        },
    ))
}
