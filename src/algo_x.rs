#[derive(Clone, Debug, Eq)]
pub struct MatrixNode {
    pub column: usize,
    pub row: usize,
}

impl MatrixNode {
    pub fn new(column: usize, row: usize) -> Self {
        Self { column, row }
    }
}

impl PartialEq for MatrixNode {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column && self.row == other.row
    }
}

#[derive(Clone, Debug, Eq)]
pub struct MatrixControlNode {
    pub column: usize,
}

impl PartialEq for MatrixControlNode {
    fn eq(&self, other: &Self) -> bool {
        self.column == other.column
    }
}

#[derive(Clone)]
pub struct Matrix {}

impl Matrix {
    // pub fn add_row_linked_list(&mut self, matrix_nodes: CircularDoublyLinkedList<MatrixNode>) {
    //     self.rows.push(matrix_nodes.clone());
    //     for matrix_node in matrix_nodes {
    //         match self.columns.find(MatrixControlNode {
    //             column: matrix_node.borrow().contents.column,
    //             rows: None,
    //         }) {
    //             None => {
    //                 let mut col_rows = CircularDoublyLinkedList::new();
    //                 col_rows.insert_tail(*matrix_node.borrow().clone().contents);
    //                 self.columns.insert_tail(MatrixControlNode {
    //                     column: matrix_node.borrow().contents.column,
    //                     rows: Some(col_rows.clone()),
    //                 });
    //             }
    //             Some(mcn) => {
    //                 let mut mcn = mcn.as_ref().borrow_mut();
    //                 mcn.contents
    //                     .rows
    //                     .as_mut()
    //                     .unwrap()
    //                     .insert_tail(*matrix_node.borrow().clone().contents);
    //             }
    //         }
    //     }
    // }

    // pub fn add_row(&mut self, matrix_nodes: Vec<MatrixNode>) {
    //     let mut row_ll: CircularDoublyLinkedList<MatrixNode> = CircularDoublyLinkedList::new();

    //     for matrix_node in matrix_nodes {
    //         row_ll.insert_tail(matrix_node.clone());

    //         match self.columns.find(MatrixControlNode {
    //             column: matrix_node.column,
    //             rows: None,
    //         }) {
    //             None => {
    //                 let mut col_rows = CircularDoublyLinkedList::new();
    //                 col_rows.insert_tail(matrix_node.clone());
    //                 self.columns.insert_tail(MatrixControlNode {
    //                     column: matrix_node.column,
    //                     rows: Some(col_rows.clone()),
    //                 });
    //             }
    //             Some(mcn) => {
    //                 let mut mcn = mcn.as_ref().borrow_mut();
    //                 mcn.contents.rows.as_mut().unwrap().insert_tail(matrix_node);
    //             }
    //         }
    //     }

    //     self.rows.push(row_ll);
    // }

    // pub fn remove_row_from_column(
    //     &mut self,
    //     row: usize,
    //     control_node: OptionalRef<DoublyLinkedListNode<MatrixControlNode>>,
    // ) {
    //     if let Some(control_node) = control_node {}
    // }

    // pub fn find_sparsest_column(&self) -> Option<MatrixControlNode> {
    //     let mut current_candidate_control_node: OptionalRef<
    //         DoublyLinkedListNode<MatrixControlNode>,
    //     > = None;
    //     let mut control_node = self.columns.head.clone();
    //     for _ in 0..self.columns.size {
    //         if let Some(inner) = &control_node {
    //             if let Some(inner_rows) = &inner.borrow().contents.rows {
    //                 let inner_rows_size = inner_rows.size;
    //                 match current_candidate_control_node {
    //                     None => current_candidate_control_node = Some(inner.clone()),
    //                     Some(ref cccn) => {
    //                         if cccn.borrow().contents.rows.as_ref().unwrap().size > inner_rows_size
    //                         {
    //                             println!("Using {}", inner.borrow().contents.column);
    //                             current_candidate_control_node = Some(inner.clone());
    //                         }
    //                     }
    //                 }
    //             }
    //             let ib = inner.clone();
    //             control_node = Some(ib.borrow().next.as_ref().unwrap().clone());
    //         }
    //     }

    //     match current_candidate_control_node {
    //         None => None,
    //         Some(n) => Some(*n.borrow().contents.clone()),
    //     }
    // }

    // pub fn remove_column(&mut self, column: usize) -> Option<MatrixControlNode> {
    //     if let Some(to_remove) = self.columns.find(MatrixControlNode { column, rows: None }) {
    //         let next_to_use = to_remove.borrow().next.clone();
    //         let prev_to_use = to_remove.borrow().prev.clone();

    //         if let Some(prev) = &to_remove.borrow().prev {
    //             prev.borrow_mut().set_next(next_to_use);
    //         }

    //         if let Some(next) = &to_remove.borrow().next {
    //             next.borrow_mut().set_prev(prev_to_use);
    //         }
    //     }

    //     None
    // }

    // pub fn restore_column(
    //     &mut self,
    //     control_node: OptionalRef<DoublyLinkedListNode<MatrixControlNode>>,
    // ) {
    //     if let Some(cn) = control_node {
    //         if let Some(prev) = &cn.borrow().prev {
    //             prev.borrow_mut().set_next(Some(cn.clone()));
    //         }

    //         if let Some(next) = &cn.borrow().next {
    //             next.borrow_mut().set_prev(Some(cn.clone()));
    //         }
    //     }
    // }
}
