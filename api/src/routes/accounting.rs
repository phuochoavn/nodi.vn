use axum::{Router, Json, extract::{State, Path, Query}, routing::get};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::http::header;
use serde::Deserialize;
use serde_json::{json, Value};
use rust_xlsxwriter::{Workbook, Format, FormatAlign, Color};

use crate::AppState;
use crate::error::AppError;
use crate::middleware::auth::{verify_token, Claims};

fn extract_claims(headers: &HeaderMap, secret: &str) -> Result<Claims, AppError> {
    let auth = headers.get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing Authorization header".into()))?;
    let token = auth.strip_prefix("Bearer ").unwrap_or(auth);
    verify_token(token, secret)
}

fn get_store_id(claims: &Claims, query_store_id: Option<i32>) -> i32 {
    if claims.role == "admin" {
        query_store_id.unwrap_or(claims.store_id)
    } else {
        claims.store_id
    }
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/api/dashboard/accounting/overview", get(accounting_overview))
        .route("/api/dashboard/accounting/books/{book_id}", get(book_detail))
        .route("/api/dashboard/accounting/books/{book_id}/export", get(book_export))
        .route("/api/dashboard/accounting/tax-declaration", get(tax_declaration))
        .route("/api/dashboard/accounting/tax-declaration/export", get(tax_declaration_export))
}

// ===== Query Params =====
#[derive(Deserialize)]
struct PeriodQuery {
    from: Option<String>,
    to: Option<String>,
    store_id: Option<i32>,
}

#[derive(Deserialize)]
struct TaxQuery {
    period: Option<String>, // e.g. "2026-Q1", "2026-01"
    store_id: Option<i32>,
}

// ===== Helper: parse period into from/to dates =====
fn parse_tax_period(period: &str) -> (String, String) {
    // Format: "2026-Q1" -> ("2026-01-01", "2026-03-31")
    // Format: "2026-01" -> ("2026-01-01", "2026-01-31")
    if let Some(q_pos) = period.find("-Q") {
        let year = &period[..q_pos];
        let quarter: u32 = period[q_pos+2..].parse().unwrap_or(1);
        let (start_month, end_month, end_day) = match quarter {
            1 => (1, 3, 31),
            2 => (4, 6, 30),
            3 => (7, 9, 30),
            4 => (10, 12, 31),
            _ => (1, 3, 31),
        };
        (format!("{}-{:02}-01", year, start_month), format!("{}-{:02}-{:02}", year, end_month, end_day))
    } else if period.len() == 7 {
        // "2026-01"
        let year: i32 = period[..4].parse().unwrap_or(2026);
        let month: u32 = period[5..7].parse().unwrap_or(1);
        let last_day = match month {
            1|3|5|7|8|10|12 => 31,
            4|6|9|11 => 30,
            2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
            _ => 31,
        };
        (format!("{}-{:02}-01", year, month), format!("{}-{:02}-{:02}", year, month, last_day))
    } else {
        // Default: current quarter
        ("2026-01-01".to_string(), "2026-12-31".to_string())
    }
}

