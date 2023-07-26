struct TypeOne;
struct TypeTwo;
struct TypeThree;

impl TypeOne {
    fn action(&self) {
        println!("TypeOne action");
    }
}

impl TypeTwo {
    fn action(&self) {
        println!("TypeTwo action");
    }
}

impl TypeThree {
    fn action(&self) {
        println!("TypeThree action");
    }
}

enum Wrapper {
    One(TypeOne),
    Two(TypeTwo),
    Three(TypeThree),
}

pub(crate) fn execute() {
    let mut items: Vec<Wrapper> = Vec::new();

    items.push(Wrapper::One(TypeOne));
    items.push(Wrapper::Two(TypeTwo));
    items.push(Wrapper::Three(TypeThree));

    for item in items {
        match item {
            Wrapper::One(x) => x.action(),
            Wrapper::Two(x) => x.action(),
            Wrapper::Three(x) => x.action(),
        }
    }
}
