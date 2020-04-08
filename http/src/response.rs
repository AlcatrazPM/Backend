use crate::statuscode::StatsCodes;

const END_LINE: &str = "\r\n";

#[derive(Debug)]
pub struct Response {
    version: String,
    status: StatsCodes,
    access_control_origin: String,
    content_type: Option<String>,
    content_length: Option<u32>,
    body: Option<String>,
}

impl Response {
    pub fn build() -> Response {
        Response {
            version: String::from("HTTP/1.1"),
            status: StatsCodes::NotImplemented,
            // TODO: Don't leave it like this for final release
            access_control_origin: String::from("*"),
            content_type: None,
            content_length: None,
            body: None,
        }
    }

    pub fn version(mut self, version: String) -> Response {
        self.version = version;
        self
    }

    pub fn status(mut self, status: StatsCodes) -> Response {
        self.status = status;
        self
    }

    pub fn content(mut self, ctype: String, clength: u32) -> Response {
        self.content_type = Option::from(ctype);
        self.content_length = Option::from(clength);
        self
    }

    pub fn body(mut self, body: String) -> Response {
        self.body = Option::from(body);
        self
    }
}

impl ToString for Response {
    fn to_string(&self) -> String {
        let mut response = String::new();
        response.push_str(self.version.as_str());
        response.push_str(" ");
        response.push_str(self.status.to_string().as_str());
        response.push_str(END_LINE);
        response.push_str("Acces-Control-Allow-Origin: ");
        response.push_str(self.access_control_origin.as_str());
        response.push_str(END_LINE);

        if let Some(ctype) = &self.content_type {
            response.push_str("Content-Type: ");
            response.push_str(ctype.as_str());
            response.push_str(END_LINE);
        }

        if let Some(clength) = &self.content_length {
            let length = format!("{}", clength);
            response.push_str("Content-Length: ");
            response.push_str(length.as_str());
            response.push_str(END_LINE);
        }

        // body
        response.push_str(END_LINE);
        if let Some(body) = &self.body {
            response.push_str(body.as_str());
            response.push_str(END_LINE);
        }

        response
    }
}
