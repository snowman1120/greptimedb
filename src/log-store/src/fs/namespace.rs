use std::sync::Arc;

use store_api::logstore::namespace::Namespace;

#[derive(Clone, Debug)]
pub struct LocalNamespace {
    inner: Arc<LocalNamespaceInner>,
}

impl Default for LocalNamespace {
    fn default() -> Self {
        LocalNamespace::new("")
    }
}

#[derive(Debug)]
struct LocalNamespaceInner {
    name: String,
}

impl Namespace for LocalNamespace {
    fn new(name: &str) -> Self {
        let inner = Arc::new(LocalNamespaceInner {
            name: name.to_string(),
        });
        Self { inner }
    }

    fn name(&self) -> &str {
        self.inner.name.as_str()
    }
}
