use std::collections::HashMap;
use std::net::IpAddr;
use std::path::{Path, PathBuf};

use anyhow::bail;
use maxminddb as mmdb;

/// Holds multiple MaxMind MMDB readers keyed by file path.
///
/// Each database is memory-mapped and classified by its detected
/// database type (City, Country, ASN, etc.).
///
/// A single [`Lookup`] can query all loaded databases for a given IP.
#[derive(Debug)]
pub struct Lookup(HashMap<PathBuf, (DatabaseType, mmdb::Reader<mmdb::Mmap>)>);

impl Lookup {
    /// Opens all provided MMDB files using memory-mapped IO.
    ///
    /// Each database is classified based on its `database_type` metadata.
    ///
    /// # Safety
    /// The underlying database files MUST NOT be modified, truncated,
    /// or replaced while they are memory-mapped. Doing so results in
    /// undefined behavior, as required by [`maxminddb::Reader::open_mmap`].
    ///
    /// The caller must ensure the database files remain valid and
    /// immutable for the lifetime of the returned `Lookup`.
    pub fn open(paths: impl Iterator<Item = impl AsRef<Path>>) -> anyhow::Result<Self> {
        let mut readers = HashMap::new();

        for path in paths {
            // SAFETY:
            //   - The file is memory-mapped.
            //   - We rely on the caller to ensure the file is not modified,
            //     truncated, or replaced while mapped.
            //   - The Reader does not outlive the Lookup that owns it.
            let reader = unsafe {
                mmdb::Reader::open_mmap(path.as_ref()).map_err(|e| {
                    anyhow::anyhow!("Failed to open database {}: {e}", path.as_ref().display())
                })?
            };

            let db_type_str = reader.metadata.database_type.to_lowercase();
            let db_type = match db_type_str.as_str() {
                db_type_str if db_type_str.contains("city") => DatabaseType::City,
                db_type_str if db_type_str.contains("country") => DatabaseType::Country,
                db_type_str if db_type_str.contains("asn") => DatabaseType::Asn,
                db_type_str if db_type_str.contains("isp") => DatabaseType::Isp,
                db_type_str if db_type_str.contains("domain") => DatabaseType::Domain,
                db_type_str if db_type_str.contains("connection-type") => {
                    DatabaseType::ConnectionType
                }
                _ => bail!("Unsupported database type: {db_type_str}"),
            };

            log::info!(
                "Opened {db_type:?} database; path='{}'",
                path.as_ref().display()
            );

            readers.insert(path.as_ref().to_path_buf(), (db_type, reader));
        }

        Ok(Self(readers))
    }

    /// Verifies integrity of all loaded databases.
    ///
    /// This performs MaxMind's internal structural validation and might take
    /// some time to finish.
    pub fn verify_dbs(&self) -> anyhow::Result<()> {
        for (db_type, reader) in self.0.values() {
            log::info!("Verifying {db_type:?} database");
            reader.verify()?;
        }
        Ok(())
    }

    /// Performs lookup of `addr` in all loaded databases.
    ///
    /// The result aggregates data from each database type.
    /// Missing entries are represented as `None`.
    pub fn get(&self, addr: IpAddr) -> anyhow::Result<LookupResult<'_>> {
        let mut result = LookupResult::new();

        // Check `addr` in each loaded DB
        for (db_type, reader) in self.0.values() {
            log::debug!("Looking up {addr} in {db_type:?} database");

            let lookedup_addr = reader.lookup(addr)?;

            match db_type {
                DatabaseType::City => result.city = lookedup_addr.decode()?,
                DatabaseType::Country => result.country = lookedup_addr.decode()?,
                DatabaseType::Asn => result.asn = lookedup_addr.decode()?,
                DatabaseType::Isp => result.isp = lookedup_addr.decode()?,
                DatabaseType::Domain => result.domain = lookedup_addr.decode()?,
                DatabaseType::ConnectionType => result.connection_type = lookedup_addr.decode()?,
            }
        }

        if result.is_empty() {
            log::warn!("{addr} not found in any database");
        } else {
            log::info!("Got results for {addr}");
        }

        Ok(result)
    }
}

/// Aggregated lookup result.
#[derive(Debug, Default)]
pub struct LookupResult<'a> {
    city: Option<mmdb::geoip2::City<'a>>,
    country: Option<mmdb::geoip2::Country<'a>>,
    asn: Option<mmdb::geoip2::Asn<'a>>,
    isp: Option<mmdb::geoip2::Isp<'a>>,
    domain: Option<mmdb::geoip2::Domain<'a>>,
    connection_type: Option<mmdb::geoip2::ConnectionType<'a>>,
}

impl<'a> LookupResult<'a> {
    fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.city.is_none()
            && self.country.is_none()
            && self.asn.is_none()
            && self.isp.is_none()
            && self.domain.is_none()
            && self.connection_type.is_none()
    }

    pub fn city(&self) -> Option<&mmdb::geoip2::City<'_>> {
        self.city.as_ref()
    }

    pub fn country(&self) -> Option<&mmdb::geoip2::Country<'_>> {
        self.country.as_ref()
    }

    pub fn asn(&self) -> Option<&mmdb::geoip2::Asn<'_>> {
        self.asn.as_ref()
    }

    pub fn isp(&self) -> Option<&mmdb::geoip2::Isp<'_>> {
        self.isp.as_ref()
    }

    pub fn domain(&self) -> Option<&mmdb::geoip2::Domain<'_>> {
        self.domain.as_ref()
    }

    pub fn connection_type(&self) -> Option<&mmdb::geoip2::ConnectionType<'_>> {
        self.connection_type.as_ref()
    }
}

/// Represents supported MMDB database types.
///
/// Marked as non_exhaustive to allow future extension
/// without breaking public API.
#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
enum DatabaseType {
    City,
    Country,
    Asn,
    Isp,
    Domain,
    ConnectionType,
}

pub fn open(paths: impl Iterator<Item = impl AsRef<Path>>) -> anyhow::Result<Lookup> {
    Lookup::open(paths)
}
