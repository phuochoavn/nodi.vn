<template>
  <div>
    <div class="page-header">
      <div class="container">
        <h1>Chính sách bảo mật</h1>
        <p>Cập nhật lần cuối: 15/02/2026</p>
      </div>
    </div>

    <section class="section">
      <div class="container">
        <div class="policy-content">
          <h2>1. Thông tin thu thập</h2>
          <p>Nodi POS thu thập các thông tin cần thiết để cung cấp dịch vụ:</p>
          <ul>
            <li>Thông tin cửa hàng: tên, địa chỉ, số điện thoại</li>
            <li>Dữ liệu bán hàng: hóa đơn, sản phẩm, khách hàng, nhà cung cấp</li>
            <li>Thông tin thiết bị: mã HWID (để xác thực license)</li>
          </ul>

          <h2>2. Mục đích sử dụng</h2>
          <p>Chúng tôi sử dụng thông tin thu thập để:</p>
          <ul>
            <li>Cung cấp dịch vụ phần mềm và hỗ trợ kỹ thuật</li>
            <li>Sao lưu (backup) dữ liệu lên cloud để bảo vệ dữ liệu</li>
            <li>Đồng bộ dữ liệu giữa thiết bị và server</li>
            <li>Cải thiện chất lượng sản phẩm</li>
          </ul>

          <h2>3. Bảo mật dữ liệu</h2>
          <p>Nodi POS cam kết bảo vệ dữ liệu của bạn:</p>
          <ul>
            <li>Mã hóa SSL/TLS cho mọi kết nối</li>
            <li>Mật khẩu được hash bằng bcrypt</li>
            <li>Dữ liệu lưu trữ tại server riêng, không dùng shared hosting</li>
            <li>Xác thực JWT cho mọi API request</li>
          </ul>

          <h2>4. Chia sẻ thông tin</h2>
          <p>Chúng tôi <strong>KHÔNG</strong> chia sẻ, bán, hoặc cho thuê thông tin cá nhân của bạn cho bất kỳ bên thứ 3 nào, trừ khi:</p>
          <ul>
            <li>Được sự đồng ý rõ ràng của bạn</li>
            <li>Theo yêu cầu của cơ quan pháp luật có thẩm quyền</li>
          </ul>

          <h2>5. Quyền của người dùng</h2>
          <p>Bạn có quyền:</p>
          <ul>
            <li>Yêu cầu xem, sửa, xóa dữ liệu cá nhân</li>
            <li>Yêu cầu xuất dữ liệu của bạn</li>
            <li>Hủy tài khoản và xóa toàn bộ dữ liệu</li>
          </ul>

          <h2>6. Liên hệ</h2>
          <p>Nếu có câu hỏi về chính sách bảo mật, vui lòng liên hệ:</p>
          <ul>
            <li>Email: <a href="mailto:hoavn12345@gmail.com">hoavn12345@gmail.com</a></li>
            <li>Zalo: <a href="https://zalo.me/0374222326">0374.222.326</a></li>
          </ul>
        </div>
      </div>
    </section>
  </div>
</template>

<script setup>
useHead({
  title: 'Chính sách bảo mật — Nodi POS',
  meta: [{ name: 'description', content: 'Chính sách bảo mật và quyền riêng tư của Nodi POS. Chúng tôi cam kết bảo vệ dữ liệu cửa hàng của bạn.' }],
})
</script>

<style scoped>
.policy-content {
  max-width: 700px;
  margin: 0 auto;
}
.policy-content h2 {
  font-size: 1.3rem;
  font-weight: 700;
  margin-top: 40px;
  margin-bottom: 12px;
  color: var(--dark-700);
}
.policy-content h2:first-child { margin-top: 0; }
.policy-content p {
  margin-bottom: 12px;
  color: var(--text);
  line-height: 1.8;
}
.policy-content ul {
  margin-bottom: 16px;
  padding-left: 24px;
}
.policy-content li {
  padding: 6px 0;
  line-height: 1.7;
  color: var(--text);
}
.policy-content a { color: var(--primary); }
.policy-content a:hover { text-decoration: underline; }
</style>