// ===== API 1: Accounting Overview =====
async fn accounting_overview(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<PeriodQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let from = q.from.unwrap_or_else(|| "2020-01-01".to_string());
    let to = q.to.unwrap_or_else(|| "2099-12-31".to_string());

    // Total revenue
    let revenue: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount)::bigint, 0) FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date"
    ).bind(sid).bind(&from).bind(&to).fetch_one(&state.pool).await.unwrap_or((0,));

    // Total expenses (cash out)
    let expenses: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(ABS(amount))::bigint, 0) FROM synced_cash_transactions WHERE store_id = $1 AND flow_type = 'OUT' AND DATE(created_at) BETWEEN $2::date AND $3::date"
    ).bind(sid).bind(&from).bind(&to).fetch_one(&state.pool).await.unwrap_or((0,));

    // Total purchase (cost of goods)
    let purchases: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount)::bigint, 0) FROM synced_purchase_orders WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date"
    ).bind(sid).bind(&from).bind(&to).fetch_one(&state.pool).await.unwrap_or((0,));

    // Tax config
    let tax_cfg: Option<(f64, f64, String)> = sqlx::query_as(
        "SELECT vat_rate::float8, pit_rate::float8, tax_period FROM tax_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);

    let (vat_rate, pit_rate, tax_period) = tax_cfg.unwrap_or((1.0, 0.5, "quarterly".to_string()));

    let vat_amount = (revenue.0 as f64 * vat_rate / 100.0) as i64;
    let pit_amount = (revenue.0 as f64 * pit_rate / 100.0) as i64;

    // Store balance
    let balance: Option<(Option<i64>,)> = sqlx::query_as(
        "SELECT COALESCE(current_balance)::bigint FROM synced_store_funds WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await?;
    let balance_val = balance.and_then(|r| r.0).unwrap_or(0);

    Ok(Json(json!({
        "revenue": revenue.0,
        "expenses": expenses.0,
        "purchases": purchases.0,
        "gross_profit": revenue.0 - purchases.0,
        "net_income": revenue.0 - purchases.0 - expenses.0,
        "vat_payable": vat_amount,
        "pit_payable": pit_amount,
        "total_tax_payable": vat_amount + pit_amount,
        "store_balance": balance_val,
        "vat_rate": vat_rate,
        "pit_rate": pit_rate,
        "tax_period": tax_period,
        "period": { "from": from, "to": to },
        "books": [
            { "id": "s1-revenue", "name": "Sổ S1-HKD: Chi tiết doanh thu bán hàng" },
            { "id": "s2-inventory", "name": "Sổ S2-HKD: Chi tiết vật tư, hàng hóa" },
            { "id": "s3-expenses", "name": "Sổ S3-HKD: Chi phí sản xuất, kinh doanh" },
            { "id": "s4-tax", "name": "Sổ S4-HKD: Theo dõi nghĩa vụ thuế" },
            { "id": "s5-salary", "name": "Sổ S5-HKD: Thanh toán tiền lương" },
            { "id": "s6-cash", "name": "Sổ S6-HKD: Quỹ tiền mặt" },
            { "id": "s7-bank", "name": "Sổ S7-HKD: Tiền gửi ngân hàng" },
        ]
    })))
}

// ===== API 2: Book Detail =====
async fn book_detail(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(book_id): Path<String>,
    Query(q): Query<PeriodQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let from = q.from.unwrap_or_else(|| "2020-01-01".to_string());
    let to = q.to.unwrap_or_else(|| "2099-12-31".to_string());

    match book_id.as_str() {
        "s1-revenue" => book_s1_revenue(sid, &from, &to, &state).await,
        "s2-inventory" => book_s2_inventory(sid, &from, &to, &state).await,
        "s3-expenses" => book_s3_expenses(sid, &from, &to, &state).await,
        "s4-tax" => book_s4_tax(sid, &from, &to, &state).await,
        "s5-salary" => book_s5_salary(sid, &from, &to, &state).await,
        "s6-cash" => book_s6_cash(sid, &from, &to, &state).await,
        "s7-bank" => book_s7_bank(sid, &from, &to, &state).await,
        _ => Err(AppError::NotFound(format!("Book '{}' not found", book_id))),
    }
}

// ===== S1: Chi tiết doanh thu =====
async fn book_s1_revenue(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query_as::<_, (chrono::NaiveDate, Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<String>)>(
        "SELECT DATE(i.created_at), i.invoice_number, COALESCE(c.name, i.customer_name, 'Khách lẻ'), \
         i.total_amount::float8, i.discount_amount::float8, i.final_amount::float8, i.payment_method \
         FROM synced_invoices i LEFT JOIN synced_customers c ON i.customer_id = c.local_id AND c.store_id = i.store_id \
         WHERE i.store_id = $1 AND DATE(i.created_at) BETWEEN $2::date AND $3::date \
         ORDER BY i.created_at"
    ).bind(sid).bind(from).bind(to).fetch_all(&state.pool).await?;

    let entries: Vec<Value> = rows.iter().map(|r| json!({
        "date": r.0.to_string(),
        "invoice_number": r.1,
        "customer_name": r.2,
        "total_amount": r.3.unwrap_or(0.0),
        "discount": r.4.unwrap_or(0.0),
        "final_amount": r.5.unwrap_or(0.0),
        "payment_method": r.6,
    })).collect();

    let total: f64 = rows.iter().map(|r| r.5.unwrap_or(r.3.unwrap_or(0.0))).sum();

    Ok(Json(json!({
        "book_id": "s1-revenue",
        "book_name": "Sổ S1-HKD: Chi tiết doanh thu bán hàng hóa, dịch vụ",
        "period": { "from": from, "to": to },
        "entries": entries,
        "total": total,
        "count": entries.len()
    })))
}

// ===== S2: Chi tiết vật tư hàng hóa =====
async fn book_s2_inventory(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query_as::<_, (Option<String>, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<String>)>(
        "SELECT p.name, p.base_unit, p.stock_quantity::float8, p.cost_price::float8, p.sell_price::float8, p.category \
         FROM synced_products p WHERE p.store_id = $1 ORDER BY p.name"
    ).bind(sid).fetch_all(&state.pool).await?;

    let entries: Vec<Value> = rows.iter().map(|r| json!({
        "product_name": r.0,
        "unit": r.1,
        "stock_quantity": r.2.unwrap_or(0.0),
        "cost_price": r.3.unwrap_or(0.0),
        "sell_price": r.4.unwrap_or(0.0),
        "inventory_value": r.2.unwrap_or(0.0) * r.3.unwrap_or(0.0),
        "category": r.5,
    })).collect();

    let total_value: f64 = rows.iter().map(|r| r.2.unwrap_or(0.0) * r.3.unwrap_or(0.0)).sum();

    Ok(Json(json!({
        "book_id": "s2-inventory",
        "book_name": "Sổ S2-HKD: Chi tiết vật tư, hàng hóa",
        "period": { "from": from, "to": to },
        "entries": entries,
        "total_inventory_value": total_value,
        "count": entries.len()
    })))
}

// ===== S3: Chi phí SXKD =====
async fn book_s3_expenses(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query_as::<_, (Option<chrono::NaiveDateTime>, Option<i64>, Option<String>, Option<String>)>(
        "SELECT created_at::timestamp, ABS(amount)::bigint, category, note FROM synced_cash_transactions \
         WHERE store_id = $1 AND flow_type = 'OUT' AND DATE(created_at) BETWEEN $2::date AND $3::date \
         ORDER BY created_at"
    ).bind(sid).bind(from).bind(to).fetch_all(&state.pool).await?;

    let entries: Vec<Value> = rows.iter().map(|r| json!({
        "date": r.0.map(|d| d.to_string()),
        "amount": r.1.unwrap_or(0),
        "category": r.2.as_deref().unwrap_or("Khác"),
        "note": r.3,
    })).collect();

    let total: i64 = rows.iter().map(|r| r.1.unwrap_or(0)).sum();

    Ok(Json(json!({
        "book_id": "s3-expenses",
        "book_name": "Sổ S3-HKD: Chi phí sản xuất, kinh doanh",
        "period": { "from": from, "to": to },
        "entries": entries,
        "total": total,
        "count": entries.len()
    })))
}

// ===== S4: Theo dõi nghĩa vụ thuế =====
async fn book_s4_tax(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    let rev_by_month = sqlx::query_as::<_, (String, i64)>(
        "SELECT TO_CHAR(DATE_TRUNC('month', created_at), 'YYYY-MM'), COALESCE(SUM(total_amount)::bigint, 0) \
         FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date \
         GROUP BY 1 ORDER BY 1"
    ).bind(sid).bind(from).bind(to).fetch_all(&state.pool).await?;

    let tax_cfg: Option<(f64, f64)> = sqlx::query_as(
        "SELECT vat_rate::float8, pit_rate::float8 FROM tax_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);
    let (vat_rate, pit_rate) = tax_cfg.unwrap_or((1.0, 0.5));

    let entries: Vec<Value> = rev_by_month.iter().map(|r| {
        let vat = (r.1 as f64 * vat_rate / 100.0) as i64;
        let pit = (r.1 as f64 * pit_rate / 100.0) as i64;
        json!({
            "month": r.0,
            "revenue": r.1,
            "vat_rate": vat_rate,
            "vat_amount": vat,
            "pit_rate": pit_rate,
            "pit_amount": pit,
            "total_tax": vat + pit,
        })
    }).collect();

    let total_rev: i64 = rev_by_month.iter().map(|r| r.1).sum();
    let total_vat = (total_rev as f64 * vat_rate / 100.0) as i64;
    let total_pit = (total_rev as f64 * pit_rate / 100.0) as i64;

    Ok(Json(json!({
        "book_id": "s4-tax",
        "book_name": "Sổ S4-HKD: Theo dõi tình hình thực hiện nghĩa vụ thuế",
        "period": { "from": from, "to": to },
        "entries": entries,
        "total_revenue": total_rev,
        "total_vat": total_vat,
        "total_pit": total_pit,
        "total_tax": total_vat + total_pit,
        "count": entries.len()
    })))
}

// ===== S5: Thanh toán tiền lương =====
async fn book_s5_salary(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query_as::<_, (Option<chrono::NaiveDateTime>, Option<i64>, Option<String>)>(
        "SELECT created_at::timestamp, ABS(amount)::bigint, note FROM synced_cash_transactions \
         WHERE store_id = $1 AND flow_type = 'OUT' AND (LOWER(category) LIKE '%salary%' OR LOWER(category) LIKE '%luong%' OR LOWER(category) LIKE '%lương%' OR LOWER(note) LIKE '%lương%') \
         AND DATE(created_at) BETWEEN $2::date AND $3::date ORDER BY created_at"
    ).bind(sid).bind(from).bind(to).fetch_all(&state.pool).await?;

    let entries: Vec<Value> = rows.iter().map(|r| json!({
        "date": r.0.map(|d| d.to_string()),
        "amount": r.1.unwrap_or(0),
        "note": r.2,
    })).collect();

    let total: i64 = rows.iter().map(|r| r.1.unwrap_or(0)).sum();

    Ok(Json(json!({
        "book_id": "s5-salary",
        "book_name": "Sổ S5-HKD: Theo dõi tình hình thanh toán tiền lương và các khoản nộp theo lương",
        "period": { "from": from, "to": to },
        "entries": entries,
        "total": total,
        "count": entries.len()
    })))
}

// ===== S6: Quỹ tiền mặt =====
async fn book_s6_cash(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    let rows = sqlx::query_as::<_, (Option<chrono::NaiveDateTime>, Option<i64>, Option<String>, Option<String>, Option<i64>)>(
        "SELECT created_at::timestamp, amount::bigint, flow_type, note, balance_after::bigint FROM synced_cash_transactions \
         WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date ORDER BY created_at"
    ).bind(sid).bind(from).bind(to).fetch_all(&state.pool).await?;

    let entries: Vec<Value> = rows.iter().map(|r| {
        let amount = r.1.unwrap_or(0);
        let flow = r.2.as_deref().unwrap_or("IN");
        json!({
            "date": r.0.map(|d| d.to_string()),
            "receipt": if flow == "IN" { amount } else { 0 },
            "payment": if flow == "OUT" { amount.abs() } else { 0 },
            "flow_type": flow,
            "note": r.3,
            "balance": r.4.unwrap_or(0),
        })
    }).collect();

    Ok(Json(json!({
        "book_id": "s6-cash",
        "book_name": "Sổ S6-HKD: Quỹ tiền mặt",
        "period": { "from": from, "to": to },
        "entries": entries,
        "count": entries.len()
    })))
}

// ===== S7: Tiền gửi ngân hàng =====
async fn book_s7_bank(sid: i32, from: &str, to: &str, state: &AppState) -> Result<Json<Value>, AppError> {
    // Bank transactions = invoices paid via bank transfer
    let rows = sqlx::query_as::<_, (Option<chrono::NaiveDateTime>, Option<String>, Option<f64>, Option<String>)>(
        "SELECT created_at, invoice_number, total_amount::float8, payment_method FROM synced_invoices \
         WHERE store_id = $1 AND LOWER(payment_method) IN ('bank', 'transfer', 'chuyển khoản', 'chuyen khoan') \
         AND DATE(created_at) BETWEEN $2::date AND $3::date ORDER BY created_at"
    ).bind(sid).bind(from).bind(to).fetch_all(&state.pool).await?;

    let entries: Vec<Value> = rows.iter().map(|r| json!({
        "date": r.0.map(|d| d.to_string()),
        "invoice_number": r.1,
        "amount": r.2.unwrap_or(0.0),
        "type": "receipt",
        "note": format!("Thu tiền HĐ {}", r.1.as_deref().unwrap_or("")),
    })).collect();

    let total: f64 = rows.iter().map(|r| r.2.unwrap_or(0.0)).sum();

    Ok(Json(json!({
        "book_id": "s7-bank",
        "book_name": "Sổ S7-HKD: Tiền gửi ngân hàng",
        "period": { "from": from, "to": to },
        "entries": entries,
        "total_receipts": total,
        "count": entries.len()
    })))
}

// ===== API 3: Book Export (Excel) =====
async fn book_export(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(book_id): Path<String>,
    Query(q): Query<PeriodQuery>,
) -> Result<Response, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let from = q.from.unwrap_or_else(|| "2020-01-01".to_string());
    let to = q.to.unwrap_or_else(|| "2099-12-31".to_string());

    // Get book data
    let book_data = match book_id.as_str() {
        "s1-revenue" => book_s1_revenue(sid, &from, &to, &state).await?,
        "s2-inventory" => book_s2_inventory(sid, &from, &to, &state).await?,
        "s3-expenses" => book_s3_expenses(sid, &from, &to, &state).await?,
        "s4-tax" => book_s4_tax(sid, &from, &to, &state).await?,
        "s5-salary" => book_s5_salary(sid, &from, &to, &state).await?,
        "s6-cash" => book_s6_cash(sid, &from, &to, &state).await?,
        "s7-bank" => book_s7_bank(sid, &from, &to, &state).await?,
        _ => return Err(AppError::NotFound(format!("Book '{}' not found", book_id))),
    };

    let data = book_data.0;
    let book_name = data["book_name"].as_str().unwrap_or("Sổ kế toán");
    let entries = data["entries"].as_array().cloned().unwrap_or_default();

    // Create Excel workbook
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();

    // Formatting
    let title_fmt = Format::new()
        .set_bold()
        .set_font_size(14)
        .set_align(FormatAlign::Center);
    let header_fmt = Format::new()
        .set_bold()
        .set_font_size(11)
        .set_background_color(Color::RGB(0x2563EB))
        .set_font_color(Color::White)
        .set_align(FormatAlign::Center);
    let money_fmt = Format::new()
        .set_num_format("#,##0");

    // Title
    sheet.merge_range(0, 0, 0, 5, book_name, &title_fmt).ok();
    sheet.write_string(1, 0, &format!("Từ {} đến {}", from, to)).ok();

    // Write headers and data based on book type
    let row_start = 3u32;
    match book_id.as_str() {
        "s1-revenue" => {
            let headers_list = ["Ngày", "Số HĐ", "Khách hàng", "Tổng tiền", "Giảm giá", "Thực thu", "Hình thức TT"];
            for (c, h) in headers_list.iter().enumerate() {
                sheet.write_string_with_format(row_start, c as u16, *h, &header_fmt).ok();
            }
            for (i, e) in entries.iter().enumerate() {
                let r = row_start + 1 + i as u32;
                sheet.write_string(r, 0, e["date"].as_str().unwrap_or("")).ok();
                sheet.write_string(r, 1, e["invoice_number"].as_str().unwrap_or("")).ok();
                sheet.write_string(r, 2, e["customer_name"].as_str().unwrap_or("")).ok();
                sheet.write_number_with_format(r, 3, e["total_amount"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number_with_format(r, 4, e["discount"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number_with_format(r, 5, e["final_amount"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_string(r, 6, e["payment_method"].as_str().unwrap_or("")).ok();
            }
        },
        "s2-inventory" => {
            let headers_list = ["Tên hàng", "ĐVT", "Tồn kho", "Giá vốn", "Giá bán", "Giá trị tồn", "Nhóm hàng"];
            for (c, h) in headers_list.iter().enumerate() {
                sheet.write_string_with_format(row_start, c as u16, *h, &header_fmt).ok();
            }
            for (i, e) in entries.iter().enumerate() {
                let r = row_start + 1 + i as u32;
                sheet.write_string(r, 0, e["product_name"].as_str().unwrap_or("")).ok();
                sheet.write_string(r, 1, e["unit"].as_str().unwrap_or("")).ok();
                sheet.write_number(r, 2, e["stock_quantity"].as_f64().unwrap_or(0.0)).ok();
                sheet.write_number_with_format(r, 3, e["cost_price"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number_with_format(r, 4, e["sell_price"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number_with_format(r, 5, e["inventory_value"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_string(r, 6, e["category"].as_str().unwrap_or("")).ok();
            }
        },
        "s3-expenses" | "s5-salary" => {
            let headers_list = ["Ngày", "Số tiền", "Loại", "Ghi chú"];
            for (c, h) in headers_list.iter().enumerate() {
                sheet.write_string_with_format(row_start, c as u16, *h, &header_fmt).ok();
            }
            for (i, e) in entries.iter().enumerate() {
                let r = row_start + 1 + i as u32;
                sheet.write_string(r, 0, e["date"].as_str().unwrap_or("")).ok();
                sheet.write_number_with_format(r, 1, e["amount"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_string(r, 2, e["category"].as_str().unwrap_or(e["note"].as_str().unwrap_or(""))).ok();
                sheet.write_string(r, 3, e["note"].as_str().unwrap_or("")).ok();
            }
        },
        "s4-tax" => {
            let headers_list = ["Tháng", "Doanh thu", "Thuế suất GTGT (%)", "Thuế GTGT", "Thuế suất TNCN (%)", "Thuế TNCN", "Tổng thuế"];
            for (c, h) in headers_list.iter().enumerate() {
                sheet.write_string_with_format(row_start, c as u16, *h, &header_fmt).ok();
            }
            for (i, e) in entries.iter().enumerate() {
                let r = row_start + 1 + i as u32;
                sheet.write_string(r, 0, e["month"].as_str().unwrap_or("")).ok();
                sheet.write_number_with_format(r, 1, e["revenue"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number(r, 2, e["vat_rate"].as_f64().unwrap_or(0.0)).ok();
                sheet.write_number_with_format(r, 3, e["vat_amount"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number(r, 4, e["pit_rate"].as_f64().unwrap_or(0.0)).ok();
                sheet.write_number_with_format(r, 5, e["pit_amount"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number_with_format(r, 6, e["total_tax"].as_f64().unwrap_or(0.0), &money_fmt).ok();
            }
        },
        "s6-cash" | "s7-bank" => {
            let headers_list = ["Ngày", "Thu", "Chi", "Ghi chú", "Số dư"];
            for (c, h) in headers_list.iter().enumerate() {
                sheet.write_string_with_format(row_start, c as u16, *h, &header_fmt).ok();
            }
            for (i, e) in entries.iter().enumerate() {
                let r = row_start + 1 + i as u32;
                sheet.write_string(r, 0, e["date"].as_str().unwrap_or("")).ok();
                sheet.write_number_with_format(r, 1, e["receipt"].as_f64().or(e["amount"].as_f64()).unwrap_or(0.0), &money_fmt).ok();
                sheet.write_number_with_format(r, 2, e["payment"].as_f64().unwrap_or(0.0), &money_fmt).ok();
                sheet.write_string(r, 3, e["note"].as_str().unwrap_or("")).ok();
                sheet.write_number_with_format(r, 4, e["balance"].as_f64().unwrap_or(0.0), &money_fmt).ok();
            }
        },
        _ => {}
    }

    // Set column widths
    for c in 0..8u16 {
        sheet.set_column_width(c, 18).ok();
    }

    let buf = workbook.save_to_buffer()
        .map_err(|e| AppError::Internal(format!("Excel generation error: {}", e)))?;

    let filename = format!("{}_{}_to_{}.xlsx", book_id, from, to);

    Ok((
        [
            (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ],
        buf,
    ).into_response())
}

// ===== API 4: Tax Declaration =====
async fn tax_declaration(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<TaxQuery>,
) -> Result<Json<Value>, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);

    let period = q.period.unwrap_or_else(|| "2026-Q1".to_string());
    let (from, to) = parse_tax_period(&period);

    // Revenue
    let revenue: (i64,) = sqlx::query_as(
        "SELECT COALESCE(SUM(total_amount)::bigint, 0) FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date"
    ).bind(sid).bind(&from).bind(&to).fetch_one(&state.pool).await.unwrap_or((0,));

    let order_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM synced_invoices WHERE store_id = $1 AND DATE(created_at) BETWEEN $2::date AND $3::date"
    ).bind(sid).bind(&from).bind(&to).fetch_one(&state.pool).await.unwrap_or((0,));

    // Tax config
    let tax_cfg: Option<(f64, f64, String)> = sqlx::query_as(
        "SELECT vat_rate::float8, pit_rate::float8, business_type FROM tax_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);
    let (vat_rate, pit_rate, biz_type) = tax_cfg.unwrap_or((1.0, 0.5, "retail".to_string()));

    let vat_amount = (revenue.0 as f64 * vat_rate / 100.0) as i64;
    let pit_amount = (revenue.0 as f64 * pit_rate / 100.0) as i64;

    // Store info
    let store_info: Option<(Option<String>, Option<String>, Option<String>)> = sqlx::query_as(
        "SELECT name, address, phone FROM synced_store_settings WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);
    let (store_name, store_addr, store_phone) = store_info.unwrap_or((None, None, None));

    // Tax code from einvoice config
    let tax_code: Option<(Option<String>,)> = sqlx::query_as(
        "SELECT tax_code FROM einvoice_config WHERE store_id = $1"
    ).bind(sid).fetch_optional(&state.pool).await.unwrap_or(None);

    Ok(Json(json!({
        "declaration": {
            "form": "Mẫu số 01/CNKD",
            "regulation": "Thông tư 40/2021/TT-BTC",
            "period": period,
            "period_dates": { "from": from, "to": to },
            "taxpayer": {
                "name": store_name.unwrap_or_default(),
                "address": store_addr.unwrap_or_default(),
                "phone": store_phone.unwrap_or_default(),
                "tax_code": tax_code.and_then(|t| t.0).unwrap_or_default(),
            },
            "business_type": biz_type,
            "revenue": revenue.0,
            "order_count": order_count.0,
            "vat_rate": vat_rate,
            "vat_amount": vat_amount,
            "pit_rate": pit_rate,
            "pit_amount": pit_amount,
            "total_tax": vat_amount + pit_amount,
        }
    })))
}

// ===== API 5: Tax Declaration Export (Excel) =====
async fn tax_declaration_export(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<TaxQuery>,
) -> Result<Response, AppError> {
    let claims = extract_claims(&headers, &state.config.jwt_secret)?;
    let sid = get_store_id(&claims, q.store_id);
    let period = q.period.unwrap_or_else(|| "2026-Q1".to_string());
    let (from, to) = parse_tax_period(&period);

    // Get tax declaration data
    let decl_res = tax_declaration(
        State(state.clone()), headers.clone(), Query(TaxQuery { period: Some(period.clone()), store_id: Some(sid) })
    ).await?;
    let decl = &decl_res.0["declaration"];

    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();

    let title_fmt = Format::new().set_bold().set_font_size(14).set_align(FormatAlign::Center);
    let label_fmt = Format::new().set_bold().set_font_size(11);
    let money_fmt = Format::new().set_num_format("#,##0").set_bold();

    // Title
    sheet.merge_range(0, 0, 0, 3, "TỜ KHAI THUẾ ĐỐI VỚI HỘ KINH DOANH, CÁ NHÂN KINH DOANH", &title_fmt).ok();
    sheet.write_string(1, 0, &format!("Mẫu số 01/CNKD — Thông tư 40/2021/TT-BTC")).ok();
    sheet.write_string(2, 0, &format!("Kỳ tính thuế: {} (Từ {} đến {})", period, from, to)).ok();

    // Taxpayer info
    let mut r = 4u32;
    sheet.write_string_with_format(r, 0, "I. THÔNG TIN NGƯỜI NỘP THUẾ", &label_fmt).ok();
    r += 1;
    sheet.write_string(r, 0, "Tên hộ kinh doanh:").ok();
    sheet.write_string(r, 1, decl["taxpayer"]["name"].as_str().unwrap_or("")).ok();
    r += 1;
    sheet.write_string(r, 0, "Mã số thuế:").ok();
    sheet.write_string(r, 1, decl["taxpayer"]["tax_code"].as_str().unwrap_or("")).ok();
    r += 1;
    sheet.write_string(r, 0, "Địa chỉ:").ok();
    sheet.write_string(r, 1, decl["taxpayer"]["address"].as_str().unwrap_or("")).ok();
    r += 1;
    sheet.write_string(r, 0, "Số điện thoại:").ok();
    sheet.write_string(r, 1, decl["taxpayer"]["phone"].as_str().unwrap_or("")).ok();

    // Tax calculation
    r += 2;
    sheet.write_string_with_format(r, 0, "II. TÍNH THUẾ", &label_fmt).ok();
    r += 1;

    let headers_list  = ["Chỉ tiêu", "Giá trị"];
    for (c, h) in headers_list.iter().enumerate() {
        let hdr_fmt = Format::new().set_bold().set_background_color(Color::RGB(0x2563EB)).set_font_color(Color::White);
        sheet.write_string_with_format(r, c as u16, *h, &hdr_fmt).ok();
    }
    r += 1;

    let items = vec![
        ("Ngành nghề kinh doanh", decl["business_type"].as_str().unwrap_or("retail").to_string()),
        ("Tổng doanh thu", format!("{}", decl["revenue"].as_i64().unwrap_or(0))),
        ("Số đơn hàng", format!("{}", decl["order_count"].as_i64().unwrap_or(0))),
        ("Thuế suất GTGT (%)", format!("{}%", decl["vat_rate"].as_f64().unwrap_or(0.0))),
        ("Thuế GTGT phải nộp", format!("{}", decl["vat_amount"].as_i64().unwrap_or(0))),
        ("Thuế suất TNCN (%)", format!("{}%", decl["pit_rate"].as_f64().unwrap_or(0.0))),
        ("Thuế TNCN phải nộp", format!("{}", decl["pit_amount"].as_i64().unwrap_or(0))),
        ("TỔNG SỐ THUẾ PHẢI NỘP", format!("{}", decl["total_tax"].as_i64().unwrap_or(0))),
    ];

    for (label, value) in &items {
        sheet.write_string(r, 0, *label).ok();
        if let Ok(num) = value.parse::<f64>() {
            sheet.write_number_with_format(r, 1, num, &money_fmt).ok();
        } else {
            sheet.write_string(r, 1, value).ok();
        }
        r += 1;
    }

    // Set column widths
    sheet.set_column_width(0, 35).ok();
    sheet.set_column_width(1, 25).ok();
    sheet.set_column_width(2, 20).ok();
    sheet.set_column_width(3, 20).ok();

    let buf = workbook.save_to_buffer()
        .map_err(|e| AppError::Internal(format!("Excel generation error: {}", e)))?;

    let filename = format!("to_khai_thue_{}.xlsx", period);

    Ok((
        [
            (header::CONTENT_TYPE, "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"),
            (header::CONTENT_DISPOSITION, &format!("attachment; filename=\"{}\"", filename)),
        ],
        buf,
    ).into_response())
}
