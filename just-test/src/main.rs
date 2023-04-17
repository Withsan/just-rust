#[tokio::main]
async fn main() {
    let mut cacher = Cacher {
        value: None,
        query: |v| 5 + v,
    };
    println!("{:?}", cacher.value(11));
}

struct Cacher<F, T>
where
    F: Fn(T) -> T,
{
    value: Option<T>,
    query: F,
}

impl<F, T> Cacher<F, T>
where
    F: Fn(T) -> T,
{
    fn value<'a>(&mut self, arg: T) -> T
    where
        T: 'a + Clone,
    {
        match &self.value {
            Some(t) => t.clone(),
            None => {
                let v = (self.query)(arg);
                self.value = Some(v.clone());
                v
            }
        }
    }
}
