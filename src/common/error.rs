//! Error handling module for Kitx database operations.
//!
//! This module provides error types for database operations,
//! including query errors and relation errors.
//! All errors implement proper error traits for integration with sqlx.
//!
//! Kitx数据库操作的错误处理模块。
//!
//! 此模块为数据库操作提供错误类型，包括查询错误和关联错误。
//! 所有错误都实现了适当的错误trait，以便与sqlx集成。

use sqlx::Error as SqlxError;
use sqlx::error::{DatabaseError, ErrorKind};
use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result};

/// Main error type for Kitx operations.
///
/// This enum represents error scenarios that can occur during
/// database query construction and execution.
///
/// # Variants
/// - `DBPoolNotInitialized`: Database connection pool is not initialized
/// - `Other`: Generic error with custom message
///
/// Kitx操作的主要错误类型。
///
/// 此枚举表示在数据库查询构建和执行期间可能发生的错误场景。
///
/// # 变体
/// - `DBPoolNotInitialized`: 数据库连接池未初始化
/// - `Other`: 带有自定义消息的通用错误
#[derive(Debug)]
pub enum QueryError {
    /// Database pool is not initialized / 数据库连接池未初始化
    DBPoolNotInitialized,
    /// Generic error with custom message / 带有自定义消息的通用错误
    Other(String),
}

/// Relation-specific error types for handling entity relationships.
///
/// This enum handles errors that occur when working with entity relationships,
/// such as foreign key constraints and value matching between related entities.
///
/// # Variants
/// - `ValueEmpty`: Expected non-empty values but got empty collection
/// - `ValueMismatch`: Value type or content mismatch between expected and actual
///
/// 处理实体关系的关联特定错误类型。
///
/// 此枚举处理在处理实体关系时发生的错误，
/// 如外键约束和相关实体之间的值匹配。
///
/// # 变体
/// - `ValueEmpty`: 期望非空值但得到空集合
/// - `ValueMismatch`: 期望值与实际值的类型或内容不匹配
#[derive(Debug)]
pub enum RelationError {
    /// Expected non-empty values but got empty collection / 期望非空值但得到空集合
    ValueEmpty(usize),
    /// Value mismatch between expected and actual / 期望值与实际值不匹配
    ValueMismatch(usize, String, String),
}

impl QueryError {
    /// Returns a descriptive error message for the query error.
    ///
    /// This method provides human-readable error messages that can be
    /// displayed to users or logged for debugging purposes.
    ///
    /// # Returns
    /// A `String` containing the error description.
    ///
    /// # Examples
    /// ```rust
    /// use kitx::common::error::QueryError;
    ///
    /// let error = QueryError::DBPoolNotInitialized;
    /// assert_eq!(error.message(), "Database pool not initialized");
    /// ```
    ///
    /// 返回查询错误的描述性错误消息。
    ///
    /// 此方法提供可读的错误消息，可以显示给用户或记录用于调试。
    ///
    /// # 返回值
    /// 包含错误描述的 `String`。
    ///
    /// # 示例
    /// ```rust
    /// use kitx::common::error::QueryError;
    ///
    /// let error = QueryError::DBPoolNotInitialized;
    /// assert_eq!(error.message(), "Database pool not initialized");
    /// ```
    pub fn message(&self) -> String {
        match self {
            Self::DBPoolNotInitialized => "Database pool not initialized".to_string(),
            Self::Other(msg) => msg.to_owned(),
        }
    }
}

impl RelationError {
    /// Returns a descriptive error message for the relation error.
    ///
    /// This method provides detailed error messages for relationship-related
    /// errors, including context about the specific values or indices involved.
    ///
    /// # Returns
    /// A `String` containing the error description with relevant context.
    ///
    /// # Examples
    /// ```rust
    /// use kitx::common::error::RelationError;
    ///
    /// let error = RelationError::ValueEmpty(0);
    /// assert_eq!(error.message(), "Expected non-empty values, got 0");
    /// ```
    ///
    /// 返回关联错误的描述性错误消息。
    ///
    /// 此方法为关系相关错误提供详细的错误消息，
    /// 包括涉及的特定值或索引的上下文。
    ///
    /// # 返回值
    /// 包含错误描述和相关上下文的 `String`。
    ///
    /// # 示例
    /// ```rust
    /// use kitx::common::error::RelationError;
    ///
    /// let error = RelationError::ValueEmpty(0);
    /// assert_eq!(error.message(), "Expected non-empty values, got 0");
    /// ```
    pub fn message(&self) -> String {
        match self {
            Self::ValueEmpty(size) => format!("Expected non-empty values, got {}", size),
            Self::ValueMismatch(index, expected, actual) => format!("Value mismatch: index {}, expected {}, got {}", index, expected, actual),
        }
    }
}

