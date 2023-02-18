/**/

use crate::common::*;

pub struct Machine {
    tape: Vec<Expression>,
    call_stack: Vec<EvaluationInProgress>,
    fetched: Expression,
    instruction_address: usize,
    verbose: bool,
}

#[derive(Debug)]
struct EvaluationInProgress {
    expression: Expression,
}

impl Machine {
    pub fn create(program: Vec<Instruction>, verbose: bool) -> Machine {
        let mut tape = Vec::new();
        let min_tape_size: usize = program
            .iter()
            .map(|instruction| instruction.label)
            .max()
            .unwrap_or_default();
        tape.resize(min_tape_size + 1, Default::default());
        for instruction in program {
            tape[instruction.label] = instruction.expression;
        }
        Machine {
            tape,
            call_stack: Vec::new(),
            fetched: Expression::default(),
            instruction_address: 0,
            verbose,
        }
    }

    pub fn evaluate_until_finished(&mut self, address: usize) -> Expression {
        if self.verbose {
            println!();
            println!();
        }
        self.fetch(address);
        while !self.is_finished() {
            self.tick();
        }
        let expression = std::mem::take(&mut self.fetched);
        match expression {
            Expression::Undefined | Expression::Number(_) => expression,
            Expression::List(_) => expression,
            Expression::PointerIntoList { address, offset } => {
                self.copy_list(address, offset)
            }
            Expression::Sequence(_) => unreachable!(),
            Expression::Unary { .. } => unreachable!(),
            Expression::Binary { .. } => unreachable!(),
            Expression::Stub => unreachable!(),
        }
    }

    fn is_finished(&self) -> bool {
        self.call_stack.is_empty()
    }

    fn fetch(&mut self, address: usize) {
        let expression = self
            .tape
            .get(address)
            .cloned()
            .unwrap_or(Expression::Undefined);
        match expression {
            _ if address == self.instruction_address => self.fetch(0),
            Expression::Undefined
            | Expression::Number(_)
            | Expression::PointerIntoList { .. } => self.fetched = expression,
            Expression::List(_) => {
                self.fetched =
                    Expression::PointerIntoList { address, offset: 0 };
            }
            Expression::Sequence(_)
            | Expression::Unary { .. }
            | Expression::Binary { .. } => {
                if self.verbose {
                    println!("Evaluating {}: {:?}", address, expression);
                }
                self.call_stack.push(EvaluationInProgress { expression });
                self.fetched = Expression::Undefined;
                self.set_called_with(Expression::Undefined);
                self.instruction_address = address;
            }
            Expression::Stub => unreachable!(),
        }
    }

    fn set_called_with(&mut self, expression: Expression) {
        if let Some(stored) = self.tape.get_mut(0) {
            *stored = expression;
        }
    }

    fn store(&mut self, address: usize, expression: Expression) {
        match self.tape.get_mut(address) {
            _ if address == 0 => {
                // Writing to address 0 is disallowed, because it is used
                // internally to store the call arguments.
                self.fetched = Expression::Undefined;
            }
            Some(stored) => {
                *stored = expression;
                self.fetched = Expression::Undefined;
            }
            None => {
                // TODO allow writes to arbitrary memory by expanding the tape
                self.fetched = Expression::Undefined;
            }
        }
    }

    fn tick(&mut self) {
        match self.call_stack.last_mut() {
            Some(evaluation) => match &mut evaluation.expression {
                Expression::Undefined
                | Expression::Number(_)
                | Expression::PointerIntoList { .. }
                | Expression::List(_) => {
                    std::mem::swap(
                        &mut self.fetched,
                        &mut evaluation.expression,
                    );
                    self.call_stack.pop();
                }
                Expression::Sequence(steps) => match steps.pop_front() {
                    Some(step) => {
                        if steps.is_empty() {
                            self.call_stack.pop();
                        }
                        self.solve(step);
                    }
                    None => self.fetched = Expression::Undefined,
                },
                Expression::Unary { operator, operand } => {
                    let expr: Expression = std::mem::take(operand);
                    let expr = match expr {
                        Expression::Stub => std::mem::take(&mut self.fetched),
                        expression => expression,
                    };
                    if is_value(&expr) {
                        let operator = *operator;
                        self.call_stack.pop();
                        self.perform_unary_on_value(operator, expr)
                    } else {
                        *operand = Box::new(Expression::Stub);
                        let sub = EvaluationInProgress { expression: expr };
                        self.call_stack.push(sub);
                    }
                }
                Expression::Binary {
                    operator,
                    left: left_operand,
                    right: right_operand,
                } => {
                    let left: Expression = std::mem::take(left_operand);
                    let left = match left {
                        Expression::Stub => std::mem::take(&mut self.fetched),
                        expression => expression,
                    };
                    let right: Expression = std::mem::take(right_operand);
                    let right = match right {
                        Expression::Stub => std::mem::take(&mut self.fetched),
                        expression => expression,
                    };
                    if is_value(&left) && is_value(&right) {
                        let operator = *operator;
                        self.call_stack.pop();
                        self.perform_binary_on_values(operator, left, right)
                    } else if is_value(&left) {
                        *left_operand = Box::new(left);
                        *right_operand = Box::new(Expression::Stub);
                        let sub = EvaluationInProgress { expression: right };
                        self.call_stack.push(sub);
                    } else {
                        *left_operand = Box::new(Expression::Stub);
                        *right_operand = Box::new(right);
                        let sub = EvaluationInProgress { expression: left };
                        self.call_stack.push(sub);
                    }
                }
                Expression::Stub => unreachable!(),
            },
            None => (),
        }
    }

