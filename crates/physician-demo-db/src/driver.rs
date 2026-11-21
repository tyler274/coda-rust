//! SQLite execution driver for native (`rusqlite`) and browser (`sql.js`) targets.

use sea_query::{
    DeleteStatement, Expr, ExprTrait, Iden, InsertStatement, Query, SelectStatement, SimpleExpr,
    SqliteQueryBuilder, UpdateStatement, Value, Values,
};
use serde_json::Value as JsonValue;
use std::collections::BTreeMap;

/// Errors produced while opening, querying, seeding, or resetting the demo database.
#[derive(Debug, thiserror::Error)]
pub enum DbError {
    /// Failed to decode JSON payloads stored in or returned from SQLite.
    #[error("demo SQLite JSON error")]
    Json(#[from] serde_json::Error),
    /// Native SQLite failure.
    #[cfg(not(target_arch = "wasm32"))]
    #[error("native SQLite error")]
    Sqlite(#[from] rusqlite::Error),
    /// Browser SQLite bridge failure.
    #[cfg(target_arch = "wasm32")]
    #[error("browser SQLite error: {0}")]
    Browser(String),
    /// Missing expected column in query result row.
    #[error("missing SQLite column `{column}`")]
    MissingColumn {
        /// Column name expected by the typed store.
        column: &'static str,
    },
    /// Requested entity does not exist in the demo database.
    #[error("{kind} `{id}` not found")]
    NotFound {
        /// Domain entity label.
        kind: &'static str,
        /// Entity identifier.
        id: String,
    },
    /// Invalid command payload sent to the demo store.
    #[error("{0}")]
    InvalidCommand(String),
    /// Optimistic version or state conflict preventing mutation.
    #[error("{0}")]
    Conflict(String),
}

/// Typed SQL parameter converted from `sea_query::Value`.
#[derive(Debug, Clone, serde::Serialize)]
#[serde(untagged)]
pub enum SqlParam {
    /// SQL `NULL`.
    Null,
    /// SQL integer.
    Int(i64),
    /// SQL floating-point number.
    Real(f64),
    /// SQL UTF-8 text.
    Text(String),
}

impl From<&Value> for SqlParam {
    fn from(value: &Value) -> Self {
        match value {
            Value::Bool(Some(b)) => Self::Int(i64::from(*b)),
            Value::TinyInt(Some(n)) => Self::Int(i64::from(*n)),
            Value::SmallInt(Some(n)) => Self::Int(i64::from(*n)),
            Value::Int(Some(n)) => Self::Int(i64::from(*n)),
            Value::BigInt(Some(n)) => Self::Int(*n),
            Value::TinyUnsigned(Some(n)) => Self::Int(i64::from(*n)),
            Value::SmallUnsigned(Some(n)) => Self::Int(i64::from(*n)),
            Value::Unsigned(Some(n)) => Self::Int(i64::from(*n)),
            Value::BigUnsigned(Some(n)) => Self::Int(i64::try_from(*n).unwrap_or(i64::MAX)),
            Value::Float(Some(f)) => Self::Real(f64::from(*f)),
            Value::Double(Some(f)) => Self::Real(*f),
            Value::String(Some(s)) => Self::Text(s.to_string()),
            Value::Char(Some(c)) => Self::Text(c.to_string()),
            _ => Self::Null,
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl rusqlite::ToSql for SqlParam {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        use rusqlite::types::{ToSqlOutput, Value as RVal, ValueRef};
        Ok(match self {
            Self::Null => ToSqlOutput::Borrowed(ValueRef::Null),
            Self::Int(n) => ToSqlOutput::Owned(RVal::Integer(*n)),
            Self::Real(f) => ToSqlOutput::Owned(RVal::Real(*f)),
            Self::Text(s) => ToSqlOutput::Borrowed(ValueRef::Text(s.as_bytes())),
        })
    }
}

fn bind_values(values: Values) -> Vec<SqlParam> {
    values.0.iter().map(SqlParam::from).collect()
}

/// A single row returned by SQLite, accessed through typed column getters.
#[derive(Debug, Clone, serde::Deserialize)]
#[serde(transparent)]
pub struct SqlRow(BTreeMap<String, JsonValue>);

impl SqlRow {
    /// Reads a required UTF-8 string column.
    pub fn get_str(&self, column: &'static str) -> Result<String, DbError> {
        match self.0.get(column) {
            Some(JsonValue::String(s)) => Ok(s.clone()),
            Some(JsonValue::Number(n)) => Ok(n.to_string()),
            _ => Err(DbError::MissingColumn { column }),
        }
    }

    /// Reads a required integer column.
    pub fn get_i64(&self, column: &'static str) -> Result<i64, DbError> {
        match self.0.get(column) {
            Some(JsonValue::Number(n)) => n.as_i64().ok_or(DbError::MissingColumn { column }),
            _ => Err(DbError::MissingColumn { column }),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod imp {
    use super::*;
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};

    #[derive(Clone)]
    pub struct SqliteBackend {
        conn: Arc<Mutex<rusqlite::Connection>>,
        reset_paths: Arc<Mutex<Option<(PathBuf, PathBuf)>>>,
    }

    impl std::fmt::Debug for SqliteBackend {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("SqliteBackend").finish_non_exhaustive()
        }
    }

    impl SqliteBackend {
        fn wrap(conn: rusqlite::Connection) -> Self {
            Self {
                conn: Arc::new(Mutex::new(conn)),
                reset_paths: Arc::new(Mutex::new(None)),
            }
        }
        pub fn open_in_memory() -> Result<Self, DbError> {
            Ok(Self::wrap(rusqlite::Connection::open_in_memory()?))
        }
        pub fn open_file(path: &Path) -> Result<Self, DbError> {
            Ok(Self::wrap(rusqlite::Connection::open(path)?))
        }
        pub fn bind_reset_files(&self, seed_path: PathBuf, active_path: PathBuf) {
            *self.reset_paths.lock().expect("reset lock") = Some((seed_path, active_path));
        }

        pub fn exec_raw(&self, sql: &str, params: &[SqlParam]) -> Result<(), DbError> {
            let guard = self.conn.lock().expect("sqlite lock");
            let refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
            guard.execute(sql, refs.as_slice())?;
            Ok(())
        }

        pub fn query_raw(&self, sql: &str, params: &[SqlParam]) -> Result<Vec<SqlRow>, DbError> {
            use rusqlite::types::ValueRef as V;
            let guard = self.conn.lock().expect("sqlite lock");
            let mut stmt = guard.prepare(sql)?;
            let cols: Vec<String> = stmt
                .column_names()
                .iter()
                .map(|n| (*n).to_owned())
                .collect();
            let refs: Vec<&dyn rusqlite::ToSql> =
                params.iter().map(|p| p as &dyn rusqlite::ToSql).collect();
            let mut rows = stmt.query(refs.as_slice())?;
            let mut out = Vec::new();
            while let Some(row) = rows.next()? {
                let mut map = BTreeMap::new();
                for (i, col) in cols.iter().enumerate() {
                    let val = match row.get_ref(i)? {
                        V::Null | V::Blob(_) => JsonValue::Null,
                        V::Integer(n) => JsonValue::from(n),
                        V::Real(f) => JsonValue::from(f),
                        V::Text(b) => JsonValue::String(String::from_utf8_lossy(b).into_owned()),
                    };
                    map.insert(col.clone(), val);
                }
                out.push(SqlRow(map));
            }
            Ok(out)
        }

        pub async fn reset(&self) -> Result<(), DbError> {
            let Some((seed, active)) = self.reset_paths.lock().expect("reset lock").clone() else {
                return Ok(());
            };
            let mut guard = self.conn.lock().expect("sqlite lock");
            let _ = std::fs::copy(&seed, &active);
            *guard = rusqlite::Connection::open(&active)?;
            Ok(())
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod imp {
    use super::*;
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/assets/sqlite_bridge.js")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = "initDemoDb")]
        async fn js_init_demo_db(db_url: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_name = "querySql")]
        fn js_query_sql(sql: &str, params_json: &str) -> Result<String, JsValue>;
        #[wasm_bindgen(catch, js_name = "execSql")]
        fn js_exec_sql(sql: &str, params_json: &str) -> Result<(), JsValue>;
        #[wasm_bindgen(catch, js_name = "resetDemoDb")]
        async fn js_reset_demo_db() -> Result<(), JsValue>;
    }

    fn js_error(err: JsValue) -> DbError {
        DbError::Browser(err.as_string().unwrap_or_else(|| format!("{err:?}")))
    }

    #[derive(Debug, Clone)]
    pub struct SqliteBackend {
        db_url: String,
    }

    impl SqliteBackend {
        pub fn new(db_url: String) -> Self {
            Self { db_url }
        }
        pub async fn ensure_ready(&self) -> Result<(), DbError> {
            js_init_demo_db(&self.db_url).await.map_err(js_error)
        }
        pub fn exec_raw(&self, sql: &str, params: &[SqlParam]) -> Result<(), DbError> {
            js_exec_sql(sql, &serde_json::to_string(params)?).map_err(js_error)
        }
        pub fn query_raw(&self, sql: &str, params: &[SqlParam]) -> Result<Vec<SqlRow>, DbError> {
            let raw = js_query_sql(sql, &serde_json::to_string(params)?).map_err(js_error)?;
            Ok(serde_json::from_str(&raw)?)
        }
        pub async fn reset(&self) -> Result<(), DbError> {
            js_reset_demo_db().await.map_err(js_error)
        }
    }
}

/// Platform-unified SQLite driver executing `sea-query` statements with bound parameters.
#[derive(Debug, Clone)]
pub struct SqliteDriver {
    backend: imp::SqliteBackend,
}

impl SqliteDriver {
    /// Opens an in-memory SQLite database (native targets and tests).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_in_memory() -> Result<Self, DbError> {
        Ok(Self {
            backend: imp::SqliteBackend::open_in_memory()?,
        })
    }

    /// Opens or creates a SQLite file at `path` (native seeding binary).
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_file(path: &std::path::Path) -> Result<Self, DbError> {
        Ok(Self {
            backend: imp::SqliteBackend::open_file(path)?,
        })
    }

    /// Creates a browser SQLite driver backed by `db_url` (`sql.js`).
    #[cfg(target_arch = "wasm32")]
    pub fn from_browser_url(db_url: impl Into<String>) -> Self {
        Self {
            backend: imp::SqliteBackend::new(db_url.into()),
        }
    }

    /// Ensures the browser SQLite runtime and seed database have loaded.
    pub async fn ensure_ready(&self) -> Result<(), DbError> {
        #[cfg(target_arch = "wasm32")]
        {
            self.backend.ensure_ready().await
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            Ok(())
        }
    }

    /// Executes a `sea-query` `SELECT` statement and returns typed rows.
    pub fn select(&self, query: &SelectStatement) -> Result<Vec<SqlRow>, DbError> {
        let (sql, values) = query.build(SqliteQueryBuilder);
        self.backend.query_raw(&sql, &bind_values(values))
    }

    /// Executes a `sea-query` `INSERT` statement.
    pub fn insert(&self, query: &InsertStatement) -> Result<(), DbError> {
        let (sql, values) = query.build(SqliteQueryBuilder);
        self.backend.exec_raw(&sql, &bind_values(values))
    }

    /// Inserts one row into `table` for `cols` and `vals`.
    pub fn insert_row<T: Iden + 'static, C: Iden + Copy + 'static>(
        &self,
        table: T,
        cols: &[C],
        vals: impl IntoIterator<Item = SimpleExpr>,
    ) -> Result<(), DbError> {
        let mut stmt = Query::insert();
        stmt.into_table(table)
            .columns(cols.iter().copied())
            .values_panic(vals);
        self.insert(&stmt)
    }

    /// Executes a `sea-query` `UPDATE` statement.
    pub fn update(&self, query: &UpdateStatement) -> Result<(), DbError> {
        let (sql, values) = query.build(SqliteQueryBuilder);
        self.backend.exec_raw(&sql, &bind_values(values))
    }

    /// Executes a `sea-query` `DELETE` statement.
    pub fn delete(&self, query: &DeleteStatement) -> Result<(), DbError> {
        let (sql, values) = query.build(SqliteQueryBuilder);
        self.backend.exec_raw(&sql, &bind_values(values))
    }

    /// Deletes rows from `table` matching all equality `conds`.
    pub fn delete_where<T: Iden + 'static, K: Iden + Copy + 'static>(
        &self,
        table: T,
        conds: &[(K, &str)],
    ) -> Result<(), DbError> {
        let mut q = Query::delete();
        q.from_table(table);
        for (k, v) in conds {
            q.and_where(Expr::col(*k).eq(*v));
        }
        self.delete(&q)
    }

    /// Selects `cols` from `table` matching all equality `conds`.
    pub fn select_where<T: Iden + 'static, C: Iden + Copy + 'static, K: Iden + Copy + 'static>(
        &self,
        table: T,
        cols: &[C],
        conds: &[(K, &str)],
    ) -> Result<Vec<SqlRow>, DbError> {
        let mut q = Query::select();
        q.columns(cols.iter().copied()).from(table);
        for (col, val) in conds {
            q.and_where(Expr::col(*col).eq(*val));
        }
        self.select(&q)
    }

    /// Updates `col = val` in `table` matching all equality `conds`.
    pub fn update_where<T: Iden + 'static, C: Iden + 'static, K: Iden + Copy + 'static>(
        &self,
        table: T,
        col: C,
        val: SimpleExpr,
        conds: &[(K, &str)],
    ) -> Result<(), DbError> {
        let mut q = Query::update();
        q.table(table).value(col, val);
        for (k, v) in conds {
            q.and_where(Expr::col(*k).eq(*v));
        }
        self.update(&q)
    }

    /// Executes DDL schema initialization statements.
    pub fn exec_ddl(&self, ddl: &[String]) -> Result<(), DbError> {
        for stmt in ddl {
            self.backend.exec_raw(stmt, &[])?;
        }
        Ok(())
    }

    /// Binds a seed file and active file path so native tests can reset cleanly.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn bind_reset_files(&self, seed_path: std::path::PathBuf, active_path: std::path::PathBuf) {
        self.backend.bind_reset_files(seed_path, active_path);
    }

    /// Resets the demo database back to its initial baseline snapshot.
    pub async fn reset(&self) -> Result<(), DbError> {
        self.backend.reset().await
    }
}
