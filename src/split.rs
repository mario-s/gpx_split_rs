struct Context<S> {
    strategy: S,
}

impl<S> Context<S>
where
    S: Splitter,
{
    fn do_things(&self) {
        println!("Common preamble");
        self.strategy.execute();
        println!("Common postamble");
    }
}

trait Splitter {
    fn execute(&self);
}

struct PointsSplitter;

impl Splitter for PointsSplitter {
    fn execute(&self) {
        println!("ConcreteStrategyA")
    }
}

struct LengthSplitter;

impl Splitter for LengthSplitter {
    fn execute(&self) {
        println!("ConcreteStrategyB")
    }
}
