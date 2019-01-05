use serde_json;

#[derive(Debug, Deserialize)]
#[serde(rename = "subnet")]
struct Subnet {
    location: String,
    range: String,
    defined: u64,
    used: u64,
    touched: u64,
    free: u64,
}

#[derive(Debug, Deserialize)]
struct DHCPDPool {
    subnets: Vec<Subnet>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let s = "{
	       \"subnets\": [
  	             { \"location\":\"Stanzino\", \"range\":\"10.1.0.30 - 10.1.0.180\", \"defined\":151, \"used\":0, \"touched\":1, \"free\":151 },
  	             { \"location\":\"Wifi.Guest\", \"range\":\"10.99.0.10 - 10.99.0.254\", \"defined\":245, \"used\":0, \"touched\":9, \"free\":245 },
  	             { \"location\":\"Pelucchi\", \"range\":\"10.100.0.30 - 10.100.0.199\", \"defined\":170, \"used\":4, \"touched\":7, \"free\":166 },
  	             { \"location\":\"Pelucchi\", \"range\":\"10.100.1.1 - 10.100.4.200\", \"defined\":968, \"used\":4, \"touched\":17, \"free\":964 },
  	             { \"location\":\"Pelucchi\", \"range\":\"10.100.6.1 - 10.100.7.200\", \"defined\":456, \"used\":0, \"touched\":0, \"free\":456 },
  	             { \"location\":\"VirtualBox\", \"range\":\"10.150.0.10 - 10.150.0.20\", \"defined\":11, \"used\":0, \"touched\":0, \"free\":11 }
  	       ],
  	       \"shared-networks\": [
  	             { \"location\":\"Stanzino\", \"defined\":151, \"used\":0, \"touched\":1, \"free\":151 },
  	             { \"location\":\"Pelucchi\", \"defined\":1594, \"used\":8, \"touched\":24, \"free\":1586 },
  	             { \"location\":\"Wifi.Guest\", \"defined\":245, \"used\":0, \"touched\":9, \"free\":245 },
  	             { \"location\":\"VirtualBox\", \"defined\":11, \"used\":0, \"touched\":0, \"free\":11 }
  	       ],
  	       \"summary\": {
  	             \"location\":\"All networks\",
  	             \"defined\":2001,
  	             \"used\":8,
  	             \"touched\":34,
  	             \"free\":1993
  	       }
  	    }";

        let dresp: DHCPDPool = serde_json::from_str(s).unwrap();

        assert_eq!(dresp.subnets.len(), 6);
        assert_eq!(dresp.subnets[1].defined, 245);
        assert_eq!(dresp.subnets[3].touched, 17);
    }
}
