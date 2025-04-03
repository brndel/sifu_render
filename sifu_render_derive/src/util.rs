pub trait CollectSynErrors {
    type Item;
    fn collect_syn_errors(self) -> syn::Result<Vec<Self::Item>>;
}

impl<I: Iterator<Item = syn::Result<T>>, T> CollectSynErrors for I {
    type Item = T;

    fn collect_syn_errors(self) -> syn::Result<Vec<Self::Item>> {
        self.fold(Ok(Vec::new()), |acc, value| match (acc, value) {
            (Ok(mut vec), Ok(value)) => {
                vec.push(value);
                Ok(vec)
            }
            (Ok(_), Err(err)) => Err(err),
            (Err(err), Ok(_)) => Err(err),
            (Err(mut acc_err), Err(err)) => {
                acc_err.combine(err);
                Err(acc_err)
            }
        })
    }
}
