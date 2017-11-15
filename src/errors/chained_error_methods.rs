use error_chain::ChainedError;

pub trait ChainedErrorMethods {
    fn traverse<F: FnMut(String)>(&self, action: F);
    fn log_error(&self);
    fn log_warning(&self);
    fn stringify(&self) -> String;
}

impl<T> ChainedErrorMethods for T where T: ChainedError {
    fn traverse<F: FnMut(String)>(&self, mut action: F) {
        action(format!("{}", self));

        for e in self.iter().skip(1) {
            action(format!("  caused by: {}", e));
        }

        if let Some(backtrace) = self.backtrace() {
            action(format!("backtrace: {:?}", backtrace));
        }
    }

    fn log_error(&self) {
        self.traverse(|msg| error!("{}", msg))
    }

    fn log_warning(&self) {
        self.traverse(|msg| warn!("{}", msg))
    }

    fn stringify(&self) -> String {
        let mut result = String::new();

        self.traverse(|msg| result.push_str(&msg));

        result
    }
}
