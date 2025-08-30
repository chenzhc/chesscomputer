#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables, unused_mut))]
#![allow(dead_code)]
#![allow(unused_variables)]

use datafusion::prelude::*;



#[cfg(test)]
mod tests {
    use log::info;
    use datafusion::prelude::*;

    use super::*;

    #[tokio::test]
    async fn it_test01() -> datafusion::error::Result<()>  {
        crate::init();

        info!("test");

         // register the table
        let ctx = SessionContext::new();
        ctx.register_csv("example", "tests/data/example.csv", CsvReadOptions::new()).await?;

        // create a plan to run a SQL query
        let df = ctx.sql("SELECT a, MIN(b) FROM example WHERE a <= b GROUP BY a LIMIT 100").await?;

        // execute and print results
        df.show().await?;

        Ok(())
    }
}