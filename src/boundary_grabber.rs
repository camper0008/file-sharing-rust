use rocket::request::{Request, FromRequest, Outcome};

pub struct BoundaryGrabber {
    pub boundary: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for BoundaryGrabber {
    type Error = String;

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let ct = request.headers().get_one("Content-Type").expect("no Content-Type header given");
        let idx = ct.find("boundary=").expect("no boundary given in Content-Type header");
        let boundary = &ct[idx + "boundary=".chars().count()..];

        Outcome::Success(BoundaryGrabber {
            boundary: boundary,
        })
    }
}