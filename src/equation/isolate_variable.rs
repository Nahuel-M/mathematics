use crate::equation::Equation;

impl Equation{
    pub fn isolate_variable(&self, variable: &str) -> Equation {
        let left = self.left.contains_variable(variable);
        let right = self.right.contains_variable(variable);

        // if left && right {
        //     unimplemented!("Variable appears on both sides of the equation")
        // } else if left {
        //     return Equation{left: self.left.isolate_variable(variable), right: self.right.clone()};
        // } else if right {
        //     return Equation{left: self.right.separate_variable(variable), right: self.left.clone()};
        // } else {
        //     panic!("Variable does not appear on either side of the equation");
        // }
        todo!();
    }
}