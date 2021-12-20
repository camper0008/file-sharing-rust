use rocket::request::{Request, FromRequest, Outcome};

pub struct FormDataBoundaryGrabber {
    pub boundary: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for FormDataBoundaryGrabber {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let ct = request.headers().get_one("Content-Type").expect("no Content-Type header given");
        let idx = ct.find("boundary=").expect("no boundary given in Content-Type header");
        let boundary = &ct[idx + "boundary=".len()..];

        Outcome::Success(FormDataBoundaryGrabber {
            boundary: boundary.to_string(),
        })
    }
}