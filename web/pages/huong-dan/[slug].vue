<template>
  <div v-if="guide">
    <section class="guide-hero">
      <div class="container">
        <nav class="breadcrumb"><NuxtLink to="/huong-dan">← Hướng dẫn</NuxtLink></nav>
        <h1>{{ guide.title }}</h1>
      </div>
    </section>
    <section class="guide-content">
      <div class="container guide-layout">
        <aside class="toc-sidebar"><div class="toc-sticky">
          <h4>📋 Mục lục</h4>
          <ul><li v-for="(h,i) in guide.headings" :key="i"><a :href="'#s'+i">{{ h }}</a></li></ul>
          <div class="toc-nav">
            <NuxtLink v-if="guide.prev" :to="'/huong-dan/'+guide.prev" class="toc-prev">← Bài trước</NuxtLink>
            <NuxtLink v-if="guide.next" :to="'/huong-dan/'+guide.next" class="toc-next">Bài tiếp →</NuxtLink>
          </div>
        </div></aside>
        <article class="article" v-html="guide.content"></article>
      </div>
    </section>
  </div>
  <div v-else class="not-found container"><h2>Hướng dẫn không tồn tại</h2><NuxtLink to="/huong-dan">← Quay lại</NuxtLink></div>
</template>

<script setup>
definePageMeta({ layout: 'default' })
const route = useRoute()
const slug = route.params.slug