// Removed KitxError and its conversions since we're using QueryError directly

impl From<RelationError> for SqlxError {
    /// Converts a RelationError into a SqlxError.
    ///
    /// This conversion enables RelationError to be used in sqlx contexts
    /// by wrapping it in a QueryError and then converting to SqlxError.
    ///
    /// # Arguments
    /// * `err` - The RelationError to convert
    ///
    /// # Returns
    /// A SqlxError containing the RelationError wrapped in a QueryError.
    ///
    /// 将RelationError转换为SqlxError。
    ///
    /// 此转换通过将RelationError包装在QueryError中然后转换为SqlxError，
    /// 使RelationError能够在sqlx上下文中使用。
    ///
    /// # 参数
    /// * `err` - 要转换的RelationError
    ///
    /// # 返回值
    /// 包含封装在QueryError中的RelationError的SqlxError。
    fn from(err: RelationError) -> Self {
        SqlxError::Database(Box::new(QueryError::new(err.message())))
    }
}

impl Display for QueryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.message())
    }
}

impl Error for QueryError {}

impl QueryError {
    /// Creates a new QueryError instance with a custom message
    ///
    /// # Arguments
    /// * `message` - Error description message
    ///
    /// 使用自定义消息创建一个新的QueryError实例
    ///
    /// # 参数
    /// * `message` - 错误描述信息
    pub fn new(message: String) -> Self {
        QueryError::Other(message)
    }
}

impl DatabaseError for QueryError {
    /// Returns a reference to the error as a trait object.
    ///
    /// # Returns
    /// A reference to self as an Error trait object.
    ///
    /// 返回错误作为trait对象的引用。
    ///
    /// # 返回值
    /// self作为Error trait对象的引用。
    fn as_error(&self) -> &(dyn Error + Send + Sync + 'static) {
        self
    }

    /// Returns the error message.
    ///
    /// # Returns
    /// A string slice containing the error message.
    ///
    /// 返回错误消息。
    ///
    /// # 返回值
    /// 包含错误消息的字符串切片。
    fn message(&self) -> &str {
        match self {
            Self::DBPoolNotInitialized => "Database pool not initialized",
            Self::Other(msg) => msg.as_str(),
        }
    }

    /// Returns a mutable reference to the error as a trait object.
    ///
    /// # Returns
    /// A mutable reference to self as an Error trait object.
    ///
    /// 返回错误作为trait对象的可变引用。
    ///
    /// # 返回值
    /// self作为Error trait对象的可变引用。
    fn as_error_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
        self
    }

    /// Converts the boxed error into a boxed trait object.
    ///
    /// # Arguments
    /// * `self` - The boxed QueryError to convert
    ///
    /// # Returns
    /// A boxed Error trait object.
    ///
    /// 将装箱的错误转换为装箱的trait对象。
    ///
    /// # 参数
    /// * `self` - 要转换的装箱QueryError
    ///
    /// # 返回值
    /// 装箱的Error trait对象。
    fn into_error(self: Box<Self>) -> Box<dyn Error + Send + Sync + 'static> {
        self
    }

    /// Returns the kind of database error.
    ///
    /// All QueryError instances are classified as "Other" error kind
    /// since they represent custom application-level errors.
    ///
    /// # Returns
    /// Always returns `ErrorKind::Other`.
    ///
    /// 返回数据库错误的类型。
    ///
    /// 所有QueryError实例都被分类为"Other"错误类型，
    /// 因为它们表示自定义的应用程序级错误。
    ///
    /// # 返回值
    /// 总是返回 `ErrorKind::Other`。
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}
