use iron;
use iron::modifiers::RedirectRaw;

// Create the Error, ErrorKind, ResultExt, and Result types
error_chain! {
    foreign_links {
        Reqwest(::reqwest::Error);
    }

    errors {
        NoSuchTweet(id: i64) {
            description("Can't find a tweet with that id")
            display("Unable to read tweet `{}`", id)
        }
    }
}

impl From<Error> for iron::IronError {
    fn from(e: Error) -> Self {
        match e {
            Error(ErrorKind::NoSuchTweet(_), _) => {
                iron::IronError::new(e,
                                     (iron::status::Found,
                                      RedirectRaw("/?error=Can't%20find%20tweet%20with%20that%20URL"
                                                      .to_owned())))
            }
            _ => {
                let msg = format!("{}", e);
                iron::IronError::new(e, msg)
            }
        }
    }
}