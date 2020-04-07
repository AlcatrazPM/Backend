pub mod statuscode;
pub mod response;

#[cfg(test)]
mod tests {
    use crate::statuscode::StatsCodes;
    use crate::response::Response;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_status() {
        let x = StatsCodes::OK;
        assert_eq!("200 OK", x.to_string());
        let x = StatsCodes::Forbidden;
        assert_eq!("403 Forbidden", x.to_string());
    }

    #[test]
    fn response() {
        let response = Response::build()
            .status(StatsCodes::OK)
            .content("html/text".to_string(), 10)
            .body("ABCDEF".to_string());

        let response = response.to_string();
        let reference = String::from("HTTP/1.1 200 OK\r\nContent-Type: html/text\r\nContent-Length: 10\r\n\r\nABCDEF\r\n");
        assert_eq!(response, reference);
    }
}