const guides = {
'cai-dat': {
  title: 'Cài đặt Nodi POS', prev: null, next: 'ban-hang',
  headings: ['Yêu cầu hệ thống', 'Tải phần mềm', 'Cài đặt', 'Nhập License Key', 'Cấu hình cửa hàng', 'Bắt đầu sử dụng'],
  content: `
<h2 id="s0">Yêu cầu hệ thống</h2>
<p>Trước khi cài đặt Nodi POS, hãy đảm bảo máy tính của bạn đáp ứng các yêu cầu tối thiểu:</p>
<ul>
<li><strong>Hệ điều hành:</strong> Windows 10 trở lên (64-bit)</li>
<li><strong>RAM:</strong> Tối thiểu 4GB (khuyến nghị 8GB)</li>
<li><strong>Ổ cứng:</strong> 500MB trống cho cài đặt + dữ liệu</li>
<li><strong>Màn hình:</strong> Độ phân giải tối thiểu 1366x768</li>
<li><strong>Kết nối mạng:</strong> Cần cho lần kích hoạt đầu tiên và đồng bộ dữ liệu</li>
</ul>
<p>💡 <em>Nodi POS hoạt động offline sau khi kích hoạt. Bạn không cần internet liên tục.</em></p>

<h2 id="s1">Tải phần mềm</h2>
<p>Truy cập trang tải phần mềm tại <a href="/tai-ung-dung">nodi.vn/tai-ung-dung</a>.</p>
<p>📥 Nhấn nút <strong>"Tải Nodi POS"</strong> để download file cài đặt (.exe). File có dung lượng khoảng 7MB.</p>
<p>⏳ Thời gian tải tùy thuộc tốc độ mạng, thường khoảng 2-5 phút.</p>

<h2 id="s2">Cài đặt</h2>
<p><strong>Bước 1:</strong> Mở file <code>NodiPOS-Setup.exe</code> vừa tải về.</p>
<p><strong>Bước 2:</strong> Nếu Windows hiện cảnh báo bảo mật, nhấn <strong>"Run anyway"</strong> (Vẫn chạy).</p>
<p><strong>Bước 3:</strong> Chọn thư mục cài đặt (mặc định: <code>C:\\Program Files\\NodiPOS</code>) → Nhấn <strong>"Install"</strong>.</p>
<p><strong>Bước 4:</strong> Chờ quá trình cài đặt hoàn tất (khoảng 1-2 phút) → Nhấn <strong>"Finish"</strong>.</p>
<p>🖥️ <em>Biểu tượng Nodi POS sẽ xuất hiện trên màn hình Desktop.</em></p>

<h2 id="s3">Nhập License Key</h2>
<p>Khi mở Nodi POS lần đầu, bạn sẽ thấy màn hình nhập License Key.</p>
<p><strong>Bước 1:</strong> Nhập License Key bạn đã mua (định dạng: <code>NODI-XXXX-XXXX-XXXX</code>).</p>
<p><strong>Bước 2:</strong> Nhấn <strong>"Kích hoạt"</strong>. Phần mềm sẽ xác thực license qua internet.</p>
<p><strong>Bước 3:</strong> Khi thấy thông báo <strong>"Kích hoạt thành công!"</strong>, bạn đã sẵn sàng sử dụng.</p>
<p>⚠️ <em>Mỗi License Key chỉ kích hoạt được trên 1 máy tính. Nếu đổi máy, liên hệ hotline để reset.</em></p>

<h2 id="s4">Cấu hình cửa hàng</h2>
<p>Sau khi kích hoạt, Nodi POS sẽ yêu cầu bạn thiết lập thông tin cửa hàng:</p>
<p><strong>Tên cửa hàng:</strong> Ví dụ "Đại lý VTNN Minh Tâm".</p>
<p><strong>Địa chỉ:</strong> Địa chỉ cửa hàng (hiển thị trên hóa đơn).</p>
<p><strong>Số điện thoại:</strong> SĐT liên hệ.</p>
<p><strong>Mật khẩu:</strong> Đặt mật khẩu đăng nhập để bảo vệ dữ liệu.</p>
<p>📝 <em>Bạn có thể thay đổi thông tin này sau trong phần Cài đặt.</em></p>

<h2 id="s5">Bắt đầu sử dụng</h2>
<p>Sau khi cấu hình xong, bạn sẽ vào màn hình chính của Nodi POS. Từ đây, bạn có thể:</p>
<ul>
<li>Thêm sản phẩm vào danh mục</li>
<li>Thêm khách hàng, nhà cung cấp</li>
<li>Bắt đầu bán hàng</li>
</ul>
<p>Xem tiếp: <a href="/huong-dan/ban-hang">Hướng dẫn bán hàng →</a></p>`
},

'ban-hang': {
  title: 'Hướng dẫn bán hàng', prev: 'cai-dat', next: 'nhap-hang',
  headings: ['Tìm sản phẩm', 'Thêm vào giỏ hàng', 'Thanh toán', 'Bán nợ', 'In hóa đơn', 'Mẹo bán hàng nhanh'],
  content: `
<h2 id="s0">Tìm sản phẩm</h2>
<p>Từ màn hình bán hàng, bạn có nhiều cách tìm sản phẩm:</p>
<p><strong>Cách 1 — Tìm kiếm:</strong> Gõ tên sản phẩm vào ô tìm kiếm. Chỉ cần gõ một phần tên, ví dụ "bassa" sẽ tìm ra "Bassa 50EC".</p>
<p><strong>Cách 2 — Quét mã vạch:</strong> Dùng máy quét mã vạch quét trên bao bì sản phẩm. Nhanh nhất, không sai sót.</p>
<p><strong>Cách 3 — Danh mục:</strong> Chọn danh mục (Phân bón, Thuốc trừ sâu, Thuốc trừ cỏ...) để lọc sản phẩm.</p>

<h2 id="s1">Thêm vào giỏ hàng</h2>
<p><strong>Bước 1:</strong> Nhấn vào sản phẩm tìm được → Sản phẩm tự động thêm vào giỏ hàng với số lượng 1.</p>
<p><strong>Bước 2:</strong> Sửa số lượng nếu cần → Nhấn vào ô số lượng, nhập số mới.</p>
<p><strong>Bước 3:</strong> Sửa giá bán (nếu muốn giảm giá) → Nhấn vào ô giá, nhập giá mới.</p>
<p>💡 <em>Nhấn nút ❌ bên cạnh sản phẩm để xóa khỏi giỏ hàng.</em></p>

<h2 id="s2">Thanh toán</h2>
<p>Sau khi thêm đủ sản phẩm, nhấn nút <strong>"Thanh toán"</strong>:</p>
<p><strong>Bước 1:</strong> Chọn khách hàng (nếu là khách quen). Nếu khách mới, nhấn "Thêm KH mới".</p>
<p><strong>Bước 2:</strong> Kiểm tra tổng tiền.</p>
<p><strong>Bước 3:</strong> Chọn hình thức thanh toán: Tiền mặt, Chuyển khoản, hoặc kết hợp.</p>
<p><strong>Bước 4:</strong> Nhập số tiền khách đưa → Phần mềm tự tính tiền thừa.</p>
<p><strong>Bước 5:</strong> Nhấn <strong>"Hoàn tất"</strong> → Đơn hàng được lưu.</p>

<h2 id="s3">Bán nợ</h2>
<p>Muốn bán nợ cho khách (không thu tiền ngay):</p>
<p><strong>Bước 1:</strong> Chọn khách hàng (bắt buộc khi bán nợ).</p>
<p><strong>Bước 2:</strong> Tại bước thanh toán, nhấn <strong>"Ghi nợ"</strong> thay vì "Hoàn tất".</p>
<p><strong>Bước 3:</strong> Hoặc thanh toán một phần: nhập số tiền thu được, phần còn lại tự động ghi nợ.</p>
<p>📌 <em>Số nợ sẽ tự động cộng vào tổng nợ của khách hàng.</em></p>

<h2 id="s4">In hóa đơn</h2>
<p>Sau khi hoàn tất đơn hàng, bạn có thể in hóa đơn:</p>
<p>Nhấn nút <strong>"In hóa đơn"</strong> → Chọn máy in (hóa đơn nhiệt 80mm hoặc A4) → Nhấn in.</p>
<p>Hóa đơn bao gồm: tên cửa hàng, danh sách sản phẩm, tổng tiền, số nợ (nếu có), ngày giờ.</p>

<h2 id="s5">Mẹo bán hàng nhanh</h2>
<ul>
<li><strong>Phím tắt:</strong> Enter để tìm, F2 để thanh toán nhanh</li>
<li><strong>Khách quen:</strong> Gõ SĐT hoặc tên khách để chọn nhanh</li>
<li><strong>Sản phẩm hay bán:</strong> Đánh dấu yêu thích để truy cập nhanh</li>
<li><strong>Mã vạch:</strong> Đầu tư máy quét mã vạch (~500.000đ) để bán hàng nhanh gấp 3 lần</li>
</ul>`
},

'nhap-hang': {
  title: 'Hướng dẫn nhập hàng', prev: 'ban-hang', next: 'cong-no',
  headings: ['Tạo phiếu nhập', 'Chọn nhà cung cấp', 'Nhập sản phẩm', 'Xác nhận phiếu nhập', 'Xem lịch sử nhập hàng'],
  content: `
<h2 id="s0">Tạo phiếu nhập</h2>
<p>Từ menu chính, chọn <strong>"Nhập hàng"</strong> → Nhấn <strong>"Tạo phiếu nhập mới"</strong>.</p>
<p>Phiếu nhập sẽ được tạo với mã tự động (ví dụ: PN-20260215-001).</p>

<h2 id="s1">Chọn nhà cung cấp</h2>
<p>Nhấn <strong>"Chọn NCC"</strong> → Tìm nhà cung cấp trong danh sách hoặc thêm mới.</p>
<p>Thông tin NCC: Tên, SĐT, địa chỉ, MST (nếu có). Đây là thông tin quan trọng cho quản lý công nợ nhà cung cấp.</p>

<h2 id="s2">Nhập sản phẩm</h2>
<p><strong>Bước 1:</strong> Tìm sản phẩm cần nhập (hoặc thêm sản phẩm mới nếu lần đầu nhập).</p>
<p><strong>Bước 2:</strong> Nhập <strong>số lượng</strong> nhập kho.</p>
<p><strong>Bước 3:</strong> Nhập <strong>giá nhập</strong> (giá vốn). Phần mềm tự tính tổng tiền phiếu nhập.</p>
<p><strong>Bước 4:</strong> Nhập <strong>hạn sử dụng</strong> (quan trọng với thuốc BVTV).</p>
<p>Lặp lại cho tất cả sản phẩm cần nhập.</p>

<h2 id="s3">Xác nhận phiếu nhập</h2>
<p>Kiểm tra lại danh sách → Nhấn <strong>"Xác nhận nhập kho"</strong>.</p>
<p>Hệ thống sẽ tự động cộng số lượng vào tồn kho hiện tại. Nếu mua nợ NCC, chọn <strong>"Ghi nợ NCC"</strong> để theo dõi.</p>

<h2 id="s4">Xem lịch sử nhập hàng</h2>
<p>Vào <strong>Nhập hàng → Lịch sử</strong> để xem tất cả phiếu nhập. Lọc theo ngày, NCC, hoặc sản phẩm. Có thể sửa hoặc hủy phiếu nhập (nếu chưa quá 24h).</p>`
},

'cong-no': {
  title: 'Quản lý công nợ', prev: 'nhap-hang', next: 'backup',
  headings: ['Xem nợ khách hàng', 'Thu tiền nợ', 'Xem nợ nhà cung cấp', 'Trả nợ NCC', 'Báo cáo công nợ'],
  content: `
<h2 id="s0">Xem nợ khách hàng</h2>
<p>Vào menu <strong>"Công nợ"</strong> → Tab <strong>"Khách hàng"</strong>.</p>
<p>Bạn sẽ thấy danh sách tất cả khách hàng đang nợ, bao gồm: tên, SĐT, tổng nợ, ngày mua nợ gần nhất. Sắp xếp theo tổng nợ giảm dần hoặc theo thời gian.</p>
<p>Nhấn vào tên khách hàng để xem <strong>chi tiết từng đơn hàng nợ</strong> và lịch sử thanh toán.</p>

<h2 id="s1">Thu tiền nợ</h2>
<p><strong>Bước 1:</strong> Trong danh sách nợ, chọn khách hàng cần thu.</p>
<p><strong>Bước 2:</strong> Nhấn <strong>"Thu tiền"</strong>.</p>
<p><strong>Bước 3:</strong> Nhập số tiền thu được. Có thể thu một phần (ví dụ: nợ 5 triệu, thu 2 triệu).</p>
<p><strong>Bước 4:</strong> Chọn hình thức: tiền mặt hoặc chuyển khoản.</p>
<p><strong>Bước 5:</strong> Nhấn <strong>"Xác nhận"</strong> → Số nợ tự động giảm.</p>
<p>📌 <em>Mỗi lần thu tiền đều được ghi nhận trong lịch sử, có thể tra cứu bất kỳ lúc nào.</em></p>

<h2 id="s2">Xem nợ nhà cung cấp</h2>
<p>Tab <strong>"Nhà cung cấp"</strong> hiển thị danh sách NCC mà bạn đang nợ. Thông tin tương tự: tên NCC, tổng nợ, phiếu nhập gần nhất.</p>

<h2 id="s3">Trả nợ NCC</h2>
<p>Chọn NCC → Nhấn <strong>"Trả nợ"</strong> → Nhập số tiền → Xác nhận. Quy trình tương tự thu nợ khách hàng.</p>

<h2 id="s4">Báo cáo công nợ</h2>
<p>Vào <strong>Báo cáo → Công nợ</strong> để xem tổng quan:</p>
<ul>
<li>Tổng nợ khách hàng phải thu</li>
<li>Tổng nợ NCC phải trả</li>
<li>Top 10 khách nợ nhiều nhất</li>
<li>Nợ quá hạn (nếu có hạn thanh toán)</li>
</ul>`
},

'backup': {
  title: 'Sao lưu & khôi phục', prev: 'cong-no', next: 'hoa-don-dien-tu',
  headings: ['Tại sao cần sao lưu?', 'Backup tự động', 'Backup thủ công', 'Khôi phục khi máy hỏng', 'Lưu ý quan trọng'],
  content: `
<h2 id="s0">Tại sao cần sao lưu?</h2>
<p>Dữ liệu kinh doanh là tài sản quý giá nhất. Chỉ cần một sự cố — virus, hỏng ổ cứng, mất máy — tất cả dữ liệu thong tin khách hàng, công nợ, tồn kho có thể bị mất sạch.</p>
<p>Nodi POS cung cấp 2 hình thức sao lưu: <strong>tự động lên cloud</strong> và <strong>thủ công vào USB/ổ cứng ngoài</strong>.</p>

<h2 id="s1">Backup tự động</h2>
<p>Nodi POS <strong>tự động sao lưu dữ liệu lên server</strong> mỗi ngày (nếu có internet).</p>
<p>Quy trình hoàn toàn tự động, bạn không cần làm gì cả. Dữ liệu được mã hóa trước khi gửi, đảm bảo an toàn.</p>
<p>Kiểm tra trạng thái backup: vào <strong>Cài đặt → Sao lưu</strong> → Xem ngày backup gần nhất.</p>

<h2 id="s2">Backup thủ công</h2>
<p>Ngoài backup tự động, bạn nên backup thủ công vào USB/ổ cứng ngoài định kỳ:</p>
<p><strong>Bước 1:</strong> Cắm USB vào máy tính.</p>
<p><strong>Bước 2:</strong> Vào <strong>Cài đặt → Sao lưu → "Sao lưu ngay"</strong>.</p>
<p><strong>Bước 3:</strong> Chọn ổ USB làm nơi lưu → Nhấn <strong>"Bắt đầu"</strong>.</p>
<p><strong>Bước 4:</strong> Chờ hoàn tất → Rút USB và cất giữ ở nơi an toàn.</p>
<p>💡 <em>Nên backup thủ công ít nhất 1 lần/tuần.</em></p>

<h2 id="s3">Khôi phục khi máy hỏng</h2>
<p>Nếu máy tính hỏng và bạn cần cài lại trên máy mới:</p>
<p><strong>Cách 1 — Từ cloud:</strong> Cài Nodi POS → Đăng nhập → Nhấn <strong>"Khôi phục từ backup"</strong> → Chọn bản backup gần nhất → Chờ tải về.</p>
<p><strong>Cách 2 — Từ USB:</strong> Cài Nodi POS → <strong>Cài đặt → Khôi phục</strong> → Chọn file backup trên USB → Nhập.</p>
<p>⚠️ <em>Liên hệ hotline để reset license key khi đổi máy.</em></p>

<h2 id="s4">Lưu ý quan trọng</h2>
<ul>
<li>Backup tự động cần internet — nếu mất mạng nhiều ngày, backup thủ công bổ sung</li>
<li>File backup có dung lượng nhỏ (vài MB), không tốn nhiều dung lượng USB</li>
<li>Không xóa file backup cũ — giữ ít nhất 3 bản gần nhất</li>
<li>Khôi phục sẽ <strong>ghi đè</strong> toàn bộ dữ liệu hiện tại — chắc chắn trước khi thực hiện</li>
</ul>`
},

'hoa-don-dien-tu': {
  title: 'Hóa đơn điện tử', prev: 'backup', next: null,
  headings: ['Giới thiệu HĐĐT', 'Cấu hình VNPT', 'Xuất HĐĐT từ đơn hàng', 'Tra cứu hóa đơn', 'Xử lý hóa đơn sai'],
  content: `
<h2 id="s0">Giới thiệu HĐĐT</h2>
<p>Hóa đơn điện tử (HĐĐT) là hóa đơn được tạo, gửi và lưu trữ bằng phương tiện điện tử. Theo quy định, tất cả doanh nghiệp và hộ kinh doanh phải sử dụng HĐĐT.</p>
<p>Nodi POS tích hợp sẵn tính năng xuất HĐĐT thông qua nhà cung cấp <strong>VNPT</strong> hoặc <strong>Viettel</strong>.</p>

<h2 id="s1">Cấu hình VNPT</h2>
<p><strong>Bước 1:</strong> Đăng ký tài khoản HĐĐT với VNPT (hoặc Viettel). Bạn sẽ nhận được: tài khoản, mật khẩu, và token xác thực.</p>
<p><strong>Bước 2:</strong> Trong Nodi POS, vào <strong>Cài đặt → Hóa đơn điện tử</strong>.</p>
<p><strong>Bước 3:</strong> Chọn nhà cung cấp (VNPT/Viettel).</p>
<p><strong>Bước 4:</strong> Nhập thông tin: MST, tài khoản API, mật khẩu/token.</p>
<p><strong>Bước 5:</strong> Nhấn <strong>"Kiểm tra kết nối"</strong> → Nếu thành công, nhấn <strong>"Lưu"</strong>.</p>

<h2 id="s2">Xuất HĐĐT từ đơn hàng</h2>
<p>Sau khi hoàn tất đơn hàng bán:</p>
<p><strong>Bước 1:</strong> Nhấn nút <strong>"Xuất HĐĐT"</strong> (biểu tượng 🧾).</p>
<p><strong>Bước 2:</strong> Nhập thông tin người mua: MST (nếu có), tên đơn vị/cá nhân, địa chỉ.</p>
<p><strong>Bước 3:</strong> Kiểm tra danh sách sản phẩm, đơn giá, thuế suất.</p>
<p><strong>Bước 4:</strong> Nhấn <strong>"Phát hành"</strong> → HĐĐT được tạo và gửi qua email khách hàng.</p>
<p>📌 <em>Mỗi đơn hàng chỉ xuất được 1 HĐĐT. Nếu cần sửa, phải hủy và xuất lại.</em></p>

<h2 id="s3">Tra cứu hóa đơn</h2>
<p>Vào <strong>Hóa đơn → Danh sách HĐĐT</strong> để xem tất cả hóa đơn đã xuất.</p>
<p>Lọc theo: ngày, khách hàng, trạng thái (đã gửi, đã hủy, điều chỉnh). Nhấn vào hóa đơn để xem chi tiết hoặc tải PDF.</p>

<h2 id="s4">Xử lý hóa đơn sai</h2>
<p>Nếu phát hiện sai sót sau khi phát hành:</p>
<p><strong>Sai thông tin người mua:</strong> Lập hóa đơn thay thế. Trong quản lý HĐĐT → Chọn hóa đơn sai → <strong>"Thay thế"</strong> → Nhập thông tin đúng → Phát hành.</p>
<p><strong>Sai số lượng/giá:</strong> Lập hóa đơn điều chỉnh. Chọn hóa đơn → <strong>"Điều chỉnh"</strong> → Ghi rõ nội dung điều chỉnh.</p>
<p><strong>Hủy hóa đơn:</strong> Chọn hóa đơn → <strong>"Hủy"</strong> → Ghi lý do → Xác nhận. Cần thông báo cho người mua.</p>`
},
}

