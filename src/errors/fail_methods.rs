//use failure;

macro_rules! traverse_err {
    ($error:expr, $action_first:expr, $action_others:expr) => {
        let mut cause: &::failure::Fail = $error;

        $action_first(cause);

        while let Some(inner_cause) = cause.cause() {
            $action_others(inner_cause);
            cause = inner_cause
        }
    };
}

pub trait FailMethods {
    fn traverse<F: FnMut(String)>(&self, action: F);
    fn log_error(&self);
    fn log_warning(&self);
    fn stringify(&self) -> String;
}

impl<T> FailMethods for T where T: ::failure::Fail {
    fn traverse<F: FnMut(String)>(&self, action: F) {
        (self as &::failure::Fail).traverse(action)
    }

    fn log_error(&self) {
        (self as &::failure::Fail).log_error()
    }

    fn log_warning(&self) {
        (self as &::failure::Fail).log_warning()
    }

    fn stringify(&self) -> String {
        (self as &::failure::Fail).stringify()
    }
}

impl FailMethods for ::failure::Fail {
    fn traverse<F: FnMut(String)>(&self, mut action: F) {
        traverse_err! {
            self,
            |failure| action(format!("{}", failure)),
            |failure| action(format!("  caused by: {}",failure))
        }
    }

    fn log_error(&self) {
        traverse_err! {
            self,
            |failure| error!("{}", failure),
            |failure| error!("  caused by: {}",failure)
        }
    }

    fn log_warning(&self) {
        traverse_err! {
            self,
            |failure| warn!("{}", failure),
            |failure| warn!("  caused by: {}",failure)
        }
    }

    fn stringify(&self) -> String {
        let mut result = String::new();

        self.traverse(|msg| result.push_str(&msg));

        result
    }
}

pub trait ErrorMethods {
    fn traverse<F: FnMut(String)>(&self, action: F);
    fn log_error(&self);
    fn log_warning(&self);
    fn stringify(&self) -> String;
}

impl ErrorMethods for ::failure::Error {
    fn traverse<F: FnMut(String)>(&self, action: F) {
        self.cause().traverse(action)
    }

    fn log_error(&self) {
        self.cause().log_error()
    }

    fn log_warning(&self) {
        self.cause().log_warning()
    }

    fn stringify(&self) -> String {
        self.cause().stringify()
    }
}
