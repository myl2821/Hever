use std::collections::HashMap;

/// suppose all file is located on 'public'
pub fn parse(uri: String, filename: &mut String, args: &mut HashMap<String, String>) {
    let uri: Vec<&str> = uri.split('?').collect();
    *filename = "public".to_string() + uri[0];
    if uri.len() > 1 {
        let args_str = uri[1].split('&');
        for arg in args_str {
            let kv: Vec<&str> = arg.split('=').collect();
            if kv.len() != 2 {
                return;
            } else {
                args.insert(kv[0].to_string(), kv[1].to_string());
            }
        }
    }
}

#[test]
fn test_parse() {
    let uri = "/xxx?a=11&b=22".to_string();
    let mut filename = String::new();
    let mut args = HashMap::<String, String>::new();
    parse(uri, &mut filename, &mut args);
    assert_eq!(filename, "public/xxx");
    let xx = args.get(&("a".to_string()));
    assert_eq!(args.len(), 2);
    assert_eq!(args.get(&("a".to_string())).unwrap(), &("11".to_string()));
    assert_eq!(args.get(&("b".to_string())).unwrap(), &("22".to_string()));
}



