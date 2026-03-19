// Guide content data for Help Center pages
// Each guide has: title, icon, headings (for TOC), prev/next (navigation), content (HTML)

export function useGuideData() {
  const guides = {

    'bat-dau-nhanh': {
      title: 'Bắt đầu nhanh', icon: '⚡',
      headings: ['Tải và cài đặt', 'Kích hoạt bản quyền', 'Cài đặt thông tin cửa hàng', 'Thêm sản phẩm đầu tiên', 'Bắt đầu bán hàng'],
      prev: null, next: 'ban-hang',
      content: `
<h2 id="s0">Tải và cài đặt</h2>
<ul>
<li>Tải file <code>.exe</code> (Windows) hoặc <code>.apk</code> (Android) từ <a href="/tai-ung-dung">nodi.vn/tai-ung-dung</a></li>
<li>Cài đặt bình thường, không cần internet liên tục</li>
<li>App hoạt động <strong>offline đầy đủ</strong> sau khi cài</li>
</ul>

<h2 id="s1">Kích hoạt bản quyền</h2>
<p>Nếu bạn đã mua gói Pro:</p>
<ul>
<li>Mở app → xuất hiện màn hình Activation</li>
<li>Nhập License Key → app xác thực HWID (mã máy) → Hoàn tất</li>
<li>Hỗ trợ tối đa <strong>10 thiết bị</strong> trên 1 license (Multi-Device)</li>
</ul>
<div class="tip-box">💡 <strong>Chưa mua?</strong> Dùng Free (20 đơn/ngày), tất cả tính năng đều mở, không bị khóa gì cả.</div>

<h2 id="s2">Cài đặt thông tin cửa hàng</h2>
<p>Vào <strong>Cài đặt → Thông tin Cửa hàng</strong>:</p>
<ul>
<li>Nhập Tên cửa hàng, Địa chỉ, SĐT</li>
<li>MST (nếu có) — cần cho hóa đơn điện tử</li>
<li>Logo cửa hàng — hiển thị trên hóa đơn</li>
</ul>

<h2 id="s3">Thêm sản phẩm đầu tiên</h2>
<p>Vào <strong>Kho hàng</strong> → nút <strong>"+ Thêm sản phẩm"</strong>:</p>
<ul>
<li>Nhập tên, giá bán, giá vốn, đơn vị tính (chai, gói, bao, kg...)</li>
<li><strong>Gợi ý AI:</strong> App có sẵn 5,700+ sản phẩm BVTV/phân bón → khi gõ tên, app tự gợi ý → chỉ cần chọn</li>
<li>Cũng có thể <strong>nhập hàng loạt</strong> từ file Excel (.xlsx)</li>
</ul>

<h2 id="s4">Bắt đầu bán hàng</h2>
<p>Vào tab <strong>Bán hàng (POS)</strong> → tìm SP → click/tap để thêm vào giỏ → nhấn <strong>Thanh toán</strong>.</p>
<p>Xem tiếp: <a href="/huong-dan/ban-hang">Hướng dẫn bán hàng chi tiết →</a></p>`
    },

    'ban-hang': {
      title: 'Bán hàng (POS)', icon: '🛒',
      headings: ['Tìm sản phẩm', 'Thêm vào giỏ', 'Quét barcode', 'Phím tắt PC', 'Giao diện Mobile'],
      prev: 'bat-dau-nhanh', next: 'thanh-toan',
      content: `
<h2 id="s0">Tìm sản phẩm</h2>
<table><thead><tr><th>Thao tác</th><th>Cách làm</th></tr></thead><tbody>
<tr><td>Tìm kiếm</td><td>Gõ tên hoặc mã barcode vào ô tìm kiếm</td></tr>
<tr><td>Lọc danh mục</td><td>Click các tab: Thuốc trừ sâu, Phân bón, Giống, v.v.</td></tr>
<tr><td>Thêm vào giỏ</td><td>Click sản phẩm → chọn đơn vị → nhập số lượng. Hoặc nút ⚡ thêm nhanh</td></tr>
<tr><td>Quét barcode USB</td><td>Cắm máy quét USB → quét → SP tự thêm vào giỏ</td></tr>
<tr><td>Quét bằng ĐT</td><td>Click 📱 → quét QR trên màn hình → điện thoại thành máy quét</td></tr>
</tbody></table>

<h2 id="s1">Thêm vào giỏ</h2>
<ul>
<li>Click sản phẩm → tự thêm với SL = 1</li>
<li>Sửa số lượng: click vào ô SL trong giỏ → gõ số mới</li>
<li>Xóa SP: click nút 🗑️ bên cạnh</li>
<li>Xóa toàn bộ giỏ: nút "Xóa giỏ" hoặc <kbd>F4</kbd></li>
</ul>

<h2 id="s2">Quét barcode</h2>
<ul>
<li><strong>Máy quét USB:</strong> Cắm vào PC → quét mã vạch trên bao bì → SP tự thêm vào giỏ</li>
<li><strong>Remote Scanner:</strong> Click biểu tượng 📱 trên PC → quét QR bằng điện thoại → điện thoại trở thành máy quét</li>
</ul>

<h2 id="s3">Phím tắt PC</h2>
<table><thead><tr><th>Phím</th><th>Chức năng</th></tr></thead><tbody>
<tr><td><kbd>F1</kbd></td><td>Focus vào ô tìm kiếm</td></tr>
<tr><td><kbd>F2</kbd></td><td>Mở thanh toán</td></tr>
<tr><td><kbd>F4</kbd></td><td>Xóa giỏ hàng</td></tr>
<tr><td><kbd>Enter</kbd></td><td>Xác nhận khi quét barcode</td></tr>
</tbody></table>

<h2 id="s4">Giao diện Mobile</h2>
<ul>
<li>Grid sản phẩm dạng lưới, thanh search phía trên</li>
<li>Giỏ hàng mở từ nút <strong>FAB</strong> (nút tròn nổi) phía dưới</li>
<li>Checkout mở toàn màn hình, touch-optimized</li>
</ul>`
    },

    'thanh-toan': {
      title: 'Thanh toán (Checkout)', icon: '💳',
      headings: ['Chọn khách hàng', 'Giảm giá & Đổi điểm', 'Phương thức thanh toán', 'VietQR', 'Xuất HĐĐT', 'Xác nhận & In'],
      prev: 'ban-hang', next: 'lich-su-don-hang',
      content: `
<h2 id="s0">Chọn khách hàng</h2>
<ul>
<li>Gõ tên/SĐT → app gợi ý khách cũ</li>
<li>Khách mới: nhập tên + SĐT → app tự tạo</li>
<li>Bỏ trống: đơn hàng ghi "Khách lẻ"</li>
</ul>

<h2 id="s1">Giảm giá & Đổi điểm</h2>
<ul>
<li><strong>Giảm giá:</strong> Nhập số tiền giảm trực tiếp cho đơn hàng</li>
<li><strong>Đổi điểm:</strong> Nếu khách có điểm loyalty → hiển thị số điểm + hạng thẻ → nhập số điểm muốn đổi → app tự trừ tiền</li>
</ul>
<div class="tip-box">💡 Quy đổi: VD 100 điểm = 1,000đ (tùy cài đặt tại Cài đặt → Tích điểm)</div>

<h2 id="s2">Phương thức thanh toán</h2>
<table><thead><tr><th>Phương thức</th><th>Mô tả</th></tr></thead><tbody>
<tr><td>💵 Tiền mặt</td><td>Nhập số tiền khách đưa → hiển thị tiền thối</td></tr>
<tr><td>📲 Chuyển khoản</td><td>Hiển thị QR VietQR → khách quét mã → tự ghi nhận</td></tr>
<tr><td>📝 Ghi nợ</td><td>Toàn bộ hoặc một phần → cộng vào công nợ KH</td></tr>
<tr><td>🔄 Kết hợp</td><td>Thêm nhiều dòng (VD: 500k mặt + 200k CK + 100k nợ)</td></tr>
</tbody></table>

<h2 id="s3">VietQR</h2>
<ul>
<li><strong>Cài đặt:</strong> Vào Cài đặt → Thông tin Cửa hàng → nhập STK + Ngân hàng</li>
<li>Khi checkout chọn "Chuyển khoản" → QR xuất hiện</li>
<li>Nếu có 2 màn hình: QR hiển thị trên màn khách</li>
<li>Sinh mã giao dịch tự động để dễ đối soát</li>
</ul>

<h2 id="s4">Xuất HĐĐT</h2>
<p>Tick ô "Xuất HĐĐT" → xem trước nội dung HĐ → xác nhận. Yêu cầu đã cài đặt HĐĐT trong Settings.</p>

<h2 id="s5">Xác nhận & In</h2>
<ul>
<li>Nhấn "Xác nhận" → đơn hàng được lưu → in hóa đơn nhiệt (nếu có máy in)</li>
<li>Điểm loyalty tự động cộng cho khách</li>
<li>Ghi chú đơn hàng: nhập VD "Giao ngày mai", "Pha 100 lít nước"</li>
</ul>`
    },

    'lich-su-don-hang': {
      title: 'Lịch sử đơn hàng', icon: '📋',
      headings: ['Xem lịch sử', 'Bộ lọc', 'Trả hàng', 'Xuất Excel'],
      prev: 'thanh-toan', next: 'kho-hang',
      content: `
<h2 id="s0">Xem lịch sử</h2>
<p>Bảng gồm: Mã đơn, Ngày, Khách, Tổng tiền, Thanh toán, Trạng thái. Click xem chi tiết: danh sách SP, giá, số lượng.</p>

<h2 id="s1">Bộ lọc</h2>
<ul>
<li>Theo ngày (date range), Trạng thái, Phương thức thanh toán</li>
<li>Loại KH: Có tên / Khách lẻ</li>
<li>Loại HĐ: Có hóa đơn / Không hóa đơn</li>
<li>Trạng thái HĐĐT: xem badge ✅ Đã xuất | ⏳ Đang xử lý | ❌ Lỗi → nút "Thử lại"</li>
</ul>

<h2 id="s2">Trả hàng</h2>
<p>Mở chi tiết đơn → nút "Trả hàng" → chọn SP cần trả, nhập SL → xác nhận. Tồn kho tự cộng lại, tiền hoàn cho khách.</p>

<h2 id="s3">Xuất Excel</h2>
<p>Nút "Xuất Excel" → tải file <code>.xlsx</code> toàn bộ đơn hàng trong khoảng ngày đã lọc.</p>`
    },

    'kho-hang': {
      title: 'Quản lý kho hàng', icon: '📦',
      headings: ['Danh sách SP', 'Bộ lọc thông minh', 'Thêm sản phẩm', 'Lô hàng (Batches)', 'Kiểm kê', 'In nhãn & Xuất Excel'],
      prev: 'lich-su-don-hang', next: 'khach-hang-cong-no',
      content: `
<h2 id="s0">Danh sách SP</h2>
<p>Bảng gồm: Tên SP, Mã barcode, Danh mục, Tồn kho, Giá bán, Giá vốn, HSD. Lọc theo danh mục, tìm theo tên/barcode/hoạt chất/nhà SX.</p>
<p>Thống kê nhanh đầu trang: <strong>Tổng SKU</strong>, <strong>Tổng tồn kho</strong>, <strong>Tổng giá trị hàng tồn</strong>.</p>

<h2 id="s1">Bộ lọc thông minh</h2>
<table><thead><tr><th>Bộ lọc</th><th>Mô tả</th></tr></thead><tbody>
<tr><td>🔴 Hết hàng</td><td>Tồn kho = 0</td></tr>
<tr><td>🟠 Sắp hết</td><td>Tồn kho dưới mức cảnh báo</td></tr>
<tr><td>🟡 Sắp hết hạn</td><td>HSD < 3 tháng</td></tr>
<tr><td>🔴 Đã hết hạn</td><td>Quá HSD</td></tr>
<tr><td>📦 Hàng chậm bán</td><td>Không bán > 30 ngày (tùy chỉnh)</td></tr>
</tbody></table>

<h2 id="s2">Thêm sản phẩm</h2>
<ul>
<li><strong>Nhập tay:</strong> Tên, Barcode, Danh mục, Giá bán/vốn, Đơn vị, Tồn kho ban đầu, HSD</li>
<li><strong>Gợi ý AI 5,700+ SP:</strong> Gõ tên → app gợi ý SP nông nghiệp → chọn → tự điền thông tin</li>
<li><strong>Import Excel:</strong> Nút Import → chọn file <code>.xlsx</code> → nhập hàng loạt</li>
</ul>

<h2 id="s3">Lô hàng (Batches)</h2>
<ul>
<li>Mỗi SP có thể có nhiều lô (ngày nhập, HSD, tồn kho riêng)</li>
<li>Click "Xem lô" → thấy chi tiết từng lô</li>
<li>Bán hàng tự động chọn lô: <strong>FEFO</strong> (hết hạn trước bán trước) hoặc <strong>FIFO</strong></li>
</ul>

<h2 id="s4">Kiểm kê</h2>
<p>Nút "Kiểm kê" → đếm SP thực tế → so sánh với hệ thống → cập nhật chênh lệch.</p>

<h2 id="s5">In nhãn & Xuất Excel</h2>
<ul>
<li><strong>In nhãn:</strong> Chọn SP → "In nhãn" → in mã vạch Code 128 lên giấy nhiệt 80mm (Tên SP, Barcode, Giá bán)</li>
<li><strong>Xuất Excel:</strong> Nút "Xuất Excel" → tải file <code>.xlsx</code> toàn bộ danh sách SP</li>
</ul>`
    },

    'khach-hang-cong-no': {
      title: 'Khách hàng & Công nợ', icon: '👥',
      headings: ['Quản lý khách hàng', 'Sổ nợ', 'Thu nợ', 'Hạn mức tín dụng'],
      prev: 'kho-hang', next: 'nha-cung-cap',
      content: `
<h2 id="s0">Quản lý khách hàng</h2>
<ul>
<li>Thêm/sửa/xóa KH (Tên, SĐT, Địa chỉ, CCCD)</li>
<li>Lọc: Tất cả / Có nợ / Không nợ</li>
<li>Tìm kiếm theo tên hoặc SĐT</li>
<li>Xuất danh sách + công nợ ra Excel</li>
</ul>

<h2 id="s1">Sổ nợ</h2>
<ul>
<li>Mỗi KH có "Nợ hiện tại" hiển thị nổi bật (<span class="text-red">đỏ khi > 0</span>)</li>
<li>Ghi nợ: tự động khi checkout chọn "Ghi nợ"</li>
<li>Xem lịch sử mua hàng + lịch sử nợ của từng khách</li>
</ul>

<h2 id="s2">Thu nợ</h2>
<p>Click nút <strong>"Thu nợ"</strong> → nhập số tiền → chọn tiền mặt/CK → xác nhận → nợ tự giảm.</p>
<div class="tip-box">💡 Có thể thu một phần. VD: nợ 5 triệu, thu 2 triệu → nợ còn 3 triệu.</div>

<h2 id="s3">Hạn mức tín dụng</h2>
<ul>
<li>Cấu hình hạn mức nợ tối đa cho mỗi khách</li>
<li>Checkout tự động <strong>cảnh báo</strong> nếu vượt hạn mức</li>
</ul>`
    },

    'nha-cung-cap': {
      title: 'Nhà cung cấp & Nhập hàng', icon: '🚛',
      headings: ['Quản lý NCC', 'Chi tiết NCC', 'Trả nợ NCC', 'Phiếu nhập hàng'],
      prev: 'khach-hang-cong-no', next: 'so-quy',
      content: `
<h2 id="s0">Quản lý NCC</h2>
<p>Danh sách NCC gồm: Mã, Tên, Loại (Công ty/Cá nhân), SĐT, Địa chỉ, Nợ hiện tại. Thêm/sửa/xóa NCC, sắp xếp theo Mã/Tên/Nợ.</p>

<h2 id="s1">Chi tiết NCC</h2>
<ul>
<li>Xem thông tin + tổng nợ + lịch sử giao dịch</li>
<li>Lịch sử: Ngày, Loại (Nhập hàng/Trả nợ/Hoàn trả), Số tiền, Ghi chú, Số dư</li>
<li>Lọc giao dịch theo loại, tìm kiếm, chọn khoảng ngày</li>
<li>Xuất lịch sử ra Excel</li>
</ul>

<h2 id="s2">Trả nợ NCC</h2>
<p>Click <strong>"Trả nợ"</strong> → nhập số tiền → trừ từ quỹ tiền mặt → cập nhật nợ.</p>

<h2 id="s3">Phiếu nhập hàng</h2>
<ul>
<li>Tạo phiếu nhập: chọn NCC, thêm SP + SL + giá nhập → lưu</li>
<li>Tồn kho tự động cập nhật khi nhập hàng</li>
<li>Hỗ trợ nhập nợ NCC (ghi nợ khi nhập)</li>
</ul>`
    },

    'so-quy': {
      title: 'Sổ quỹ (Thu Chi)', icon: '💰',
      headings: ['Tổng quan', 'Tạo phiếu thu/chi', 'Bộ lọc', 'Xuất Excel'],
      prev: 'nha-cung-cap', next: 'bao-cao',
      content: `
<h2 id="s0">Tổng quan</h2>
<ul>
<li>3 thẻ tóm tắt: <strong>Số dư quỹ</strong>, <strong>Tổng thu</strong>, <strong>Tổng chi</strong> (trong khoảng thời gian)</li>
<li>Click vào thẻ Thu/Chi → lọc giao dịch tương ứng</li>
<li>Biểu đồ cột: Thu vs Chi theo ngày</li>
</ul>

<h2 id="s1">Tạo phiếu thu/chi</h2>
<p>Nút <strong>"+ Tạo phiếu"</strong> → chọn Loại (Thu/Chi) → Danh mục → Số tiền → Ghi chú → Lưu.</p>
<table><thead><tr><th>Thu</th><th>Chi</th></tr></thead><tbody>
<tr><td>Bán hàng</td><td>Nhập hàng</td></tr>
<tr><td>Thu nợ KH</td><td>Trả nợ NCC</td></tr>
<tr><td>Thu khác</td><td>Tiền thuê, Chi phí KD, Chi khác</td></tr>
</tbody></table>

<h2 id="s2">Bộ lọc</h2>
<p>Chọn khoảng ngày (date range picker), lọc Thu / Chi.</p>

<h2 id="s3">Xuất Excel</h2>
<p>Toàn bộ giao dịch trong khoảng ngày → file <code>.xlsx</code>.</p>`
    },

    'bao-cao': {
      title: 'Báo cáo & Thống kê', icon: '📊',
      headings: ['KPI tổng quan', 'Biểu đồ', 'Top SP bán chạy', 'Xuất Excel'],
      prev: 'so-quy', next: 'thue-ke-toan',
      content: `
<h2 id="s0">KPI tổng quan</h2>
<p>3 thẻ chính: <strong>Doanh thu</strong>, <strong>Lợi nhuận gộp</strong>, <strong>Số đơn hàng</strong>. Có tỷ lệ lợi nhuận (%).</p>

<h2 id="s1">Biểu đồ</h2>
<ul>
<li>Chọn xem: Doanh thu / Lợi nhuận / Đơn hàng</li>
<li>Quick filters: 7 ngày / 30 ngày / 90 ngày, hoặc custom date range</li>
</ul>

<h2 id="s2">Top SP bán chạy</h2>
<p>Bảng: Tên SP, Số lượng bán, Doanh thu, Lợi nhuận.</p>

<h2 id="s3">Xuất Excel</h2>
<p>Xuất báo cáo doanh thu theo ngày ra file <code>.xlsx</code>.</p>`
    },

    'thue-ke-toan': {
      title: 'Thuế & Kế toán', icon: '🧾',
      headings: ['Doanh thu năm', 'Thuế khoán theo quý', 'Thuế TNCN', 'VAT Breakdown', 'Dual-Mode'],
      prev: 'bao-cao', next: 'hoa-don-dien-tu',
      content: `
<h2 id="s0">Doanh thu năm</h2>
<p>Hiển thị doanh thu cả năm + thanh % so với ngưỡng <strong>500 triệu</strong> (chuyển sang thuế kê khai). Cảnh báo nếu vượt ngưỡng.</p>

<h2 id="s1">Thuế khoán theo quý</h2>
<table><thead><tr><th>Cột</th><th>Nội dung</th></tr></thead><tbody>
<tr><td>Kỳ thuế</td><td>Q1, Q2, Q3, Q4</td></tr>
<tr><td>Doanh thu quý</td><td>Tự tổng hợp từ đơn hàng</td></tr>
<tr><td>Thuế GTGT</td><td>Tính tự động (mặc định 1%)</td></tr>
<tr><td>Hạn nộp</td><td>Ngày cuối quý + 30 ngày</td></tr>
<tr><td>Trạng thái</td><td>Nút "Đã nộp" để đánh dấu</td></tr>
</tbody></table>

<h2 id="s2">Thuế TNCN</h2>
<p>Tính tự động theo TT 40/2021: <strong>0.5%</strong> (phân phối) hoặc <strong>1%</strong> (dịch vụ). Hiển thị: doanh thu chịu thuế, thuế TNCN ước tính, số còn lại sau thuế.</p>

<h2 id="s3">VAT Breakdown</h2>
<p>Chọn tháng → xem chi tiết HĐ theo mức VAT (0%, 5%, 8%, 10%). Bảng: Mã HĐ, Ngày, Khách, Giá trước thuế, VAT, Tổng.</p>

<h2 id="s4">Dual-Mode (Trắng/Xám)</h2>
<ul>
<li><strong>Bán có hóa đơn:</strong> tính vào doanh thu thuế, có thể xuất HĐĐT</li>
<li><strong>Bán lẻ không báo thuế:</strong> không tính vào doanh thu thuế</li>
<li>Chọn khi checkout: tick ô "Xuất hóa đơn"</li>
</ul>
<div class="tip-box">⚙️ Vào nút ⚙️ để tùy chỉnh % GTGT, % TNCN → Lưu.</div>`
    },

  }

  return { guides }
}
