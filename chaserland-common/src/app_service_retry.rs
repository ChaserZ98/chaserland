pub trait RetryErrorPolicy<E>
where
    E: std::error::Error,
{
    fn set_policy(&mut self, should_retry: fn(&E) -> bool);
}

pub trait RetryAsyncFn<T, E, F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, E>>,
{
    fn run(&self, f: F) -> impl Future<Output = Result<T, E>>;
}
