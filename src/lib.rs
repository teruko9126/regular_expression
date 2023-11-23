//! # 正規表現エンジン用クレート。
//!
//! ## 利用例
//!
//! ```
//! use regular_expression;
//! let expr = "a(bc)+|c(def)*"; // 正規表現
//! let line = "cdefdefdef"; // マッチ対象文字列
//! regular_expression::do_matching(expr, line, true); // 幅優先探索でマッチング
//! regular_expression::print(expr); // 正規表現のASTと命令列を表示
//! ```
mod engine;
mod helper;

pub use engine::{do_matching, print};
