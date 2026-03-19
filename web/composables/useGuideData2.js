// Guide content data part 2 — remaining 9 guides
// Merged into useGuideData at runtime

export function useGuideDataPart2() {
  const guides = {

    'hoa-don-dien-tu': {
      title: 'Hóa đơn điện tử', icon: '📄',
      headings: ['Cài đặt HĐĐT', 'Xuất HĐĐT khi bán hàng', 'Theo dõi trạng thái', 'Nhập HĐ đầu vào', 'Lưu ý'],
      prev: 'thue-ke-toan', next: 'khuyen-mai-voucher',
      content: `
<h2 id="s0">Cài đặt HĐĐT</h2>
<ul>
<li>Vào <strong>Cài đặt → Hóa đơn Điện tử</strong></li>
<li>Chọn nhà cung cấp: <strong>VNPT S-Invoice</strong> / <strong>Viettel S-Invoice</strong> / <strong>MISA</strong></li>
<li>Nhập: API URL, Tài khoản/Username, Mật khẩu/Secret key</li>
<li>Nút <strong>"Test kết nối"</strong> → kiểm tra liên kết thành công</li>
</ul>

<h2 id="s1">Xuất HĐĐT khi bán hàng</h2>
<ul>
<li>Khi checkout: tick ô "Xuất HĐĐT"</li>
<li>App hiển thị preview: Tên KH, MST, Địa chỉ, Danh sách SP, Tổng tiền</li>
<li>Xác nhận → app gọi API NCC → tạo HĐ</li>
</ul>

<h2 id="s2">Theo dõi trạng thái</h2>
<p>Trong Lịch sử đơn hàng: mỗi đơn có badge trạng thái HĐĐT:</p>
<ul>
<li>✅ Đã xuất | ⏳ Đang xử lý | ❌ Lỗi</li>
<li>Nếu lỗi → nút "Thử lại" → gửi lại API</li>
</ul>

<h2 id="s3">Nhập HĐ đầu vào</h2>
<p>Import file XML hóa đơn mua vào để quản lý và đối soát.</p>

<h2 id="s4">Lưu ý</h2>
<ul>
<li>Phải có hợp đồng với NCC HĐĐT (VNPT/Viettel/MISA)</li>
<li>Cần internet khi xuất HĐ</li>
<li>❌ Chưa hỗ trợ: Hủy / Điều chỉnh / Thay thế HĐ (sẽ có trong bản sau)</li>
</ul>`
    },

    'khuyen-mai-voucher': {
      title: 'Khuyến mãi & Voucher', icon: '🎁',
      headings: ['Tạo chương trình KM', 'Quản lý Voucher', 'Trạng thái KM'],
      prev: 'hoa-don-dien-tu', next: 'tich-diem',
      content: `
<h2 id="s0">Tạo chương trình KM</h2>
<p>Nhập: Tên, Loại KM, Giá trị, Ngày bắt đầu/kết thúc, Trạng thái.</p>
<table><thead><tr><th>Loại KM</th><th>Ví dụ</th></tr></thead><tbody>
<tr><td>Giảm giá %</td><td>Giảm 10% cho tất cả SP</td></tr>
<tr><td>Giảm giá cố định</td><td>Giảm 50,000đ cho đơn > 500k</td></tr>
<tr><td>Mua X tặng Y</td><td>Mua 2 tặng 1</td></tr>
</tbody></table>
<p>Áp dụng cho: Tất cả SP / Một SP cụ thể / Nhóm SP.</p>

<h2 id="s1">Quản lý Voucher</h2>
<ul>
<li>Mỗi KM có thể tạo nhiều mã voucher</li>
<li>Nhập mã + giới hạn số lần sử dụng</li>
<li>Khi checkout: nhập mã voucher → app tự giảm giá</li>
</ul>

<h2 id="s2">Trạng thái KM</h2>
<p>🟢 Đang chạy | 🟡 Sắp diễn ra | 🔴 Đã kết thúc</p>`
    },

    'tich-diem': {
      title: 'Tích điểm Loyalty', icon: '⭐',
      headings: ['Cài đặt', 'Cách hoạt động', 'Hạng thẻ'],
      prev: 'khuyen-mai-voucher', next: 'chot-so',
      content: `
<h2 id="s0">Cài đặt</h2>
<ul>
<li>Vào <strong>Cài đặt → Tích điểm</strong></li>
<li>Bật/tắt tính năng tích điểm</li>
<li>Cấu hình: Bao nhiêu đồng = 1 điểm, 1 điểm = bao nhiêu đồng khi đổi</li>
</ul>

<h2 id="s1">Cách hoạt động</h2>
<ul>
<li>Mỗi đơn hàng → khách tự động tích điểm (theo tổng tiền)</li>
<li>Khi checkout → hiển thị điểm hiện tại + hạng thẻ</li>
<li>Khách chọn đổi điểm → trừ vào tiền thanh toán</li>
</ul>

<h2 id="s2">Hạng thẻ</h2>
<p>Bronze → Silver → Gold → Platinum → Diamond (tùy cấu hình ngưỡng điểm).</p>
<div class="tip-box">💡 Hạng thẻ tự động nâng/hạ dựa trên tổng điểm tích lũy.</div>`
    },

    'chot-so': {
      title: 'Chốt sổ cuối ngày', icon: '📋',
      headings: ['Mục đích', 'Quy trình', 'KPI sau khi chốt', 'Lịch sử chốt sổ'],
      prev: 'tich-diem', next: 'nhan-vien',
      content: `
<h2 id="s0">Mục đích</h2>
<p>Tổng kết hoạt động bán hàng trong ngày, đối soát tiền mặt thực tế với hệ thống.</p>

<h2 id="s1">Quy trình</h2>
<ol>
<li>Chọn ngày → app tự tổng hợp: Doanh thu, Tiền mặt, Chuyển khoản, Trả hàng</li>
<li>Nhập số tiền mặt thực tế đếm được</li>
<li>App so sánh: <strong>Tiền hệ thống vs Tiền thực tế</strong> → hiện Chênh lệch</li>
<li>Nhập ghi chú giải thích (nếu có lệch)</li>
<li>Nhấn <strong>"Chốt sổ"</strong> → lưu lại, <strong>không sửa được</strong></li>
</ol>

<h2 id="s2">KPI sau khi chốt</h2>
<p>4 thẻ: Doanh thu, Tiền mặt, Chuyển khoản, Trả hàng. Bảng đối soát: Tiền hệ thống / Tiền thực tế / Chênh lệch.</p>

<h2 id="s3">Lịch sử chốt sổ</h2>
<p>Xem các ngày đã chốt, filter theo tháng.</p>`
    },

    'nhan-vien': {
      title: 'Nhân viên & Phân quyền', icon: '👨‍💼',
      headings: ['Thêm nhân viên', 'Phân quyền', 'Chuyển ca / Khóa màn hình'],
      prev: 'chot-so', next: 'ai-chatbot',
      content: `
<h2 id="s0">Thêm nhân viên</h2>
<p>Vào <strong>Cài đặt → Quản lý Nhân viên</strong> (chỉ Chủ cửa hàng). Nhập tên + tạo PIN 4 số.</p>

<h2 id="s1">Phân quyền</h2>
<p>Chọn quyền truy cập cho mỗi nhân viên:</p>
<table><thead><tr><th>Quyền</th><th>Mô tả</th></tr></thead><tbody>
<tr><td>Xem Bán hàng</td><td>Mở tab POS</td></tr>
<tr><td>Xem Khách hàng</td><td>Danh sách KH + công nợ</td></tr>
<tr><td>Xem Kho hàng</td><td>Danh sách SP, tồn kho</td></tr>
<tr><td>Xem Báo cáo</td><td>Báo cáo doanh thu</td></tr>
<tr><td>Xem Thu Chi</td><td>Sổ quỹ</td></tr>
<tr><td>Xóa hóa đơn</td><td>Xóa đơn đã bán</td></tr>
<tr><td>Sửa giá SP</td><td>Thay đổi giá sản phẩm</td></tr>
<tr><td>Vào Cài đặt</td><td>Mở trang cài đặt</td></tr>
<tr><td>Quản lý KM</td><td>Tạo/sửa khuyến mãi</td></tr>
</tbody></table>

<h2 id="s2">Chuyển ca / Khóa màn hình</h2>
<ul>
<li>Khi NV rời quầy → khóa màn hình</li>
<li>NV khác nhập PIN → đăng nhập vào ca của mình</li>
</ul>`
    },

    'ai-chatbot': {
      title: 'AI Chatbot thông minh', icon: '🤖',
      headings: ['Mở chatbot', 'Các lệnh có thể hỏi', 'Feedback AI', '100% Offline'],
      prev: 'nhan-vien', next: 'cai-dat-may-in',
      content: `
<h2 id="s0">Mở chatbot</h2>
<p>Click biểu tượng 💬 ở sidebar (PC) hoặc header (Mobile).</p>

<h2 id="s1">Các lệnh có thể hỏi</h2>
<table><thead><tr><th>Loại lệnh</th><th>Ví dụ</th></tr></thead><tbody>
<tr><td>🛒 Bán hàng</td><td>"Bán 2 chai Đạm Phú Mỹ", "Thêm 5 bao Ure 46%"</td></tr>
<tr><td>💰 Tra cứu giá</td><td>"Giá Thuốc trừ sâu Regent", "Bao nhiêu tiền 1 bao NPK?"</td></tr>
<tr><td>🔍 Tìm SP</td><td>"Thuốc trị rầy nâu", "Phân bón cho lúa"</td></tr>
<tr><td>🌿 Chẩn đoán bệnh</td><td>"Lúa bị vàng lá", "Chuối bị đốm đen"</td></tr>
<tr><td>💊 Gợi ý thuốc</td><td>"Thuốc trị sâu cuốn lá", "Hoạt chất nào trị nấm?"</td></tr>
<tr><td>📊 Tra doanh thu</td><td>"Doanh thu hôm nay", "Tháng này bán bao nhiêu?"</td></tr>
<tr><td>📦 Tra tồn kho</td><td>"Còn bao nhiêu Đạm?", "Kiểm tra kho Thuốc"</td></tr>
<tr><td>📝 Tra công nợ</td><td>"Nợ của chú Hai", "Khách nào nợ nhiều nhất?"</td></tr>
<tr><td>🧭 Điều hướng</td><td>"Mở kho hàng", "Đi đến báo cáo"</td></tr>
<tr><td>📤 Xuất Excel</td><td>"Xuất Excel kho hàng", "Export đơn hàng"</td></tr>
</tbody></table>

<h2 id="s2">Feedback AI</h2>
<p>Mỗi câu trả lời có nút 👍 / 👎 — giúp AI tự cải thiện qua thời gian.</p>

<h2 id="s3">100% Offline</h2>
<p>Chatbot hoạt động hoàn toàn <strong>không cần internet</strong>. Xử lý ngôn ngữ tự nhiên tiếng Việt (NLP) ngay trên máy.</p>`
    },

    'cai-dat-may-in': {
      title: 'Cài đặt & Máy in', icon: '⚙️',
      headings: ['Thông tin cửa hàng', 'Máy in & Mẫu in', 'Dữ liệu & Sao lưu', 'Giới thiệu'],
      prev: 'ai-chatbot', next: 'mobile-app',
      content: `
<h2 id="s0">Thông tin cửa hàng</h2>
<ul>
<li>Tên, Địa chỉ, SĐT, MST, Logo</li>
<li>QR thanh toán: STK + Ngân hàng + Tên chủ TK</li>
</ul>

<h2 id="s1">Máy in & Mẫu in</h2>
<ul>
<li>Chọn máy in mặc định (máy in nhiệt 80mm)</li>
<li>In thử hóa đơn để kiểm tra</li>
<li>Tùy chỉnh mẫu in (logo, thông tin, footer)</li>
</ul>

<h2 id="s2">Dữ liệu & Sao lưu</h2>
<p>(Chỉ Chủ cửa hàng)</p>
<ul>
<li>Cloud backup tự động (khi có mạng)</li>
<li>Xuất/nhập file SQLite (backup thủ công)</li>
<li>Khôi phục khi đổi máy</li>
</ul>

<h2 id="s3">Giới thiệu</h2>
<p>Phiên bản app, kiểm tra cập nhật, thông tin liên hệ hỗ trợ.</p>`
    },

    'mobile-app': {
      title: 'Mobile App', icon: '📱',
      headings: ['Bottom Navigation', 'Hub "Thêm"', 'Thông báo thông minh', 'Đặc điểm'],
      prev: 'cai-dat-may-in', next: 'cloud-dong-bo',
      content: `
<h2 id="s0">Bottom Navigation</h2>
<p>5 tabs chính:</p>
<ul>
<li>🏠 <strong>Trang chủ:</strong> Dashboard doanh thu, đơn hàng, SP bán chạy</li>
<li>📦 <strong>Kho:</strong> Danh sách SP, tìm kiếm, lọc</li>
<li>🛒 <strong>Bán hàng:</strong> Nút FAB nổi — POS grid, tìm SP, thêm giỏ</li>
<li>➕ <strong>Thêm:</strong> Hub truy cập nhanh tới trang khác</li>
<li>⚙️ <strong>Cài đặt:</strong> Thông tin, bảo mật</li>
</ul>

<h2 id="s1">Hub "Thêm"</h2>
<p>Gồm: Báo cáo, Sổ quỹ, Nhà cung cấp, Đơn hàng, Chốt ca, Khuyến mãi, Thông báo.</p>

<h2 id="s2">Thông báo thông minh</h2>
<ul>
<li>🟡 Cảnh báo SP sắp hết hạn</li>
<li>🟠 Cảnh báo tồn kho thấp</li>
<li>📦 Hàng chậm bán > 30 ngày</li>
</ul>

<h2 id="s3">Đặc điểm</h2>
<ul>
<li>Touch-optimized: nút lớn, gesture navigation</li>
<li>Dark mode hỗ trợ</li>
<li>Scroll mượt, animation chuyên nghiệp</li>
<li>Giỏ hàng dạng sheet kéo lên từ dưới</li>
</ul>`
    },

    'cloud-dong-bo': {
      title: 'Cloud & Đồng bộ dữ liệu', icon: '☁️',
      headings: ['Offline-first', 'Multi-Device Sync', 'Cloud Backup'],
      prev: 'mobile-app', next: 'vietqr',
      content: `
<h2 id="s0">Offline-first</h2>
<ul>
<li>Dữ liệu lưu trên máy (SQLite), mọi thao tác offline đều hoạt động</li>
<li>Khi có mạng → tự đồng bộ lên cloud</li>
</ul>

<h2 id="s1">Multi-Device Sync</h2>
<ul>
<li>1 license = 1 cửa hàng = tối đa <strong>10 thiết bị</strong></li>
<li>Mở app trên PC → dữ liệu đồng bộ sang Mobile và ngược lại</li>
<li><strong>22+ bảng dữ liệu</strong> đồng bộ: SP, KH, NCC, Đơn hàng, Thu chi, Thuế, v.v.</li>
</ul>

<h2 id="s2">Cloud Backup</h2>
<ul>
<li>Backup tự động lên server (khi có mạng)</li>
<li>Khôi phục khi cài lại máy hoặc đổi thiết bị</li>
<li>Dữ liệu được mã hóa trước khi gửi</li>
</ul>`
    },

    'vietqr': {
      title: 'VietQR & Thanh toán', icon: '💸',
      headings: ['Cài đặt VietQR', 'Sử dụng khi checkout', 'Lưu ý'],
      prev: 'cloud-dong-bo', next: 'faq',
      content: `
<h2 id="s0">Cài đặt VietQR</h2>
<ul>
<li>Vào <strong>Cài đặt → Thông tin Cửa hàng</strong></li>
<li>Nhập: Số tài khoản + Ngân hàng + Tên chủ tài khoản</li>
<li>Lưu → sẵn sàng sử dụng</li>
</ul>

<h2 id="s1">Sử dụng khi checkout</h2>
<ul>
<li>Chọn phương thức "Chuyển khoản" → QR VietQR xuất hiện tự động</li>
<li>Nếu có 2 màn hình: QR hiển thị trên màn khách</li>
<li>Sinh mã giao dịch tự động để dễ đối soát</li>
</ul>

<h2 id="s2">Lưu ý</h2>
<ul>
<li>VietQR <strong>hoàn toàn miễn phí</strong></li>
<li>Hỗ trợ hầu hết ngân hàng Việt Nam</li>
<li>Khách quét từ app ngân hàng bất kỳ</li>
</ul>`
    },

    'faq': {
      title: 'Câu hỏi thường gặp (FAQ)', icon: '❓',
      headings: ['Internet & Offline', 'Gói cước', 'Thiết bị', 'Dữ liệu', 'Tính năng'],
      prev: 'vietqr', next: null,
      content: `
<h2 id="s0">Internet & Offline</h2>
<div class="faq-item"><strong>App có cần internet không?</strong>
<p>Không. App hoạt động 100% offline. Internet chỉ cần khi: đồng bộ cloud, xuất HĐĐT, kiểm tra license.</p></div>
<div class="faq-item"><strong>Chatbot AI hoạt động offline không?</strong>
<p>Có, 100% offline. Xử lý NLP trên máy, không gửi data đi đâu.</p></div>

<h2 id="s1">Gói cước</h2>
<div class="faq-item"><strong>Free khác gì Pro?</strong>
<p>Tất cả tính năng giống nhau. Free giới hạn 20 đơn/ngày, Pro không giới hạn.</p></div>
<div class="faq-item"><strong>QR thanh toán có miễn phí không?</strong>
<p>Có, VietQR miễn phí hoàn toàn. Cài STK ngân hàng trong Settings.</p></div>
<div class="faq-item"><strong>Hóa đơn điện tử có miễn phí không?</strong>
<p>App Nodi POS miễn phí chức năng xuất HĐĐT, nhưng bạn cần có hợp đồng với NCC (VNPT/Viettel/MISA) và trả phí cho họ.</p></div>

<h2 id="s2">Thiết bị</h2>
<div class="faq-item"><strong>Có dùng được trên điện thoại không?</strong>
<p>Có. App hỗ trợ cả PC (Windows) và Mobile (Android). Dữ liệu đồng bộ giữa 2 thiết bị.</p></div>
<div class="faq-item"><strong>Một license dùng được mấy máy?</strong>
<p>Tối đa 10 thiết bị (PC + Mobile).</p></div>
<div class="faq-item"><strong>In hóa đơn bằng máy in gì?</strong>
<p>Máy in nhiệt 80mm (USB). Vào Cài đặt → Máy in để chọn.</p></div>
<div class="faq-item"><strong>Bán hàng bằng barcode như thế nào?</strong>
<p>Cắm máy quét USB → quét → SP tự thêm vào giỏ. Hoặc dùng điện thoại quét qua RemoteScanner.</p></div>

<h2 id="s3">Dữ liệu</h2>
<div class="faq-item"><strong>Mất dữ liệu khi cài lại máy?</strong>
<p>Không. Dữ liệu backup trên cloud, cài lại → đăng nhập → tự khôi phục.</p></div>
<div class="faq-item"><strong>Có import sản phẩm từ Excel được không?</strong>
<p>Có. Kho hàng → nút Import → chọn file .xlsx.</p></div>
<div class="faq-item"><strong>5,700 sản phẩm nông nghiệp là sao?</strong>
<p>Đó là CSDL tham khảo. Khi thêm SP, app gợi ý từ đó. Kho hàng thật sự chỉ chứa SP bạn thêm vào.</p></div>

<h2 id="s4">Tính năng</h2>
<div class="faq-item"><strong>Thuế TNCN tính thế nào?</strong>
<p>Tự động theo TT 40/2021: 0.5% doanh thu (hộ bán lẻ). Xem chi tiết tại mục Thuế.</p></div>
<div class="faq-item"><strong>Quên PIN nhân viên?</strong>
<p>Chủ cửa hàng vào Cài đặt → Nhân viên → sửa PIN.</p></div>
<div class="faq-item"><strong>Dark mode có không?</strong>
<p>Có, tự động theo hệ thống hoặc chuyển thủ công.</p></div>`
    },

  }

  return { guides }
}
