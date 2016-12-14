#[macro_use] extern crate log;
extern crate env_logger;
extern crate rusoto;

use rusoto::{DefaultCredentialsProvider, Region};
use rusoto::cloudwatch::{CloudWatchClient, PutMetricDataInput, Dimension, MetricDatum};

fn get_metric_data() -> PutMetricDataInput {
    let metric_data = vec![MetricDatum {
        dimensions: Some(vec![Dimension {name: "foo".to_string(), value: "bar".to_string()}]),
        metric_name: "buffers".to_string(),
        statistic_values: None,
        timestamp: None,
        unit: Some("Bytes".to_string()),
        value: Some(1.0),
    }];
    PutMetricDataInput {
        namespace: "TestNamespace".to_string(),
        metric_data: metric_data,
    }
}

fn main() {
    env_logger::init().unwrap();

    let provider = DefaultCredentialsProvider::new().unwrap();
    let client = CloudWatchClient::new(provider, Region::UsEast1);

    let metric_input = get_metric_data();
    let result = client.put_metric_data(&metric_input);
    println!("result: {:?}!", result);
}
