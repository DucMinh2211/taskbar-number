# Taskbar Number

Ứng dụng nhỏ gọn giúp hiển thị số thứ tự trên các biểu tượng ở Taskbar Windows, giúp bạn dễ dàng sử dụng phím tắt `Win + [Số]` để mở nhanh ứng dụng.

## Tính năng
- **Tự động đánh số:** Hiển thị số (1-0) tương ứng với vị trí các ứng dụng trên Taskbar.
- **Tự động cập nhật:** Theo dõi và cập nhật vị trí số khi bạn di chuyển icon hoặc mở/đóng ứng dụng (mỗi 500ms).
- **Tạm dừng/Tiếp tục (Pause/Resume):** Tạm thời ẩn các con số khi xem video toàn màn hình (YouTube, Netflix) hoặc khi không cần thiết thông qua khay hệ thống để không gây xao nhãng.
- **Chế độ chạy ẩn:** Ẩn hoàn toàn cửa sổ Console, chỉ hiển thị số trên Taskbar.
- **Khay hệ thống (System Tray):** Biểu tượng ở góc màn hình giúp quản lý, tạm dừng và thoát ứng dụng dễ dàng.
- **Chống ghi đè:** Tự động đẩy cửa sổ số lên trên cùng khi bạn tương tác với Taskbar.
- **Tối ưu hóa Windows 11:** Hỗ trợ các cấu trúc Taskbar mới nhất và lọc bỏ các nút hệ thống (Start, Search, Widgets).

## Đánh giá hiệu suất
Ứng dụng được tối ưu hóa sâu để chạy liên tục mà không ảnh hưởng đến trải nghiệm người dùng:
- **Tài nguyên cực thấp:** 
    - CPU: < 0.5% (chỉ quét tọa độ 2 lần mỗi giây).
    - RAM: 2MB - 8MB (viết bằng Rust thuần, không runtime nặng nề).
- **Độ nhạy:** Tần suất 500ms đảm bảo các con số "đuổi kịp" biểu tượng khi có thay đổi mà không gây lag dịch vụ hệ thống.
- **Công nghệ tối ưu:** 
    - Sử dụng **UI Automation API** chính chủ từ Microsoft để lấy tọa độ icon an toàn.
    - Sử dụng **Layered Windows + GDI** để vẽ overlay trong suốt với hiệu năng cao nhất.
    - Xử lý trạng thái Pause thông qua `SW_HIDE` giúp ứng dụng hoàn toàn không can thiệp vào màn hình khi không cần thiết.

## Cách hoạt động
1. Khởi tạo COM và UI Automation.
2. Quét vùng chứa danh sách ứng dụng (`MSTaskListWClass` hoặc `TaskbarFrameAutomationPeer`).
3. Lọc bỏ các thành phần hệ thống không thuộc danh sách phím tắt `Win + [Số]`.
4. Vẽ đè (Overlay) các con số lên đúng vị trí trung tâm của biểu tượng ứng dụng.
5. Duy trì trạng thái "luôn trên cùng" (TopMost) để đảm bảo tính hiển thị.
6. Khi ở trạng thái **Pause**, ứng dụng ẩn cửa sổ overlay và tạm dừng việc quét UI Automation để tiết kiệm tài nguyên tuyệt đối.

## Yêu cầu hệ thống
- Windows 10/11.
- Rust toolchain (nếu muốn biên dịch từ nguồn).

## Cách chạy
```bash
cargo run
```
**Điều khiển qua Khay hệ thống (System Tray):**
- **Tạm dừng:** Chuột phải vào biểu tượng và chọn **Pause** (số sẽ biến mất).
- **Tiếp tục:** Chuột phải vào biểu tượng và chọn **Resume** (số sẽ hiện lại).
- **Thoát:** Chuột phải vào biểu tượng và chọn **Exit**.
