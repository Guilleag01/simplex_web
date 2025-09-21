use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct Simplex {
    variables: Vec<String>,
    constraints: Vec<Vec<f32>>,
    target: Vec<f32>,
}

#[derive(Debug, Clone)]
pub enum VariableType {
    Basic,
    NonBasic(String),
}

impl Simplex {
    pub fn run_simplex(&self) -> (f32, Vec<(String, f32)>) {
        print_matrix(&self.constraints);

        let mut matrix = vec![
            vec![0_f32; self.variables.len() + self.constraints.len() + 1];
            self.constraints.len() + 1
        ];

        for (i, row) in self.constraints.iter().enumerate() {
            // web_sys::console::log_1(&format!("row {} {}", i, self.constraints.len()).into());
            for (j, col) in row.iter().take(self.variables.len()).enumerate() {
                matrix[i][j] = *col;
            }
        }
        for i in 0..self.constraints.len() {
            *matrix[i].last_mut().unwrap() = *self.constraints[i].last().unwrap();
        }

        for i in 0..self.constraints.len() {
            matrix[i][i + self.variables.len()] = 1_f32;
        }

        for (i, t) in self.target.iter().take(self.target.len() - 1).enumerate() {
            matrix.last_mut().unwrap()[i] = *t * -1_f32;
        }
        // *matrix.last_mut().unwrap().last_mut().unwrap() = *self.target.last().unwrap() * -1_f32;
        // matrix.last_mut().unwrap()[0] = 1_f32;

        // let mut basic_variables = self.variables.clone();
        print_matrix(&matrix);

        let mut solution: HashMap<String, f32> = HashMap::new();

        let mut basic_variables = vec![VariableType::Basic; self.constraints.len()];
        let mut non_basic_variables =
            vec![VariableType::Basic; self.variables.len() + self.constraints.len() + 1];
        for (i, v) in self.variables.iter().enumerate() {
            non_basic_variables[i] = VariableType::NonBasic(v.clone());
            solution.insert(v.clone(), 0_f32);
            // web_sys::console::log_1(&format!("asdasdasd").into());
        }

        loop {
            print_matrix(&matrix);

            let pivot_column = get_pivot_column(&matrix);
            let pivot_row = get_pivot_row(&matrix, pivot_column);

            web_sys::console::log_1(
                &format!("Pivot row: {}\nPivot col: {}\n", pivot_row, pivot_column).into(),
            );
            let mut exit = true;

            for e in matrix.last().unwrap() {
                if *e < 0_f32 {
                    exit = false;
                }
            }
            if exit {
                break;
            }

            web_sys::console::log_1(&format!("basic_vars {:?}", basic_variables).into());
            web_sys::console::log_1(&format!("non_basic_vars {:?}", non_basic_variables).into());

            let temp = basic_variables[pivot_row].clone();
            basic_variables[pivot_row] = non_basic_variables[pivot_column].clone();
            non_basic_variables[pivot_column] = temp;

            matrix = get_new_matrix(matrix.clone(), pivot_column, pivot_row);
        }

        for (i, v) in basic_variables.iter().enumerate() {
            if let VariableType::NonBasic(s) = v {
                let sol = *matrix[i].last().unwrap();
                web_sys::console::log_1(&format!("sol {}", sol).into());
                solution.insert(s.clone(), sol);
            }
        }

        (
            *matrix.last().unwrap().last().unwrap(),
            self.variables
                .iter()
                .map(|v| (v.clone(), *solution.get(v).unwrap()))
                .collect(),
        )
    }

    pub fn add_constraint(&mut self, constraint: Vec<f32>) {
        self.constraints.push(constraint);
    }

    pub fn clear_constraint(&mut self) {
        self.constraints.clear();
    }

    pub fn add_variable(&mut self, variable: String) {
        self.variables.push(variable);
    }

    pub fn clear_variables(&mut self) {
        self.variables.clear();
    }

    pub fn set_target(&mut self, target: Vec<f32>) {
        self.target = target;
    }

    pub fn get_constraints(&self) -> Vec<Vec<f32>> {
        self.constraints.clone()
    }

    pub fn get_variables(&self) -> Vec<String> {
        self.variables.clone()
    }

    pub fn get_target(&self) -> Vec<f32> {
        self.target.clone()
    }
}

fn print_matrix(m: &Vec<Vec<f32>>) {
    let mut s = String::new();

    for row in m {
        for col in row {
            s.push_str(format!("{:.2} ", col).as_str());
        }
        s.push_str("\n");
    }
    web_sys::console::log_1(&s.into());
}

fn get_pivot_column(matrix: &Vec<Vec<f32>>) -> usize {
    let mut pivot_column = 0;

    for (i, e) in matrix
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .take(matrix.last().unwrap().len() - 1)
    {
        if *e < matrix.last().unwrap()[pivot_column] {
            pivot_column = i;
        }
    }

    pivot_column
}

fn get_pivot_row(matrix: &Vec<Vec<f32>>, pivot_column: usize) -> usize {
    let mut pivot_row = 0;

    for (i, e) in matrix.iter().enumerate().take(matrix.len() - 1) {
        if e.last().unwrap() / e[pivot_column]
            < matrix[pivot_row].last().unwrap() / matrix[pivot_row][pivot_column]
        {
            pivot_row = i;
        }
    }
    pivot_row
}

fn get_new_matrix(
    mut matrix: Vec<Vec<f32>>,
    pivot_column: usize,
    pivot_row: usize,
) -> Vec<Vec<f32>> {
    let pivot_element = matrix[pivot_row][pivot_column];

    for e in matrix[pivot_row].iter_mut() {
        *e /= pivot_element;
    }

    let p_row = matrix[pivot_row].clone();
    for (i, row) in matrix.iter_mut().enumerate() {
        if i == pivot_row {
            continue;
        }

        let pivot_c = row[pivot_column];
        for (j, e) in row.iter_mut().enumerate() {
            web_sys::console::log_1(&format!("e = {} - {}*{}", *e, pivot_c, p_row[j]).into());
            *e -= pivot_c * p_row[j];
        }
    }
    web_sys::console::log_1(&format!("pivot_element {}", pivot_element).into());
    print_matrix(&matrix);
    matrix
}
