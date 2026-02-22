# mmq

`mmq` is a command-line tool for querying IP addresses and hostnames in MaxMind
MMDB (GeoIP2) databases.

## Building

Release:

```
cargo build --release
cargo b -r
```

Debug:

```
cargo build
cargo b
```

For more information on how to install the Rust toolchain refer to the official
Rust documentation.

## Options

There are a few flags to customize `mmq`:

```
Arguments:
  <HOSTNAME>...  Hosts to lookup

Options:
  -d, --database <PATH>  Path(s) to GeoIP databases; can be used multiple times
  -j, --jobs <N>         Number of parallel jobs (defaults to the number of CPUs)
  -n                     Do not resolve hostnames
  -c, --no-verify        Do not verify database integrity before lookups
  -h, --help             Print help (see more with '--help')
  -V, --version          Print version
```

## Examples

Query for `google.com`

```
$ mmq google.com -d <path to some mmdb>
-----------------------------------------------------------------142.250.130.139
ASN (Autonomous system number)                    : 15169
ASN Organization                                  : GOOGLE
Time zone                                         : America/Chicago
Country                                           : United States (US) (geoname id: 6252001)
Continent                                         : North America (NA) (geoname id: 6255149)
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
Registered country                                : United States (US) (geoname id: 6252001)
--------------------------------------------------------------------------------
-----------------------------------------------------------------142.250.130.138
Continent                                         : North America (NA) (geoname id: 6255149)
Country                                           : United States (US) (geoname id: 6252001)
Time zone                                         : America/Chicago
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
ASN (Autonomous system number)                    : 15169
ASN Organization                                  : GOOGLE
Registered country                                : United States (US) (geoname id: 6252001)
--------------------------------------------------------------------------------
-----------------------------------------------------------------142.250.130.102
ASN (Autonomous system number)                    : 15169
ASN Organization                                  : GOOGLE
Registered country                                : United States (US) (geoname id: 6252001)
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
Continent                                         : North America (NA) (geoname id: 6255149)
Country                                           : United States (US) (geoname id: 6252001)
Time zone                                         : America/Chicago
--------------------------------------------------------------------------------
-----------------------------------------------------------------142.250.130.101
ASN Organization                                  : GOOGLE
Continent                                         : North America (NA) (geoname id: 6255149)
Country                                           : United States (US) (geoname id: 6252001)
Time zone                                         : America/Chicago
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
Registered country                                : United States (US) (geoname id: 6252001)
ASN (Autonomous system number)                    : 15169
--------------------------------------------------------------------------------
-----------------------------------------------------------------142.250.130.100
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
Continent                                         : North America (NA) (geoname id: 6255149)
Country                                           : United States (US) (geoname id: 6252001)
ASN (Autonomous system number)                    : 15169
Time zone                                         : America/Chicago
Registered country                                : United States (US) (geoname id: 6252001)
ASN Organization                                  : GOOGLE
--------------------------------------------------------------------------------
-----------------------------------------------------------------142.250.130.113
ASN Organization                                  : GOOGLE
ASN (Autonomous system number)                    : 15169
Country                                           : United States (US) (geoname id: 6252001)
Continent                                         : North America (NA) (geoname id: 6255149)
Time zone                                         : America/Chicago
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
Registered country                                : United States (US) (geoname id: 6252001)
--------------------------------------------------------------------------------
```

Query for non existing address:

```
$ mmq localhost -d <path to mmdb>
No results found for 127.0.0.1
No results found for ::1
```

Query for multiple hosts:

```
$ mmq cloudflare.com microsoft.com -d <path to mmdb>
-----------------------------------------------------------------104.16.133.229
ASN Organization              : CLOUDFLARENET
Registered country            : United States (US) (geoname id: 6252001)
ASN (Autonomous system number): 13335
--------------------------------------------------------------------------------
-----------------------------------------------------------------104.16.132.229
ASN Organization              : CLOUDFLARENET
Registered country            : United States (US) (geoname id: 6252001)
ASN (Autonomous system number): 13335
--------------------------------------------------------------------------------
-----------------------------------------------------------------13.107.253.44
ASN Organization                                  : MICROSOFT-CORP-MSN-AS-BLOCK
Time zone                                         : America/Chicago
Continent                                         : North America (NA) (geoname id: 6255149)
Country                                           : United States (US) (geoname id: 6252001)
Registered country                                : United States (US) (geoname id: 6252001)
ASN (Autonomous system number)                    : 8075
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
--------------------------------------------------------------------------------
-----------------------------------------------------------------13.107.226.44
Registered country                                : United States (US) (geoname id: 6252001)
Time zone                                         : America/Chicago
ASN (Autonomous system number)                    : 8075
Country                                           : United States (US) (geoname id: 6252001)
ASN Organization                                  : MICROSOFT-CORP-MSN-AS-BLOCK
Approximate latitude/longitude and accuracy radius: 37.751/-97.822 (1000 km)
Continent                                         : North America (NA) (geoname id: 6255149)
--------------------------------------------------------------------------------
```