const guide = guides[slug] || null
if (guide) {
  useHead({
    title: guide.title + ' — Hướng dẫn Nodi POS',
    meta: [{ name: 'description', content: guide.title + ' — Hướng dẫn chi tiết sử dụng Nodi POS' }]
  })
}
</script>

<style scoped>
.guide-hero { background: linear-gradient(135deg, #065F46, #10B981); color: white; padding: 80px 0 50px; }
.breadcrumb { margin-bottom: 16px; } .breadcrumb a { color: #A7F3D0; font-size: 0.9rem; }
.guide-hero h1 { font-size: 2rem; font-weight: 800; }
.container { max-width: 1000px; margin: 0 auto; padding: 0 24px; }
.guide-content { padding: 48px 0; background: #F8FAFC; }
.guide-layout { display: grid; grid-template-columns: 200px 1fr; gap: 40px; }
.toc-sticky { position: sticky; top: 80px; }
.toc-sticky h4 { font-size: 0.9rem; font-weight: 700; color: #64748B; margin-bottom: 12px; }
.toc-sticky ul { list-style: none; padding: 0; } .toc-sticky li { margin-bottom: 8px; }
.toc-sticky a { font-size: 0.85rem; color: #64748B; } .toc-sticky a:hover { color: #10B981; }
.toc-nav { margin-top: 24px; display: flex; flex-direction: column; gap: 8px; }
.toc-prev, .toc-next { font-size: 0.85rem; color: #10B981; font-weight: 600; }
.article { background: white; border-radius: 16px; padding: 40px; box-shadow: 0 1px 3px rgb(0 0 0 / 0.06); }
.article :deep(h2) { font-size: 1.3rem; font-weight: 800; color: #1E293B; margin: 28px 0 14px; }
.article :deep(h2:first-child) { margin-top: 0; }
.article :deep(p) { line-height: 1.8; color: #334155; margin-bottom: 14px; }
.article :deep(ul) { margin: 14px 0; padding-left: 24px; } .article :deep(li) { margin-bottom: 6px; line-height: 1.7; color: #334155; }
.article :deep(code) { background: #F1F5F9; padding: 2px 6px; border-radius: 4px; font-size: 0.9em; }
.article :deep(em) { color: #64748B; }
.not-found { padding: 80px 0; text-align: center; }
@media (max-width: 768px) { .guide-layout { grid-template-columns: 1fr; } .toc-sidebar { display: none; } .article { padding: 24px; } .guide-hero h1 { font-size: 1.4rem; } }
</style>
