trait Action {
    fn action(&self);
}

struct TypeOne;
struct TypeTwo;
struct TypeThree;

impl Action for TypeOne {
    fn action(&self) {
        println!("TypeOne action");
    }
}

impl Action for TypeTwo {
    fn action(&self) {
        println!("TypeTwo action");
    }
}

impl Action for TypeThree {
    fn action(&self) {
        println!("TypeThree action");
    }
}

pub(crate) fn execute() {
    let mut items: Vec<Box<dyn Action>> = Vec::new();

    items.push(Box::new(TypeOne));
    items.push(Box::new(TypeTwo));
    items.push(Box::new(TypeThree));

    for item in items {
        item.action();
    }
}
