use reqwest;
use reqwest::header::{HeaderMap, CONTENT_TYPE, AUTHORIZATION,HeaderValue};

const ANZO_URL:&str = "http://172.17.0.2:7070/sparql";
const ANZO_USER:&str = "admin";
const ANZO_PASS:&str = "Passw0rd1";

fn send_query_to_db(query:String) {
    

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&["Basic ",&ANZO_USER,":",&ANZO_PASS].concat().to_string()).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/sparql-query; charset=utf-8"));

    let client = reqwest::blocking::Client::new();
    let res = client    
    .post(ANZO_URL)
    .timeout(std::time::Duration::from_secs(9000))
    .headers(headers)
    .body(query)
    .send();
    println!("{:?}", res);
    println!("{:?}", res.unwrap().text().unwrap());
    ()
}


fn main() {
    println!("Testing Payload To Anzo");

    // let query_init = "LOAD WITH 'global' <s3://csi-notebook-datasets/MovieTicketAnalysis/20190217/tickit.ttl.gz>
    // INTO GRAPH <tickit>".to_string();
    
    let query_test = "SELECT ?g 
    WHERE {
      GRAPH ?g { }
    }".to_string();

    // send_query_to_db(query_init);

    send_query_to_db(query_test);
}
