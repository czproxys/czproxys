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
            live_number INTEGER NOT NULL,
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
            "UPDATE all_proxies SET last_checked = ?1, check_number = check_number + 1, live = ?2, live_number = ?3 WHERE ip = ?4 AND port = ?5",
        )?;
        let mut insert_stmt = tx.prepare(
            "INSERT INTO all_proxies (ip, port, proxy_type, country, last_checked, check_number, live_number, live) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        )?;

        for proxy in proxies.iter() {
            let updated = update_stmt.execute(params![proxy.last_checked, proxy.live, proxy.live_number, proxy.ip, proxy.port])?;

            if updated == 0 {
                insert_stmt.execute(params![proxy.ip, proxy.port, proxy.proxy_type, proxy.country, proxy.last_checked, proxy.check_number, proxy.live_number, proxy.live])?;
            }
        }
    } 


    tx.commit()?;

    Ok(())
}

pub fn live_proxies_db_update() -> Result<usize,Box<dyn Error>> {
    let conn = Connection::open("proxies.db")?;

    let cnt = conn.execute(
        "DELETE FROM live_proxies;",
        [],
    )?; 
    println!("DELETE FROM live_proxies {cnt}");
    let cnt = conn.execute(
        "
        INSERT INTO live_proxies (ip, port, proxy_type, country, last_checked, check_number, live_number, live)
        SELECT ip, port, proxy_type, country, last_checked, check_number, live_number, live 
        FROM all_proxies WHERE live = TRUE",
        [],
    )?;
    println!("INSERT INTO live_proxies - {cnt}");
    
    /*let mut stmt = conn.prepare(
        "
        SELECT Count(*) FROM live_proxies;"
    )?;
    let cnt = stmt.query([])?.next()?.unwrap().get(0)?;
    println!("SELECT * FROM live_proxies - {cnt}");*/


    Ok(cnt)
}

pub fn fetch_all_proxies() -> Result<Vec<Proxy>, Box<dyn Error>> {
    let conn = Connection::open("proxies.db")?;
    let mut stmt = conn.prepare("SELECT id, ip, port, proxy_type, country, last_checked, check_number, live_number, live FROM all_proxies")?;
    let proxy_iter = stmt.query_map(params![], |row| {
        Ok(Proxy {
            ip: row.get(1)?,
            port: row.get(2)?,
            proxy_type: row.get(3)?,
            country: row.get(4)?,
            last_checked: row.get(5)?,
            check_number: row.get(6)?,
            live_number: row.get(7)?,
            live: row.get(8)?,
        })
    })?;

    let mut proxies = Vec::new();
    for proxy in proxy_iter {
        proxies.push(proxy?);
    }

    Ok(proxies)
}