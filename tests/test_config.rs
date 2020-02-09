#[cfg(test)]
mod test {
    use std::collections::LinkedList;
    use std::string::String;
    use sun::config::config::{load_hosts, read_config_file};
    use sun::entity::entity::Host;

    #[test]
    #[should_panic]
    fn test_read_config_file_abnormal_path() {
        read_config_file(String::from("test"));
    }

    #[test]
    #[should_panic]
    fn test_load_hosts_empty_string() {
        load_hosts(String::from(""));
    }

    #[test]
    #[should_panic]
    fn test_load_hosts_invalid_doc() {
        let invalid_content = "hosts:
                                        - name : user
                                          ip: 10.20.30.33
                                        - name: token
                                          ip: 10.20.10.23
                                    domain:
                                        - ur: google.com
                                    ";
        load_hosts(String::from(invalid_content.to_string()));
    }

    #[test]
    fn test_load_hosts() {
        let content = "hosts:
                                - name : user
                                  ip: 10.20.30.33
                                - name: token
                                  ip: 10.20.10.23
                            ";

        let mut expected: LinkedList<Host> = LinkedList::new();
        expected.push_back(Host {
            name: "user".to_string(),
            ip: "10.20.30.33".to_string(),
        });
        expected.push_back(Host {
            name: "token".to_string(),
            ip: "10.20.10.23".to_string(),
        });
        // todo : assert_eq!(load_hosts(String::from(content.to_string())), expected)
    }
}
