use std::error::Error;
use crate::structer::Proxy;
use rusqlite::{params, Connection, Result};

pub fn initialize_db() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open("proxies.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS all_proxies (
            id INTEGER PRIMARY KEY,
            ip TEXT NOT NULL,
            port TEXT NOT NULL,
            proxy_type TEXT NOT NULL,
            country TEXT NOT NULL,
            last_checked TEXT NOT NULL,
            check_number INTEGER NOT NULL,
            live BOOLEAN NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS live_proxies AS 
         SELECT * FROM all_proxies WHERE live = TRUE",
        [],
    )?;

    Ok(())
}

pub fn store_proxies(proxies: Vec<Proxy>) -> Result<(), Box<dyn Error>> {
    let mut conn = Connection::open("proxies.db")?;

    let tx = conn.transaction()?;

    {
        let mut update_stmt = tx.prepare(
            "UPDATE all_proxies SET last_checked = ?1, check_number = check_number + 1, live = ?2 WHERE ip = ?3 AND port = ?4",
        )?;
        let mut insert_stmt = tx.prepare(
            "INSERT INTO all_proxies (ip, port, proxy_type, country, last_checked, check_number, live) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        )?;

        for proxy in proxies.iter() {
            let updated = update_stmt.execute(params![proxy.last_checked, proxy.live, proxy.ip, proxy.port])?;

            if updated == 0 {
                insert_stmt.execute(params![proxy.ip, proxy.port, proxy.proxy_type, proxy.country, proxy.last_checked, proxy.check_number, proxy.live])?;
            }
        }
    } 


    tx.commit()?;

    Ok(())
}

pub fn live_proxies_db_update() -> Result<(),Box<dyn Error>> {
    let conn = Connection::open("proxies.db")?;
    let mut stmt = conn.prepare("SELECT ip, port, proxy_type, country, last_checked, check_number, live FROM all_proxies WHERE live = TRUE")?;
    let proxy_iter = stmt.query_map([], |row| {
        Ok(Proxy {
            ip: row.get(0)?,
            port: row.get(1)?,
            proxy_type: row.get(2)?,
            country: row.get(3)?,
            last_checked: row.get(4)?,
            check_number: row.get(5)?,
            live: row.get(6)?,
        })
    })?;

    conn.execute("DELETE FROM live_proxies", [])?;

    for proxy in proxy_iter {
        let proxy = proxy?;
        conn.execute(
            "INSERT INTO live_proxies (ip, port, proxy_type, country, last_checked, check_number, live) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![proxy.ip, proxy.port, proxy.proxy_type, proxy.country, proxy.last_checked, proxy.check_number, proxy.live],
        )?;
    }
    Ok(())
}

pub fn fetch_all_proxies() -> Result<Vec<Proxy>, Box<dyn Error>> {
    let conn = Connection::open("proxies.db")?;
    let mut stmt = conn.prepare("SELECT id, ip, port, proxy_type, country, last_checked, check_number, live FROM all_proxies")?;
    let proxy_iter = stmt.query_map(params![], |row| {
        Ok(Proxy {
            ip: row.get(1)?,
            port: row.get(2)?,
            proxy_type: row.get(3)?,
            country: row.get(4)?,
            last_checked: row.get(5)?,
            check_number: row.get(6)?,
            live: row.get(7)?,
        })
    })?;

    let mut proxies = Vec::new();
    for proxy in proxy_iter {
        proxies.push(proxy?);
    }

    Ok(proxies)
}