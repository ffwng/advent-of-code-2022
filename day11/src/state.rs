use itertools::Itertools;

pub type Level = u32;

#[derive(Debug, Clone)]
pub enum Operand {
    Old,
    Level(Level),
}

#[derive(Debug, Clone)]
pub enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub lhs: Operand,
    pub op: Operator,
    pub rhs: Operand,
}

impl Operand {
    fn eval(&self, old: Level) -> Level {
        match self {
            Self::Old => old,
            &Self::Level(n) => n,
        }
    }
}

impl Operator {
    fn eval(&self, lhs: Level, rhs: Level) -> Level {
        match self {
            Self::Add => lhs + rhs,
            Self::Multiply => lhs * rhs,
        }
    }
}

impl Operation {
    pub fn eval(&self, old: Level) -> Level {
        self.op.eval(self.lhs.eval(old), self.rhs.eval(old))
    }
}

#[derive(Debug, Clone)]
pub struct Test {
    pub divisor: Level,
    pub if_true: usize,
    pub if_false: usize,
}

impl Test {
    pub fn test(&self, input: Level) -> usize {
        if input % self.divisor == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }
}

#[derive(Debug)]
pub struct Action<T> {
    pub item: T,
    pub target: usize,
}

#[derive(Debug, Clone)]
pub struct Monkey {
    pub items: Vec<Level>,
    pub operation: Operation,
    pub test: Test,
}

impl Monkey {
    pub fn turn(&mut self) -> Vec<Action<Level>> {
        self.items
            .drain(..)
            .map(|mut item| {
                item = self.operation.eval(item);
                item /= 3;
                let target = self.test.test(item);
                Action { item, target }
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
pub struct ModuloLevel<'a> {
    residues: Vec<Level>,
    moduli: &'a [Level],
}

impl<'a> ModuloLevel<'a> {
    pub fn new(level: Level, moduli: &'a [Level]) -> Self {
        Self {
            residues: moduli.iter().map(|m| level % m).collect(),
            moduli,
        }
    }

    pub fn eval_mut(&mut self, op: &Operation) {
        for i in 0..self.residues.len() {
            self.residues[i] = op.eval(self.residues[i]) % self.moduli[i];
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModuloMonkey<'a> {
    pub items: Vec<ModuloLevel<'a>>,
    pub operation: Operation,
    pub test: Test,
    pub idx: usize,
}

impl<'a> ModuloMonkey<'a> {
    pub fn new(monkey: Monkey, moduli: &'a [u32]) -> Self {
        let Monkey {
            items,
            operation,
            test,
        } = monkey;
        let items = items
            .into_iter()
            .map(|i| ModuloLevel::new(i, moduli))
            .collect();

        let idx = moduli
            .iter()
            .find_position(|&&m| m == test.divisor)
            .unwrap()
            .0;

        Self {
            items,
            operation,
            test,
            idx,
        }
    }

    pub fn turn(&mut self) -> Vec<Action<ModuloLevel<'a>>> {
        self.items
            .drain(..)
            .map(|mut item| {
                item.eval_mut(&self.operation);
                let target = self.test.test(item.residues[self.idx]);
                Action { item, target }
            })
            .collect()
    }
}
