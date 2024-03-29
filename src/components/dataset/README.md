# Datasets

The Datasets are similar to the QRadar reference sets. They store information such as IP, IOC ... and can be completed almost in real time from the information collected from the events.
A Dataset is synchronized across multiple SIEM instances using a PUB / SUB pattern. Each instance, when started, loads the entire dataset from the DDBB and subscribes to changes to that particular dataset.
Only delta changes (patch) are sent, but we can also do a full update. 

The [Datasets](https://github.com/u-siem/u-siem-core/blob/a4009b5d759263b4a9b7a6ca1c2a8d36d391bfef/src/components/dataset/mod.rs#L24) are fixed to simplify the development of new modules: 
* GeoIp to map a IP to a country, city...
* IpHost: Association of a IP with a hostname (dynamically populated)
* IpMac: Association of a IP with a MAC address (populated using DHCP logs)
* HostUser: Association of a workstation with the user it belongs to.
* HostMac: If we know the MAC address of each workstation it will help us enchance DHCP logs.
* BlockIp: EDL to be blocked in the firewall 
* IpCloudService: Each Cloud service publish the networks it has daily. This will enchance firewall logs for example.
* IpCloudProvider: To detect connections to Cloud Providers like a IP that belongs to AWS.

It also let us define custom datasets:
* CustomMapText: It has associated a name and a Map of pairs Key-Value(String).
* CustomTextList: Key-Value maps where the value is a list of strings.
...

### Multi-Tenant
The datasets don't support multiple tenants. For that reason a cluster of uSIEM nodes must be dedicated to each client exclusivly, it also helps to calculate the cost for each SOC client.

#### About BlockLists
The blocklists are intended to be used internally by the SIEM not directly by an operator. So, to add an element to the block list, a component must implement the appropiate command, like FILTER_IP.

### Slow GeoIP
Instead of having the full GeoIP database in memory, we can use the `slow_geoip` to have the GeoIP dataset in a Sled database on disk.

We can get one million requests on the slow dataset in 14.2673 seconds => 0.0142673 ms/req with a loading time of 31.714024 the first time.
For the fast version we get 0.45017052 seconds => 0.00045017052 ms/req 32 times faster, but also it requires 13 seconds to load the dataset in memory with each change.

## TODO List

- [x] Block list email sender
- [x] DNS-IP association dataset. If we have DNS logs, we can populate this dataset with the real IPs associated with that domain (If we try to resolve them in another place it may be different).
- [ ] Datasets for WebServers: WebHosted (name of the app hosted in the server); WebAppTechnology (If the APP uses PHP, .NET, JavaScript, Java...) this is useful to detect SCANNERS; SlowWebRequest if some request like "/download/all_logs" are always slow, we can tag them as to not interfere with the rules used to detect BlindSQL injections that uses the sleep command or Web exploits that downloads excessive data.
- [x] Authentication dataset: pairs user/secret only to be used by user facing components: inputs, outputs, comander, database...
- [ ] uSIEM user dataset: Local auth for uSIEM.
- [ ] Dataset with the list of hosts that send logs and the last time they did it.