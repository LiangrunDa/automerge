use crate::types::{ObjId, Op, OpId};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MoveManager {
    op_log: Vec<LogEntry>,
    tree: DocumentTree,
    cycle_safe_operation_map: HashMap<OpId, bool>,
    // we can't use HashSet for this
    winner_indicator: WinnerIndicator,
}

impl MoveManager {
    pub(crate) fn new() -> Self {
        Self {
            op_log: Vec::new(),
            tree: DocumentTree::new(),
            cycle_safe_operation_map: HashMap::new(),
            winner_indicator: WinnerIndicator::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct LogEntry {
    op: Op,
    parent_id: ObjId,
}

#[derive(Debug, Clone, PartialEq)]
struct DocumentTree {
    parent_map: HashMap<ObjId, Option<ObjId>>,
}

impl DocumentTree {
    pub(crate) fn new() -> Self {
        Self {
            parent_map: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct WinnerIndicator {
    obj_id_to_move_stack: HashMap<ObjId, Vec<OpId>>,
    winner_set: HashSet<OpId>,
}

impl WinnerIndicator {
    pub(crate) fn new() -> Self {
        Self {
            obj_id_to_move_stack: HashMap::new(),
            winner_set: HashSet::new(),
        }
    }
}
