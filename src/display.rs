use std::collections::HashMap;
use std::io::{self, Write};
use std::net::IpAddr;

use color_print::cwriteln;
use maxminddb::geoip2;

use crate::lookup::LookupResult;

pub fn print_lookup_result(
    writer: &mut impl Write,
    addr: &IpAddr,
    result: &LookupResult<'_>,
) -> io::Result<()> {
    let mut fields = HashMap::new();

    if let Some(city) = result.city() {
        add_city_fields(&mut fields, city);
    }

    // The `city` result already includes `country`, so printing both
    // is pointless.
    if let Some(country) = result.country()
        && result.city().is_none()
    {
        add_country_fields(&mut fields, country);
    }

    if let Some(asn) = result.asn() {
        add_asn_fields(&mut fields, asn);
    }

    if let Some(isp) = result.isp() {
        add_isp_fields(&mut fields, isp);
    }

    if let Some(domain) = result.domain() {
        add_domain_fields(&mut fields, domain);
    }

    if let Some(connection_type) = result.connection_type() {
        add_connection_type_fields(&mut fields, connection_type);
    }

    cwriteln!(writer, "<k!>{:-<65}</><g><s>{addr}</></>", "-")?;

    let longest = fields.keys().map(|k| k.len()).max().unwrap_or(0);

    for key in fields.keys() {
        let value = &fields[key];

        if *key == "Subdivisions" {
            cwriteln!(
                writer,
                "<c><s>{:<width$}</></><k!>:</>",
                key,
                width = longest
            )?;
            for sub in value.split(", ") {
                writeln!(writer, "  + {}", sub)?;
            }
        } else {
            cwriteln!(
                writer,
                "<c><s>{:<width$}</></><k!>:</> {}",
                key,
                value,
                width = longest
            )?;
        }
    }

    cwriteln!(writer, "<k!>{:-<80}</>", "-")?;

    Ok(())
}

fn add_city_fields(fields: &mut HashMap<&str, String>, city: &geoip2::City<'_>) {
    // City
    if let Some(name) = city.city.names.english {
        let value = if let Some(id) = city.city.geoname_id {
            format!("{} (geoname id: {})", name, id)
        } else {
            name.to_string()
        };
        fields.insert("City", value);
    }

    // Continent
    if let Some(name) = city.continent.names.english {
        let mut value = name.to_string();
        if let Some(code) = city.continent.code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = city.continent.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Continent", value);
    }

    // Country
    if let Some(name) = city.country.names.english {
        let mut value = name.to_string();
        if let Some(code) = city.country.iso_code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = city.country.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Country", value);
    }

    // Time zone
    if let Some(time_zone) = city.location.time_zone {
        fields.insert("Time zone", time_zone.to_string());
    }

    // Latitude / Longitude / Accuracy radius
    if let (Some(lat), Some(long), Some(ar)) = (
        city.location.latitude,
        city.location.longitude,
        city.location.accuracy_radius,
    ) {
        fields.insert(
            "Approximate latitude/longitude and accuracy radius",
            format!("{}/{} ({} km)", lat, long, ar),
        );
    }

    // Postal code
    if let Some(code) = city.postal.code {
        fields.insert("Postal code", code.to_string());
    }

    // Registered country
    if let Some(name) = city.registered_country.names.english {
        let mut value = name.to_string();
        if let Some(code) = city.registered_country.iso_code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = city.registered_country.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Registered country", value);
    }

    // Represented country
    if let Some(name) = city.represented_country.names.english {
        let mut value = name.to_string();
        if let Some(code) = city.represented_country.iso_code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = city.represented_country.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Represented country", value);
    }

    // Representation type
    if let Some(ty) = city.represented_country.representation_type {
        fields.insert("Representation type", ty.to_string());
    }

    // Subdivisions
    if !city.subdivisions.is_empty() {
        let mut subdivision_lines = Vec::new();
        for subdivision in &city.subdivisions {
            if let Some(name) = subdivision.names.english {
                let mut line = name.to_string();
                if let Some(code) = subdivision.iso_code {
                    line.push_str(&format!(" ({})", code));
                }
                if let Some(id) = subdivision.geoname_id {
                    line.push_str(&format!(" (geoname id: {})", id));
                }
                subdivision_lines.push(line);
            }
        }
        if !subdivision_lines.is_empty() {
            fields.insert("Subdivisions", subdivision_lines.join(", "));
        }
    }

    // Traits
    if let Some(is_anycast) = city.traits.is_anycast {
        fields.insert("Anycast", if is_anycast { "yes" } else { "no" }.to_string());
    }
}

fn add_country_fields(fields: &mut HashMap<&str, String>, country: &geoip2::Country<'_>) {
    // Continent
    if let Some(name) = country.continent.names.english {
        let mut value = name.to_string();
        if let Some(code) = country.continent.code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = country.continent.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Continent", value);
    }

    // Country
    if let Some(name) = country.country.names.english {
        let mut value = name.to_string();
        if let Some(code) = country.country.iso_code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = country.country.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Country", value);
    }

    // Registered country
    if let Some(name) = country.registered_country.names.english {
        let mut value = name.to_string();
        if let Some(code) = country.registered_country.iso_code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = country.registered_country.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Registered country", value);
    }

    // Represented country
    if let Some(name) = country.represented_country.names.english {
        let mut value = name.to_string();
        if let Some(code) = country.represented_country.iso_code {
            value.push_str(&format!(" ({})", code));
        }
        if let Some(id) = country.represented_country.geoname_id {
            value.push_str(&format!(" (geoname id: {})", id));
        }
        fields.insert("Represented country", value);
    }

    // Representation type
    if let Some(ty) = country.represented_country.representation_type {
        fields.insert("Representation type", ty.to_string());
    }

    // Traits
    if let Some(is_anycast) = country.traits.is_anycast {
        fields.insert("Anycast", if is_anycast { "yes" } else { "no" }.to_string());
    }
}

fn add_asn_fields(fields: &mut HashMap<&str, String>, asn: &geoip2::Asn<'_>) {
    if let Some(num) = asn.autonomous_system_number {
        fields.insert("ASN (Autonomous system number)", num.to_string());
    }

    if let Some(org) = asn.autonomous_system_organization {
        fields.insert("ASN Organization", org.to_owned());
    }
}

fn add_isp_fields(fields: &mut HashMap<&str, String>, isp: &geoip2::Isp<'_>) {
    if let Some(isp) = isp.isp {
        fields.insert("ISP (Internet Service Provider)", isp.to_owned());
    }

    if let Some(org) = isp.organization {
        fields.insert("ISP Organization", org.to_owned());
    }

    if let Some(mobile_country_code) = isp.mobile_country_code {
        fields.insert("Mobile country code", mobile_country_code.to_owned());
    }

    if let Some(mobile_network_code) = isp.mobile_network_code {
        fields.insert("Mobile network code", mobile_network_code.to_owned());
    }
}

fn add_domain_fields(fields: &mut HashMap<&str, String>, domain: &geoip2::Domain<'_>) {
    if let Some(domain) = domain.domain {
        fields.insert("Domain", domain.to_owned());
    }
}

fn add_connection_type_fields(
    fields: &mut HashMap<&str, String>,
    connection_type: &geoip2::ConnectionType<'_>,
) {
    if let Some(cty) = connection_type.connection_type {
        fields.insert("Connection type", cty.to_owned());
    }
}
