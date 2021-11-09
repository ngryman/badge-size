use tide::http::{headers, Request};

/// List of denied user agents
const UA_DENYLIST: [&str; 1] = [
  // Baidu seems to ignore robots.txt and is pretty agressive
  "Baiduspider",
];

/// List of denied URLs
const URL_DENYLIST: [&str; 1] = [
  // This project uses dozens of badge in the README for each JS file.
  // I contacted the author and asked to reduce the number of badges, without
  // success.
  "vxe-table",
];

/// Return true if the given url and user agents are allowed
pub fn is_allowed(req: &Request) -> bool {
  let is_ua_allowed = match req.header(headers::USER_AGENT) {
    Some(ua) => {
      let ua = ua.as_str();
      !UA_DENYLIST.iter().any(|deny| ua.contains(deny))
    }
    None => false,
  };

  let path = req.url().path();
  let is_url_allowed = !URL_DENYLIST.iter().any(|deny| path.contains(deny));

  is_ua_allowed && is_url_allowed
}

#[cfg(test)]
mod tests {
  use tide::http::{headers, Request};

  use super::is_allowed;

  #[test]
  fn allow_valid_url_and_user_agent() {
    // Arrange
    let mut req = Request::get("http://🚀/ngryman/example/main/README.md.svg");
    req.append_header(
      headers::USER_AGENT,
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.63 Safari/537.36"
    );

    // Act
    let res = is_allowed(&req);

    // Assert
    assert!(res);
  }

  #[test]
  fn deny_invalid_user_agent() {
    // Arrange
    let mut req = Request::get("http://🚀/ngryman/example/main/README.md.svg");
    req.append_header(
      headers::USER_AGENT,
      "Baiduspider ( http://www.baidu.com/search/spider.htm)",
    );

    // Act
    let res = is_allowed(&req);

    // Assert
    assert!(!res);
  }

  #[test]
  fn deny_invalid_url() {
    // Arrange
    let mut req =
      Request::get("http://🚀/vxe-table/vxe-table/master/README.md.svg");
    req.append_header(
      headers::USER_AGENT,
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/93.0.4577.63 Safari/537.36"
    );

    // Act
    let res = is_allowed(&req);

    // Assert
    assert!(!res);
  }
}