    fn solve(&mut self, expression: Expression) {
        match expression {
            Expression::Undefined
            | Expression::Number(_)
            | Expression::PointerIntoList { .. }
            | Expression::List(_) => {
                self.fetched = expression;
            }
            Expression::Sequence(_)
            | Expression::Unary { .. }
            | Expression::Binary { .. } => {
                if self.verbose {
                    println!("Evaluating {:?}", expression);
                }
                self.call_stack.push(EvaluationInProgress { expression });
                self.fetched = Expression::Undefined;
            }
            Expression::Stub => unreachable!(),
        }
    }

    fn perform_unary_on_value(&mut self, operator: Unary, operand: Expression) {
        match operator {
            Unary::Fetch => match operand {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(number) => {
                    // Fetch or evaluate the expression at the given address.
                    if let Some(address) = self.address_from_number(number) {
                        self.fetch(address)
                    } else {
                        self.solve(Expression::Undefined);
                    }
                }
                Expression::List(elements) => {
                    // Get element 0 from the list.
                    if let Some(element) = elements.into_iter().next() {
                        self.solve(element);
                    } else {
                        self.solve(Expression::Undefined);
                    }
                }
                Expression::PointerIntoList { address, offset } => {
                    // Get element 0 from the slice that begins at offset.
                    let element = self.copy_element(address, offset);
                    self.solve(element);
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&operand) && false),
            },
            Unary::Signum => match operand {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(number) => {
                    let expr = if number == 0.0 || number.is_subnormal() {
                        Expression::Number(0.0)
                    } else if number.is_normal() {
                        Expression::Number(number.signum())
                    } else {
                        Expression::Undefined
                    };
                    self.solve(expr);
                }
                Expression::List(elements) => {
                    // This has to match the behavior below.
                    let list = Expression::List(elements);
                    self.solve(list);
                }
                Expression::PointerIntoList { address, offset } => {
                    // Overload signum to copy lists.
                    let list = self.copy_list(address, offset);
                    self.solve(list);
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&operand) && false),
            },
            Unary::Neg => match operand {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(number) => {
                    let expr = Expression::Number(-number);
                    self.solve(expr);
                }
                Expression::List(_) => {
                    // TODO maybe list reversal?
                    unimplemented!()
                }
                Expression::PointerIntoList { .. } => {
                    // This has to match the behavior above.
                    unimplemented!()
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&operand) && false),
            },
            Unary::Recip => match operand {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(number) => {
                    // TODO do we need to handle divide by zero?
                    let expr = Expression::Number(1.0 / number);
                    self.solve(expr);
                }
                Expression::List(_) => {
                    // TODO ???
                    unimplemented!()
                }
                Expression::PointerIntoList { .. } => {
                    // This has to match the behavior above.
                    unimplemented!()
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&operand) && false),
            },
        }
    }

    fn perform_binary_on_values(
        &mut self,
        operator: Binary,
        left: Expression,
        right: Expression,
    ) {
        match operator {
            Binary::Plus => match left {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(a) => match right {
                    Expression::Undefined => self.solve(Expression::Undefined),
                    Expression::Number(b) => {
                        self.solve(Expression::Number(a + b));
                    }
                    Expression::List(_) => {
                        // TODO ???
                        unimplemented!()
                    }
                    Expression::PointerIntoList { .. } => {
                        // This has to match the behavior above.
                        unimplemented!()
                    }
                    Expression::Sequence(_)
                    | Expression::Unary { .. }
                    | Expression::Binary { .. }
                    | Expression::Stub => assert!(is_value(&right) && false),
                },
                Expression::List(mut elements) => match right {
                    Expression::Undefined => self.solve(Expression::Undefined),
                    Expression::Number(number) => {
                        // Drop the first NUM elements from the list.
                        if let Some(offset) = self.address_from_number(number) {
                            elements.splice(0..offset, std::iter::empty());
                            let shifted = Expression::List(elements);
                            self.solve(shifted);
                        } else {
                            self.solve(Expression::Undefined);
                        }
                    }
                    Expression::List(_) => {
                        // TODO ???
                        unimplemented!()
                    }
                    Expression::PointerIntoList { .. } => {
                        // This has to match the behavior above.
                        unimplemented!()
                    }
                    Expression::Sequence(_)
                    | Expression::Unary { .. }
                    | Expression::Binary { .. }
                    | Expression::Stub => assert!(is_value(&right) && false),
                },
                Expression::PointerIntoList { address, offset } => {
                    match right {
                        Expression::Undefined => {
                            self.solve(Expression::Undefined)
                        }
                        Expression::Number(number) => {
                            // Drop the first NUM elements from the list.
                            if let Some(skipped) =
                                self.address_from_number(number)
                            {
                                let shifted = Expression::PointerIntoList {
                                    address,
                                    offset: offset + skipped,
                                };
                                self.solve(shifted);
                            } else {
                                self.solve(Expression::Undefined);
                            }
                        }
                        Expression::List(_) => {
                            // TODO ???
                            unimplemented!()
                        }
                        Expression::PointerIntoList { .. } => {
                            // This has to match the behavior above.
                            unimplemented!()
                        }
                        Expression::Sequence(_)
                        | Expression::Unary { .. }
                        | Expression::Binary { .. }
                        | Expression::Stub => {
                            assert!(is_value(&right) && false)
                        }
                    }
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&left) && false),
            },
            Binary::Mult => {
                // TODO
                unimplemented!()
            }
            Binary::Assign => match left {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(number) => {
                    // Assign a value to a register.
                    if let Some(address) = self.address_from_number(number) {
                        self.store(address, right);
                    } else {
                        self.solve(Expression::Undefined);
                    }
                }
                Expression::List(_) => {
                    // TODO ???
                    unimplemented!()
                }
                Expression::PointerIntoList { .. } => {
                    // This has to match the behavior above.
                    unimplemented!()
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&left) && false),
            },
            Binary::CallWith => match left {
                Expression::Undefined => self.solve(Expression::Undefined),
                Expression::Number(number) => {
                    // Evaluate the expression at the given address...
                    if let Some(address) = self.address_from_number(number) {
                        self.fetch(address);
                    } else {
                        self.solve(Expression::Undefined);
                    }
                    // ...after storing the argument list.
                    self.set_called_with(right);
                }
                Expression::List(_) => {
                    // TODO ???
                    unimplemented!()
                }
                Expression::PointerIntoList { .. } => {
                    // This has to match the behavior above.
                    unimplemented!()
                }
                Expression::Sequence(_)
                | Expression::Unary { .. }
                | Expression::Binary { .. }
                | Expression::Stub => assert!(is_value(&left) && false),
            },
        }
    }

    fn copy_list(&self, address: usize, offset: usize) -> Expression {
        match self.tape.get(address) {
            Some(Expression::List(elements)) => Expression::List(
                elements.iter().skip(offset).cloned().collect(),
            ),
            Some(_) => Expression::Undefined,
            None => Expression::Undefined,
        }
    }

    fn copy_element(&self, address: usize, offset: usize) -> Expression {
        match self.tape.get(address) {
            Some(Expression::List(elements)) => {
                let element = elements.iter().skip(offset).next().cloned();
                match element {
                    Some(element) => element,
                    None => Expression::Undefined,
                }
            }
            Some(_) => Expression::Undefined,
            None => Expression::Undefined,
        }
    }

    fn address_from_number(&self, number: Float) -> Option<usize> {
        if number.is_normal() {
            if number < 0.5 {
                Some(0)
            } else if number < (self.tape.len() * 2) as f64 {
                // TODO is there a better way to do a sound conversion?
                Some(number as usize)
            } else {
                None
            }
        } else if number == 0.0 {
            Some(0)
        } else {
            None
        }
    }
}

fn is_value(expression: &Expression) -> bool {
    match expression {
        Expression::Undefined => true,
        Expression::Number(_) => true,
        Expression::List(_) => true,
        Expression::PointerIntoList { .. } => true,
        Expression::Sequence(_) => false,
        Expression::Unary { .. } => false,
        Expression::Binary { .. } => false,
        Expression::Stub { .. } => false,
    }
}
