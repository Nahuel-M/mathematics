use crate::expression::Expression;

// a * c + b * c -> c * (a + b)
// (a + b) * (c + d) -> a * (c + d) + b * (c + d)
impl Expression {
    pub fn isolate_variable(&self, variable: &str) -> Option<Expression> {
        use crate::expression::Expression::*;
        match self.simplify(){
            Constant(_) | Number(_) => None,
            Variable(name) if name == variable => Some(self.clone()),
            Variable(_) => None,
            Add(add) => {
                if add.0.iter().filter(|child| child.contains_variable(variable)).count() == 1{
                    return Some(self.clone());
                }
                None
            }
            _ => {
                let children = self.children();
                if children.len() == 1{
                    return Some(self.clone());
                }

                let isolations: Vec<_> = children
                    .iter()
                    .filter_map(|child| child.isolate_variable(variable))
                    .collect();
                None
                // match isolations.len(){
                //     0 => return None,
                //     1 => return Some(self.clone()),
                //     2 => {
                //         match self{
                //             Add(a) => {},
                //             Multiply(a) => {},
                //             Power(a, b) => {},
                //             Log(a, b) => {}
                //             _ => unreachable!()
                //         }
                //     }
                //     _ => unreachable!()
                // }
            }
        }

    }
}