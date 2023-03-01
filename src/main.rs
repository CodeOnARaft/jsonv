use serde_json::{Result, Value};

fn main() -> Result<()> {
    // Some JSON input data as a &str. Maybe this comes from the user.
    let data = r#"
    {
        "_id": "YIIUQOFH4QV9SGR0",
        "name": "Belkis Langdon-Wiles",
        "dob": "2017-06-30",
        "address": {
            "street": "8545 Rathmel Street",
            "town": "Langport",
            "postode": "MK1 8JM"
        },
        "telephone": "+965-9918-490-205",
        "pets": [
            "Loki",
            "Lilly"
        ],
        "score": 5.7,
        "email": "reagan4@hotmail.com",
        "url": "http://five.com",
        "description": "somewhat locking pmc guided jaguar rotary frequency explained difficulties representatives tramadol hot denver separation inflation urge environmental functions silent technical",
        "verified": true,
        "salary": 62045
    }"#;

    // Parse the string of data into serde_json::Value.
    let mut v: Value = Value::default();
     match serde_json::from_str(data) {
        Ok(vv) =>v=vv,
        Err(e) => println!("{}",e)
     }

    printItem(&v, "");
    // Access parts of the data by indexing with square brackets.

    Ok(())
}


fn printItem(v:&Value,name:&str ){
    match v {
        Value::Object(ref m)=>
        {            
           for key in  m.keys(){
                printItem(&v[key],key);
           }
        }

        Value::String(ref s)=>{
            println!("{name} : {s}");
        }

        Value::Number(ref n)=>{
            println!("{name} : {n}");
        }

        Value::Array(ref a)=>{
            println!("{name} :");
            for item in a {
                printItem(item,name);
            }
        }
        _ => {}
     }
}