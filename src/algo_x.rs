use std::collections::HashMap;

#[derive(Clone)]
pub struct Matrix {
    nodes: Vec<(u32, u32)>,
    removed_cols: Vec<u32>,
    removed_rows: Vec<u32>,
    col_length_map: HashMap<u32, u32>,
    pub solution_rows: Vec<u32>,
}

impl Matrix {
    pub fn new(nodes: Vec<(u32, u32)>) -> Self {
        let mut col_length_map: HashMap<u32, u32> = HashMap::new();
        let mut num_cols: u32 = 0;
        for (col, _) in &nodes {
            num_cols = u32::max(*col, num_cols);
            let col_length = col_length_map.get(col).unwrap_or(&0) + 1;
            col_length_map.insert(*col, col_length);
        }

        Self {
            nodes,
            removed_cols: vec![],
            removed_rows: vec![],
            col_length_map,
            solution_rows: vec![],
        }
    }

    pub fn get_nodes_in_col(&self, col: u32) -> Vec<(u32, u32)> {
        let mut nodes: Vec<(u32, u32)> = vec![];
        for (c, r) in &self.nodes {
            if !self.removed_cols.contains(c) && !self.removed_rows.contains(r) && c == &col {
                nodes.push((*c, *r));
            }
        }
        nodes
    }

    pub fn remove_col(&mut self, col: u32) {
        if self.removed_cols.contains(&col) {
            return;
        }
        println!("x Removing col {}", col);
        self.removed_cols.push(col);
    }

    pub fn get_nodes_in_row(&self, row: u32) -> Vec<(u32, u32)> {
        let mut nodes: Vec<(u32, u32)> = vec![];
        for (c, r) in &self.nodes {
            if !self.removed_rows.contains(r) && !self.removed_cols.contains(c) && r == &row {
                nodes.push((*c, *r));
            }
        }
        nodes
    }

    pub fn remove_row(&mut self, row: u32) {
        if self.removed_rows.contains(&row) {
            return;
        }
        println!("x Removing row {}", row);
        self.removed_rows.push(row);

        for (c, r) in &self.nodes {
            if row == *r {
                let new_col_length = self.col_length_map.get(c).unwrap_or(&1) - 1;
                self.col_length_map.insert(*c, new_col_length);
            }
        }
    }

    pub fn get_sparsest_col(&self) -> Option<(u32, u32)> {
        let mut sparsest: Option<(&u32, &u32)> = None;
        let mut keys: Vec<&u32> = self.col_length_map.keys().collect::<Vec<&u32>>();
        keys.sort();
        for current_col in keys {
            if self.removed_cols.contains(current_col) {
                continue;
            }
            println!("Checking {}", current_col);

            let current_len = self.col_length_map.get(current_col).unwrap();
            println!("-> Length {}", current_len);
            if sparsest.is_none() {
                sparsest = Some((current_col, current_len));
            } else if let Some(s) = sparsest {
                if s.1 > current_len {
                    sparsest = Some((current_col, current_len));
                }
            }
        }

        match sparsest {
            None => None,
            Some((c, l)) => Some((*c, *l)),
        }
    }

    pub fn solve(&mut self) -> bool {
        println!(
            "\nSOLVE\nself: {:?}, {:?}, {:?}",
            self.solution_rows, self.removed_cols, self.removed_rows
        );
        let sparsest_col = self.get_sparsest_col();
        println!("Choosing col {:?}", sparsest_col);
        match sparsest_col {
            None => true,
            Some((sc, sl)) => {
                if sl == 0 {
                    println!("Sparsest column is 0, failing");
                    return false;
                } else {
                    let potential_rows = self.get_nodes_in_col(sc);
                    for (_, row) in potential_rows {
                        println!("Adding row {} to solution", row);
                        let mut child_clone = self.clone();
                        child_clone.solution_rows.push(row);
                        for (col, _) in child_clone.get_nodes_in_row(row) {
                            for (_, r) in child_clone.get_nodes_in_col(col) {
                                child_clone.remove_row(r);
                            }
                            child_clone.remove_col(col);
                        }
                        child_clone.remove_row(row);
                        if child_clone.solve() {
                            self.solution_rows = child_clone.solution_rows;
                            return true;
                        } else {
                            println!("Next candidate");
                        }
                    }
                    false
                }
            }
        }
    }
}
