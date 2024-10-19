use std::{cell::RefCell, rc::Rc};

type MatrixControlNodeValue = u8;

pub struct MatrixControlNode {
    pub column: MatrixControlNodeValue,
    pub num_rows: u8,

    pub prev_control_node: Option<Rc<RefCell<MatrixControlNode>>>,
    pub next_control_node: Option<Rc<RefCell<MatrixControlNode>>>,

    pub prev_node: Option<Rc<RefCell<MatrixNode>>>,
    pub next_node: Option<Rc<RefCell<MatrixNode>>>,
}

impl MatrixControlNode {
    pub fn new(column: u8) -> Self {
        Self {
            column,
            num_rows: 0,
            prev_control_node: None,
            next_control_node: None,
            prev_node: None,
            next_node: None,
        }
    }

    pub fn find_node(&self, node_value: MatrixNodeValue) -> Option<Rc<RefCell<MatrixNode>>> {
        match self.next_node.clone() {
            None => None,
            Some(n) => {
                let mut current_node = Some(n);
                for _ in 0..self.num_rows {
                    if let Some(cn) = current_node {
                        if cn.borrow().location == node_value {
                            return Some(cn.clone());
                        }
                        current_node = cn.borrow().next_row.clone();
                    }
                }
                return None;
            }
        }
    }

    pub fn add_node(&mut self, node: Rc<RefCell<MatrixNode>>) {
        if let Some(pn) = &self.prev_node {
            pn.borrow_mut().next_row = Some(node.clone());
            self.prev_node = Some(node.clone());
        } else {
            self.next_node = Some(node.clone());
            self.prev_node = Some(node.clone());
        }
        self.num_rows += 1;
    }
}

type MatrixNodeValue = (u8, u8);

pub struct MatrixNode {
    pub location: MatrixNodeValue,

    pub control_node: Option<Rc<RefCell<MatrixControlNode>>>,

    pub prev_row: Option<Rc<RefCell<MatrixNode>>>,
    pub next_row: Option<Rc<RefCell<MatrixNode>>>,

    pub prev_col: Option<Rc<RefCell<MatrixNode>>>,
    pub next_col: Option<Rc<RefCell<MatrixNode>>>,
}

impl PartialEq for MatrixNode {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}

impl Eq for MatrixNode {}

impl MatrixNode {
    pub fn new(location: (u8, u8)) -> Rc<RefCell<MatrixNode>> {
        let s = MatrixNode {
            location,
            control_node: None,

            prev_row: None,
            next_row: None,

            prev_col: None,
            next_col: None,
        };

        Rc::new(RefCell::new(s))
    }
}

pub struct Matrix {
    pub root_control_node: Option<Rc<RefCell<MatrixControlNode>>>,
    pub num_columns: u8,
}

impl Matrix {
    pub fn new() -> Self {
        Self {
            root_control_node: None,
            num_columns: 0,
        }
    }

    pub fn add_node(&mut self, node: Rc<RefCell<MatrixNode>>) {
        let mut borrowed_node = node.borrow_mut();
        let control_node = self.find_control_node(borrowed_node.location.0);

        match control_node {
            None => {
                let mut new_control_node = MatrixControlNode::new(borrowed_node.location.0);
                println!("Adding new control node {}", new_control_node.column);
                new_control_node.add_node(node.clone());
                let ncn = Rc::new(RefCell::new(new_control_node));
                self.add_control_node(ncn.clone());
                borrowed_node.control_node = Some(ncn.clone());
            }
            Some(cn) => {
                println!("Using existing control node {}", cn.borrow().column);
                cn.borrow_mut().add_node(node.clone());
                borrowed_node.control_node = Some(cn.clone());
            }
        }
    }

    pub fn add_control_node(&mut self, control_node: Rc<RefCell<MatrixControlNode>>) {
        if let Some(rcn) = self.root_control_node.clone() {
            let last_control_node = rcn.borrow().prev_control_node.clone();
            if let Some(lcn) = last_control_node {
                lcn.borrow_mut().next_control_node = Some(control_node.clone());
                rcn.borrow_mut().prev_control_node = Some(control_node.clone());
            } else {
                rcn.borrow_mut().next_control_node = Some(control_node.clone());
                rcn.borrow_mut().prev_control_node = Some(control_node.clone());
            }
        } else {
            self.root_control_node = Some(control_node.clone());
        }
        self.num_columns += 1;
    }

    pub fn find_control_node(&self, column: u8) -> Option<Rc<RefCell<MatrixControlNode>>> {
        match self.root_control_node.clone() {
            None => None,
            Some(n) => {
                let mut current_node = Some(n);
                for _ in 0..self.num_columns {
                    if let Some(cn) = current_node {
                        if cn.borrow().column == column {
                            return Some(cn.clone());
                        }
                        current_node = cn.borrow().next_control_node.clone();
                    }
                }
                return None;
            }
        }
    }

    pub fn find_sparsest_column(&self) -> Option<Rc<RefCell<MatrixControlNode>>> {
        let mut current_lowest_control_node: Option<Rc<RefCell<MatrixControlNode>>> = None;
        if let Some(rcn) = self.root_control_node.clone() {
            let mut current_control_node = Some(rcn);
            for _ in 0..self.num_columns {
                if let Some(ccn) = current_control_node {
                    current_lowest_control_node = match current_lowest_control_node {
                        None => Some(ccn.clone()),
                        Some(clcn) => {
                            if clcn.borrow().num_rows > ccn.borrow().num_rows {
                                Some(ccn.clone())
                            } else {
                                Some(clcn.clone())
                            }
                        }
                    };

                    current_control_node = ccn.borrow().next_control_node.clone();
                }
            }
        } else {
            return None;
        }

        current_lowest_control_node
    }

    pub fn remove_column(&mut self, column: u8) {
        if let Some(c) = self.find_control_node(column) {
            let opcn = c.borrow().prev_control_node.clone();
            let oncn = c.borrow().next_control_node.clone();
        }
    }
}
