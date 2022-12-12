use std::{cell::RefCell, rc::Rc};

pub struct Monkey {
    throw_to_if_true: usize,
    throw_to_if_false: usize,
    operation: Box<dyn Fn(usize) -> usize>,
    divide_by: usize,
    test: Box<dyn Fn(usize) -> bool>,
    items: Vec<usize>,
    appraisal_count: usize,
}

impl Monkey {
    pub fn new() -> Self {
        Self {
            throw_to_if_false: 0,
            throw_to_if_true: 0,
            operation: Box::new(|a| a),
            divide_by: 1,
            test: Box::new(|_| false),
            items: Vec::new(),
            appraisal_count: 0,
        }
    }

    pub fn set_throw_to_if_true(&mut self, index: usize) {
        println!("If true: {}", index);
        self.throw_to_if_true = index;
    }

    pub fn set_throw_to_if_false(&mut self, index: usize) {
        println!("If false: {}", index);
        self.throw_to_if_false = index;
    }

    pub fn set_operator_from(&mut self, lhs: &str, op: &str, rhs: &str) {
        let func = move |lhs: &str, op: &str, rhs: &str| {
            let lhs = lhs.to_owned();
            let rhs = rhs.to_owned();
            let op = op.to_owned();
            println!("{lhs} {op} {rhs}");
            return move |a| {
                let left = match lhs.as_str() {
                    "old" => a,
                    left => left.parse().unwrap(),
                };

                let right = match rhs.as_str() {
                    "old" => a,
                    right => right.parse().unwrap(),
                };

                match op.as_str() {
                    "*" => left * right,
                    "+" => left + right,
                    "/" => left / right,
                    "-" => left - right,
                    _ => panic!("Invalid operation"),
                }
            };
        };

        self.set_operator(Box::new(func(lhs, op, rhs)));
    }
    pub fn set_operator(&mut self, operation: Box<dyn Fn(usize) -> usize>) {
        self.operation = operation;
    }

    pub fn set_test_from_dividable_by(&mut self, number: usize) {
        println!("Divisible by {}", number);
        self.divide_by = number;
        self.set_test(Box::new(move |a| a % number == 0));
    }

    pub fn set_test(&mut self, test: Box<dyn Fn(usize) -> bool>) {
        self.test = test;
    }

    pub fn set_items(&mut self, items: Vec<usize>) {
        println!("Items: {:?}", items);
        self.items = items;
    }

    pub fn turn(&mut self, monkeys: &mut Vec<Rc<RefCell<Monkey>>>) {
        for i in self.items.iter() {
            let y = (self.operation)(*i);
            let y = (y / 3);
            if (self.test)(y) {
                monkeys[self.throw_to_if_true].borrow_mut().items.push(y);
            } else {
                monkeys[self.throw_to_if_false].borrow_mut().items.push(y);
            }
            self.appraisal_count += 1;
        }
        self.items.clear();
    }

    pub fn turn_p2(&mut self, modulo: usize, monkeys: &mut Vec<Rc<RefCell<Monkey>>>) {
        for i in self.items.iter() {
            let y = (self.operation)(*i);

            let y = y % modulo;
            if (self.test)(y) {
                monkeys[self.throw_to_if_true].borrow_mut().items.push(y);
            } else {
                monkeys[self.throw_to_if_false].borrow_mut().items.push(y);
            }
            self.appraisal_count += 1;
        }
        self.items.clear();
    }

    pub fn get_modulo(&self) -> usize {
        return self.divide_by;
    }

    pub fn get_appraisal_count(&self) -> usize {
        return self.appraisal_count;
    }

    pub fn get_items_count(&self) -> usize {
        return self.items.len();
    }
}
